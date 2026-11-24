using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using BepInEx;
using BepInEx.Logging;
using BepInEx.Unity.IL2CPP;
using FM.GamePlugin;
using FM.UI;
using HarmonyLib;
using SI.Bindable;
using SI.Bindable.Reference.Core;
using SI.Core;
using SI.Interop;

namespace FM27Trainer.Bridge;

[BepInPlugin("com.fm27trainer.bridge", "FM27 Trainer Bridge", "0.6.0")]
public sealed class Plugin : BasePlugin
{
    internal static ManualLogSource LogSource = null!;
    internal static readonly string OutDir = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
        "FM27 Trainer"
    );
    internal static readonly string DumpPath = Path.Combine(OutDir, "bepinex-tactical-dump.json");
    internal static readonly string CmdPath = Path.Combine(OutDir, "bepinex-intensity-cmd.json");
    internal static readonly string CmdResultPath = Path.Combine(OutDir, "bepinex-intensity-result.json");

    private Harmony? _harmony;

    public override void Load()
    {
        LogSource = Log;
        Directory.CreateDirectory(OutDir);
        _harmony = new Harmony("com.fm27trainer.bridge");
        _harmony.PatchAll(typeof(InteropChannelPatches));
        Log.LogInfo("FM27 Trainer Bridge 0.6.0 loaded. Watching TacticalEffectiveness + Players + Legitimate Intensity Events.");
    }
}

internal enum ChannelKind
{
    TacticalEffectiveness,
    PlayerIntensity,
}

internal sealed record ChannelInfo(ChannelKind Kind, string RefName, string PropertyName, uint RuntimeId = 0);

internal sealed class IntensityCommandPayload
{
    public uint manager_runtime_id { get; set; }
    public List<IntensityCommandItem>? items { get; set; }
}

internal sealed class IntensityCommandItem
{
    public uint runtime_id { get; set; }
    public int level { get; set; }
}

internal static class InteropChannelPatches
{
    private const uint PairRefType = 825307168u;
    private const uint PairSession = 1198877028u;
    private const uint PairManager = 1348620377u;
    private const uint PairPerson = 1348825715u;
    private const uint PairContextFlag = 1416196963u;
    private const int PersonTag = 0x01000000;

    private const uint EventPlayerIntensityChanged = 1735948081u; // InteropEventIDs.EventFmxPlayerTrainingIntensityLevelChanged
    private const uint KeyOptionId = 1886680684u;
    private const uint KeyOptionName = 1903577654u;
    private const uint KeyConfirmFlagA = 1215644527u;
    private const uint KeyConfirmFlagB = 840966176u;

    private static readonly ConcurrentDictionary<int, ChannelInfo> Tracked = new();
    private static readonly ConcurrentDictionary<uint, int> CurrentIntensities = new();
    private static readonly object WriteLock = new();

    private static volatile int _sessionId;
    private static volatile int _managerRefVal;
    private static long _lastCmdPollMs;
    private static string? _lastEntryKey;
    private static object? _lastTacticalDump;

    [HarmonyPatch(typeof(InteropDataHandler), nameof(InteropDataHandler.OpenChannel))]
    [HarmonyPostfix]
    private static void OnOpenChannel(TypedValue source, Bindings.Property property, Bindings.Key key)
    {
        try
        {
            HarvestReferenceContext(source, out var playerRuntimeId);

            var propName = property?.Name ?? "";
            if (string.Equals(propName, "TacticalEffectiveness", StringComparison.OrdinalIgnoreCase))
            {
                var refName = DescribeSource(source);
                if (!refName.StartsWith("TeamTactic:", StringComparison.OrdinalIgnoreCase))
                    return;

                Tracked[key.GetHashCode()] = new ChannelInfo(ChannelKind.TacticalEffectiveness, refName, propName);
                Plugin.LogSource.LogInfo($"Tracked: {refName}.{propName}");
                return;
            }

            if (playerRuntimeId != 0 && propName.EndsWith("IndividualTrainingIntensity", StringComparison.OrdinalIgnoreCase))
            {
                Tracked[key.GetHashCode()] = new ChannelInfo(ChannelKind.PlayerIntensity, "PlayerTraining", propName, playerRuntimeId);
            }
        }
        catch (Exception ex)
        {
            Plugin.LogSource.LogDebug($"OpenChannel err: {ex.Message}");
        }
    }

