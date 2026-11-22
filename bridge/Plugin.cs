using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using BepInEx;
using BepInEx.Logging;
using BepInEx.Unity.IL2CPP;
using FM.UI;
using HarmonyLib;
using SI.Bindable;
using SI.Bindable.Reference.Core;
using SI.Core;
using SI.Interop;

namespace FM27Trainer.Bridge;

[BepInPlugin("com.fm27trainer.bridge", "FM27 Trainer Bridge", "0.5.0")]
public sealed class Plugin : BasePlugin
{
    internal static ManualLogSource LogSource = null!;
    internal static readonly string OutDir = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
        "FM27 Trainer"
    );
    internal static readonly string DumpPath = Path.Combine(OutDir, "bepinex-tactical-dump.json");

    private Harmony? _harmony;

    public override void Load()
    {
        LogSource = Log;
        Directory.CreateDirectory(OutDir);
        _harmony = new Harmony("com.fm27trainer.bridge");
        _harmony.PatchAll(typeof(InteropChannelPatches));
        Log.LogInfo("FM27 Trainer Bridge 0.5.0 loaded. Watching TeamTactic.TacticalEffectiveness.");
    }
}

internal sealed record ChannelInfo(string RefName, string PropertyName);

internal static class InteropChannelPatches
{
    private static readonly ConcurrentDictionary<int, ChannelInfo> Tracked = new();
    private static readonly object WriteLock = new();

    [HarmonyPatch(typeof(InteropDataHandler), nameof(InteropDataHandler.OpenChannel))]
    [HarmonyPostfix]
    private static void OnOpenChannel(TypedValue source, Bindings.Property property, Bindings.Key key)
    {
        try
        {
            var propName = property?.Name ?? "";
            if (!string.Equals(propName, "TacticalEffectiveness", StringComparison.OrdinalIgnoreCase))
                return;

            var refName = DescribeSource(source);
            if (!refName.StartsWith("TeamTactic:", StringComparison.OrdinalIgnoreCase))
                return;

            var k = key.GetHashCode();
            Tracked[k] = new ChannelInfo(refName, propName);
            Plugin.LogSource.LogInfo($"Tracked: {refName}.{propName}");
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

    private static void CaptureValue(Bindings.Key key, TypedValue? value)
    {
        try
        {
            if (!Tracked.TryGetValue(key.GetHashCode(), out var info))
                return;

            var dumped = DumpTypedValue(value, 0);
            if (dumped == null) return;

            var entryKey = $"{info.RefName}.{info.PropertyName}";
            WriteLatest(entryKey, dumped);
        }
        catch (Exception ex)
        {
            Plugin.LogSource.LogWarning($"CaptureValue err: {ex.Message}");
        }
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

                // Yalnızca skor ve ana bileşenleri tut; dev Players / KeyFactors / Advice listelerini atla (~3 KB JSON)
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
                var payload = new Dictionary<string, object?>
                {
                    ["updated_utc"] = System.DateTime.UtcNow.ToString("O"),
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
