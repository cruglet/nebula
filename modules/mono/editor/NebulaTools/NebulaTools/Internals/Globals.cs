using Nebula;
using Nebula.NativeInterop;
using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;

namespace NebulaTools.Internals
{
    public static class Globals
    {
        public static float EditorScale => Internal.nebula_icall_Globals_EditorScale();

        // ReSharper disable once UnusedMethodReturnValue.Global
        public static Variant GlobalDef(string setting, Variant defaultValue, bool restartIfChanged = false)
        {
            using nebula_string settingIn = Marshaling.ConvertStringToNative(setting);
            using nebula_variant defaultValueIn = defaultValue.CopyNativeVariant();
            Internal.nebula_icall_Globals_GlobalDef(settingIn, defaultValueIn, restartIfChanged,
                out nebula_variant result);
            return Variant.CreateTakingOwnershipOfDisposableValue(result);
        }

        // ReSharper disable once UnusedMethodReturnValue.Global
        public static Variant EditorDef(string setting, Variant defaultValue, bool restartIfChanged = false)
        {
            using nebula_string settingIn = Marshaling.ConvertStringToNative(setting);
            using nebula_variant defaultValueIn = defaultValue.CopyNativeVariant();
            Internal.nebula_icall_Globals_EditorDef(settingIn, defaultValueIn, restartIfChanged,
                out nebula_variant result);
            return Variant.CreateTakingOwnershipOfDisposableValue(result);
        }

        public static Shortcut EditorDefShortcut(string setting, string name, Key keycode = Key.None, bool physical = false)
        {
            using nebula_string settingIn = Marshaling.ConvertStringToNative(setting);
            using nebula_string nameIn = Marshaling.ConvertStringToNative(name);
            Internal.nebula_icall_Globals_EditorDefShortcut(settingIn, nameIn, keycode, physical.ToNebulaBool(), out nebula_variant result);
            return (Shortcut)Variant.CreateTakingOwnershipOfDisposableValue(result);
        }

        public static Shortcut EditorGetShortcut(string setting)
        {
            using nebula_string settingIn = Marshaling.ConvertStringToNative(setting);
            Internal.nebula_icall_Globals_EditorGetShortcut(settingIn, out nebula_variant result);
            return (Shortcut)Variant.CreateTakingOwnershipOfDisposableValue(result);
        }

        public static void EditorShortcutOverride(string setting, string feature, Key keycode = Key.None, bool physical = false)
        {
            using nebula_string settingIn = Marshaling.ConvertStringToNative(setting);
            using nebula_string featureIn = Marshaling.ConvertStringToNative(feature);
            Internal.nebula_icall_Globals_EditorShortcutOverride(settingIn, featureIn, keycode, physical.ToNebulaBool());
        }

        [SuppressMessage("ReSharper", "InconsistentNaming")]
        public static string TTR(this string text)
        {
            using nebula_string textIn = Marshaling.ConvertStringToNative(text);
            Internal.nebula_icall_Globals_TTR(textIn, out nebula_string dest);
            using (dest)
                return Marshaling.ConvertStringToManaged(dest);
        }
    }
}