    [HarmonyPatch(typeof(InteropDataHandler), nameof(InteropDataHandler.CloseChannel))]
    [HarmonyPostfix]
    private static void OnCloseChannel(Bindings.Key key)
    {
        Tracked.TryRemove(key.GetHashCode(), out _);
    }

    [HarmonyPatch(typeof(Bindings.HandlerAccess), nameof(Bindings.HandlerAccess.Set))]
    [HarmonyPostfix]
    private static void OnSet(ref Bindings.Key key, TypedValue value) => CaptureValue(key, value);

    [HarmonyPatch(typeof(Bindings.HandlerAccess), nameof(Bindings.HandlerAccess.SetPrefetched))]
    [HarmonyPostfix]
    private static void OnSetPrefetched(ref Bindings.Key key, TypedValue value) => CaptureValue(key, value);

    [HarmonyPatch(typeof(GameInteropSubsystem), nameof(GameInteropSubsystem.Tick))]
    [HarmonyPostfix]
    private static void OnSubsystemTick(GameInteropSubsystem __instance)
    {
        var now = Environment.TickCount64;
        if (now - _lastCmdPollMs < 750)
            return;
        _lastCmdPollMs = now;

        try
        {
            if (!File.Exists(Plugin.CmdPath))
                return;

            var raw = File.ReadAllText(Plugin.CmdPath);
            try { File.Delete(Plugin.CmdPath); } catch { }

            var cmd = JsonSerializer.Deserialize<IntensityCommandPayload>(raw);
            if (cmd?.items == null || cmd.items.Count == 0)
                return;

            ApplyIntensityCommands(__instance, cmd);
        }
        catch (Exception ex)
        {
            Plugin.LogSource.LogWarning($"Intensity cmd err: {ex.Message}");
            WriteCommandResult(0, new List<string> { ex.Message });
        }
    }

    private static void ApplyIntensityCommands(GameInteropSubsystem subsystem, IntensityCommandPayload cmd)
    {
        var errors = new List<string>();
        var applied = 0;

        var mgrVal = _managerRefVal != 0
            ? _managerRefVal
            : (cmd.manager_runtime_id != 0 ? (PersonTag | (int)cmd.manager_runtime_id) : 0);

        if (mgrVal == 0)
        {
            WriteCommandResult(0, new List<string> { "Menajer referansı bulunamadı (FM'de kariyer açık mı?)." });
            return;
        }

        foreach (var item in cmd.items!)
        {
            if (item.runtime_id == 0) continue;
            var level = Math.Clamp(item.level, 0, 2);
            var label = level switch
            {
                0 => "Half Intensity",
                2 => "Double Intensity",
                _ => "Normal Intensity",
            };

            try
            {
                var pairs = new Il2CppSystem.Collections.Generic.List<InteropReference.Pair>();
                pairs.Add(new InteropReference.Pair(PairRefType, 77));
                if (_sessionId != 0)
                    pairs.Add(new InteropReference.Pair(PairSession, _sessionId));
                pairs.Add(new InteropReference.Pair(PairManager, mgrVal));
                pairs.Add(new InteropReference.Pair(PairPerson, PersonTag | (int)item.runtime_id));
                pairs.Add(new InteropReference.Pair(PairContextFlag, 1));

                var targetRef = new InteropReference(pairs);

                var dataRef = new DynamicReference();
                var dict = dataRef.Cast<Il2CppSystem.Collections.Generic.Dictionary<uint, TypedValue>>();

                var dn = new DynamicNumber();
                dn.Set(level);
                dict[KeyOptionId] = TypedValue.CreateValueType(dn);
                dict[KeyOptionName] = TypedValue.Create(label);
                dict[KeyConfirmFlagA] = TypedValue.Create(true);
                dict[KeyConfirmFlagB] = TypedValue.Create(true);

                subsystem.SendEvent(EventPlayerIntensityChanged, targetRef, TypedValue.Create(dataRef));
                CurrentIntensities[item.runtime_id] = level;
                applied++;
            }
            catch (Exception ex)
            {
                errors.Add($"id={item.runtime_id}: {ex.Message}");
            }
        }

        Plugin.LogSource.LogInfo($"Applied legitimate training intensity for {applied} player(s) (errors={errors.Count}).");
        WriteCommandResult(applied, errors);
        if (_lastEntryKey != null && _lastTacticalDump != null)
            WriteLatest(_lastEntryKey, _lastTacticalDump);
    }

