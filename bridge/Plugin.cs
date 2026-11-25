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

[BepInPlugin("com.fm27trainer.bridge", "FM27 Training Assistant Bridge", "0.7.0")]
public sealed class Plugin : BasePlugin
{
    internal static ManualLogSource LogSource = null!;
    internal static readonly string OutDir = ResolveOutDir();

    private static string ResolveOutDir()
    {
        if (OperatingSystem.IsMacOS())
        {
            return Path.Combine(
                Environment.GetFolderPath(Environment.SpecialFolder.UserProfile),
                "Library",
                "Application Support",
                "FM27 Trainer"
            );
        }

        return Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
            "FM27 Trainer"
        );
    }
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
        Log.LogInfo("FM27 Training Assistant Bridge 0.7.0 loaded. Watching TacticalEffectiveness + Players + Intensity/Focus Events.");
    }
}

internal enum ChannelKind
{
    TacticalEffectiveness,
    PlayerIntensity,
    FocusOptions,
}

internal sealed record ChannelInfo(ChannelKind Kind, string RefName, string PropertyName, uint RuntimeId = 0);

internal sealed class IntensityCommandPayload
{
    public uint manager_runtime_id { get; set; }
    public List<IntensityCommandItem>? items { get; set; }
    public List<FocusCommandItem>? focus_items { get; set; }
}

internal sealed class IntensityCommandItem
{
    public uint runtime_id { get; set; }
    public int level { get; set; }
}

internal sealed class FocusCommandItem
{
    public uint runtime_id { get; set; }
    public int option_id { get; set; }
    public string? name { get; set; }
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
    private const uint EventPlayerFocusChanged = 1280659764u;     // InteropEventIDs.EventFmxPlayerTrainingFocusChanged
    private const uint KeyFocusFlagA = 1299866233u;
    private const uint KeyFocusFlagB = 1500668246u;
    private const uint KeyFocusInner = 1916366157u;

    private const uint KeyOptionId = 1886680684u;
    private const uint KeyOptionName = 1903577654u;
    private const uint KeyConfirmFlagA = 1215644527u;
    private const uint KeyConfirmFlagB = 840966176u;