    private static void WriteCommandResult(int applied, List<string> errors)
    {
        try
        {
            var res = new Dictionary<string, object?>
            {
                ["updated_utc"] = System.DateTime.UtcNow.ToString("O"),
                ["applied"] = applied,
                ["errors"] = errors,
            };
            var json = JsonSerializer.Serialize(res, new JsonSerializerOptions { WriteIndented = true });
            var tmp = Plugin.CmdResultPath + ".tmp";
            File.WriteAllText(tmp, json);
            File.Move(tmp, Plugin.CmdResultPath, true);
        }
        catch { }
    }

    private static void HarvestReferenceContext(TypedValue? source, out uint playerRuntimeId)
    {
        playerRuntimeId = 0;
        if (source == null || source.IsNull) return;
        try
        {
            var ir = source.Get()?.TryCast<InteropReference>();
            var pairs = ir?.m_id;
            if (pairs == null) return;
            var count = pairs.Count;
            for (var i = 0; i < count; i++)
            {
                var p = pairs[i];
                if (p.ID == PairSession && p.Value != 0)
                    _sessionId = p.Value;
                else if (p.ID == PairManager && p.Value != 0)
                    _managerRefVal = p.Value;
                else if (p.ID == PairPerson && (p.Value & 0xFF000000) == PersonTag)
                    playerRuntimeId = (uint)(p.Value & 0x00FFFFFF);
            }
        }
        catch { }
    }

    private static void CaptureValue(Bindings.Key key, TypedValue? value)
    {
        try
        {
            if (!Tracked.TryGetValue(key.GetHashCode(), out var info))
                return;

            if (info.Kind == ChannelKind.PlayerIntensity)
            {
                if (info.RuntimeId != 0 && TryExtractInt(value, out var lvl) && lvl is >= 0 and <= 4)
                    CurrentIntensities[info.RuntimeId] = lvl;
                return;
            }

            var dumped = DumpTypedValue(value, 0);
            if (dumped == null) return;

            var entryKey = $"{info.RefName}.{info.PropertyName}";
            _lastEntryKey = entryKey;
            _lastTacticalDump = dumped;
            WriteLatest(entryKey, dumped);
        }
        catch (Exception ex)
        {
            Plugin.LogSource.LogWarning($"CaptureValue err: {ex.Message}");
        }
    }

    private static bool TryExtractInt(TypedValue? tv, out int val)
    {
        val = -1;
        if (tv == null || tv.IsNull) return false;
        try
        {
            var s = tv.AsString() ?? tv.Get()?.ToString();
            if (s != null && int.TryParse(s.Trim(), out val))
                return true;
        }
        catch { }
        try
        {
            var rec = tv.Get()?.TryCast<Record>();
            if (rec != null)
            {
                var en = rec.GetEnumerator();
                while (en.MoveNext())
                {
                    var cur = en.Current;
                    if (cur.Key == KeyOptionId && TryExtractInt(cur.Value, out val))
                        return true;
                }
            }
        }
        catch { }
        return false;
    }

    private static object? DumpTypedValue(TypedValue? tv, int depth)
    {
        if (tv == null || tv.IsNull) return null;
        if (depth > 4) return "<max-depth>";

        try
        {
            var obj = tv.Get();
            if (obj != null)
            {
                var ir = obj.TryCast<InteropReference>();
                if (ir != null)
                    return DumpInteropReference(ir);

                var rec = obj.TryCast<Record>();
                if (rec != null)
                    return DumpRecord(rec, depth + 1);
            }
        }
        catch { }

        try
        {
            var s = tv.AsString();
            if (s != null) return s;
        }
        catch { }

        try
        {
            return tv.Get()?.ToString() ?? tv.ToString();
        }
        catch
        {
            return "<unreadable>";
        }
    }