    private static readonly ConcurrentDictionary<int, ChannelInfo> Tracked = new();
    private static readonly ConcurrentDictionary<uint, int> CurrentIntensities = new();
    private static readonly ConcurrentDictionary<int, string> FocusOptions = new();
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
                return;
            }

            if (propName.IndexOf("AdditionalFocus", StringComparison.OrdinalIgnoreCase) >= 0
                || propName.IndexOf("IndividualTrainingType", StringComparison.OrdinalIgnoreCase) >= 0)
            {
                Tracked[key.GetHashCode()] = new ChannelInfo(ChannelKind.FocusOptions, DescribeSource(source), propName, playerRuntimeId);
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
            if (cmd == null)
                return;

            var hasIntensity = cmd.items != null && cmd.items.Count > 0;
            var hasFocus = cmd.focus_items != null && cmd.focus_items.Count > 0;
            if (!hasIntensity && !hasFocus)
                return;

            ApplyCommands(__instance, cmd);
        }
        catch (Exception ex)
        {
            Plugin.LogSource.LogWarning($"Training cmd err: {ex.Message}");
            WriteCommandResult(0, new List<string> { ex.Message });
        }
    }

    private static InteropReference BuildPlayerTargetRef(int mgrVal, uint runtimeId)
    {
        var pairs = new Il2CppSystem.Collections.Generic.List<InteropReference.Pair>();
        pairs.Add(new InteropReference.Pair(PairRefType, 77));
        if (_sessionId != 0)
            pairs.Add(new InteropReference.Pair(PairSession, _sessionId));
        pairs.Add(new InteropReference.Pair(PairManager, mgrVal));
        pairs.Add(new InteropReference.Pair(PairPerson, PersonTag | (int)runtimeId));
        pairs.Add(new InteropReference.Pair(PairContextFlag, 1));
        return new InteropReference(pairs);
    }

    private static void ApplyCommands(GameInteropSubsystem subsystem, IntensityCommandPayload cmd)
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

        if (cmd.items != null)
        {
            foreach (var item in cmd.items)
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
                    var targetRef = BuildPlayerTargetRef(mgrVal, item.runtime_id);
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
                    errors.Add($"intensity id={item.runtime_id}: {ex.Message}");
                }
            }
        }

        if (cmd.focus_items != null)
        {
            foreach (var item in cmd.focus_items)
            {
                if (item.runtime_id == 0) continue;
                try
                {
                    ApplyFocusItem(subsystem, mgrVal, item);
                    applied++;
                }
                catch (Exception ex)
                {
                    errors.Add($"focus id={item.runtime_id}: {ex.Message}");
                }
            }
        }

        Plugin.LogSource.LogInfo($"Applied legitimate training command for {applied} item(s) (errors={errors.Count}).");
        WriteCommandResult(applied, errors);
        if (_lastEntryKey != null && _lastTacticalDump != null)
            WriteLatest(_lastEntryKey, _lastTacticalDump);
    }

    private static void ApplyFocusItem(GameInteropSubsystem subsystem, int mgrVal, FocusCommandItem item)
    {
        var targetRef = BuildPlayerTargetRef(mgrVal, item.runtime_id);

        var outerRef = new DynamicReference();
        var outer = outerRef.Cast<Il2CppSystem.Collections.Generic.Dictionary<uint, TypedValue>>();
        outer[KeyFocusFlagA] = TypedValue.Create(true);
        outer[KeyFocusFlagB] = TypedValue.Create(true);
        outer[KeyConfirmFlagA] = TypedValue.Create(true);

        var innerRef = new DynamicReference();
        var inner = innerRef.Cast<Il2CppSystem.Collections.Generic.Dictionary<uint, TypedValue>>();
        var dn = new DynamicNumber();
        dn.Set(item.option_id);
        inner[KeyOptionId] = TypedValue.CreateValueType(dn);
        if (!string.IsNullOrWhiteSpace(item.name))
            inner[KeyOptionName] = TypedValue.Create(item.name);

        outer[KeyFocusInner] = TypedValue.Create(innerRef);
        subsystem.SendEvent(EventPlayerFocusChanged, targetRef, TypedValue.Create(outerRef));
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

            if (info.Kind == ChannelKind.FocusOptions)
            {
                if (HarvestFocusOptions(value, 0) && _lastEntryKey != null && _lastTacticalDump != null)
                    WriteLatest(_lastEntryKey, _lastTacticalDump);
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

    private static bool HarvestFocusOptions(TypedValue? tv, int depth)
    {
        if (tv == null || tv.IsNull || depth > 3) return false;
        var added = false;
        try
        {
            var obj = tv.Get();
            if (obj == null) return false;

            var rec = obj.TryCast<Record>();
            if (rec != null)
            {
                var optId = -1;
                string? optName = null;
                var en = rec.GetEnumerator();
                while (en.MoveNext())
                {
                    var kv = en.Current;
                    if (kv.Key == KeyOptionId && TryExtractInt(kv.Value, out var id))
                        optId = id;
                    else if (kv.Key == KeyOptionName)
                        optName = kv.Value?.AsString()?.Trim();
                    else if (HarvestFocusOptions(kv.Value, depth + 1))
                        added = true;
                }
                if (optId >= 0 && optId <= 100 && !string.IsNullOrWhiteSpace(optName))
                {
                    if (!FocusOptions.TryGetValue(optId, out var prev) || prev != optName)
                    {
                        FocusOptions[optId] = optName!;
                        added = true;
                    }
                }
                return added;
            }

            var enumerable = obj.TryCast<Il2CppSystem.Collections.IEnumerable>();
            if (enumerable != null)
            {
                var en = enumerable.GetEnumerator();
                var count = 0;
                while (en.MoveNext() && count < 80)
                {
                    count++;
                    var cur = en.Current;
                    if (cur == null) continue;
                    var itemTv = cur.TryCast<TypedValue>();
                    if (itemTv != null)
                    {
                        if (HarvestFocusOptions(itemTv, depth + 1)) added = true;
                        continue;
                    }
                    var itemRec = cur.TryCast<Record>();
                    if (itemRec != null && HarvestFocusOptions(TypedValue.Create(itemRec), depth + 1))
                        added = true;
                }
            }
        }
        catch { }
        return added;
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

                var focusOpts = new Dictionary<string, string>();
                foreach (var kv in FocusOptions)
                    focusOpts[kv.Key.ToString()] = kv.Value;

                var payload = new Dictionary<string, object?>
                {
                    ["updated_utc"] = System.DateTime.UtcNow.ToString("O"),
                    ["session_ready"] = _sessionId != 0,
                    ["current_intensities"] = intensities,
                    ["focus_options"] = focusOpts,
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