    private static Dictionary<string, object?> DumpInteropReference(InteropReference ir)
    {
        var res = new Dictionary<string, object?>
        {
            ["ref"] = ir.ID.ToString() ?? ""
        };
        try
        {
            var pairs = ir.m_id;
            if (pairs != null)
            {
                var count = pairs.Count;
                for (var i = 0; i < count; i++)
                {
                    var p = pairs[i];
                    if ((p.ID == PairPerson || (p.Value & 0xFF000000) == PersonTag) && (p.Value & 0x00FFFFFF) != 0)
                    {
                        res["runtime_id"] = p.Value & 0x00FFFFFF;
                    }
                }
            }
        }
        catch { }
        return res;
    }

    private static Dictionary<string, object?> DumpRecord(Record rec, int depth)
    {
        var dict = new Dictionary<string, object?>();
        try
        {
            var enumerator = rec.GetEnumerator();
            while (enumerator.MoveNext())
            {
                var kv = enumerator.Current;
                var propName = new PropertyID(kv.Key).ToString();
                if (string.IsNullOrEmpty(propName)) propName = $"0x{kv.Key:X}";

                if (depth == 1 && propName.StartsWith("Players:", StringComparison.OrdinalIgnoreCase))
                {
                    var players = DumpPlayers(kv.Value);
                    if (players != null)
                        dict["Players"] = players;
                    continue;
                }

                if (propName.StartsWith("Players:", StringComparison.OrdinalIgnoreCase)
                    || propName.StartsWith("KeyFactors:", StringComparison.OrdinalIgnoreCase)
                    || propName.StartsWith("ExtraFactors:", StringComparison.OrdinalIgnoreCase)
                    || propName.StartsWith("TacticalAreasKeyFactors:", StringComparison.OrdinalIgnoreCase)
                    || propName.StartsWith("TacticalComprehensionKeyFactors:", StringComparison.OrdinalIgnoreCase)
                    || propName.StartsWith("Advice", StringComparison.OrdinalIgnoreCase))
                {
                    continue;
                }

                dict[propName] = DumpTypedValue(kv.Value, depth);
            }
        }
        catch (Exception ex)
        {
            dict["_err"] = ex.Message;
        }
        return dict;
    }

    private static List<Dictionary<string, object?>>? DumpPlayers(TypedValue? tv)
    {
        if (tv == null || tv.IsNull) return null;
        try
        {
            var obj = tv.Get();
            var enumerable = obj?.TryCast<Il2CppSystem.Collections.IEnumerable>();
            if (enumerable == null) return null;

            var list = new List<Dictionary<string, object?>>();
            var en = enumerable.GetEnumerator();
            while (en.MoveNext())
            {
                var cur = en.Current;
                if (cur == null) continue;

                var itemTv = cur.TryCast<TypedValue>();
                var inner = itemTv != null ? itemTv.Get() : cur;
                var pRec = inner?.TryCast<Record>();
                if (pRec == null) continue;

                list.Add(DumpRecord(pRec, 2));
                if (list.Count >= 60) break;
            }
            return list;
        }
        catch (Exception ex)
        {
            Plugin.LogSource.LogDebug($"DumpPlayers err: {ex.Message}");
            return null;
        }
    }

    private static string DescribeSource(TypedValue? source)
    {
        if (source == null || source.IsNull) return "";
        try
        {
            var obj = source.Get();
            if (obj != null)
            {
                var ir = obj.TryCast<InteropReference>();
                if (ir != null)
                    return ir.ID.ToString() ?? "";
            }
        }
        catch { }
        return "";
    }

    private static void WriteLatest(string entryKey, object dumped)
    {
        lock (WriteLock)
        {
            try
            {
                var intensities = new Dictionary<string, int>();
                foreach (var kv in CurrentIntensities)
                    intensities[kv.Key.ToString()] = kv.Value;

                var payload = new Dictionary<string, object?>
                {
                    ["updated_utc"] = System.DateTime.UtcNow.ToString("O"),
                    ["session_ready"] = _sessionId != 0,
                    ["current_intensities"] = intensities,
                    ["data"] = new Dictionary<string, object?>
                    {
                        [entryKey] = dumped
                    }
                };
                var json = JsonSerializer.Serialize(payload, new JsonSerializerOptions { WriteIndented = true });
                var tmp = Plugin.DumpPath + ".tmp";
                File.WriteAllText(tmp, json);
                File.Move(tmp, Plugin.DumpPath, true);
            }
            catch (Exception ex)
            {
                Plugin.LogSource.LogWarning($"Flush failed: {ex.Message}");
            }
        }
    }
}
