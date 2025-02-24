#pragma warning disable IDE1006 // Naming rule violation
// ReSharper disable InconsistentNaming

using System;
using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using Nebula;
using Nebula.NativeInterop;
using Nebula.SourceGenerators.Internal;
using NebulaTools.IdeMessaging.Requests;

namespace NebulaTools.Internals
{
    [GenerateUnmanagedCallbacks(typeof(InternalUnmanagedCallbacks))]
    internal static partial class Internal
    {
        public const string CSharpLanguageType = "CSharpScript";
        public const string CSharpLanguageExtension = ".cs";

        public static string FullExportTemplatesDir
        {
            get
            {
                nebula_icall_Internal_FullExportTemplatesDir(out nebula_string dest);
                using (dest)
                    return Marshaling.ConvertStringToManaged(dest);
            }
        }

        public static string SimplifyNebulaPath(this string path) => Nebula.StringExtensions.SimplifyPath(path);

        public static bool IsMacOSAppBundleInstalled(string bundleId)
        {
            using nebula_string bundleIdIn = Marshaling.ConvertStringToNative(bundleId);
            return nebula_icall_Internal_IsMacOSAppBundleInstalled(bundleIdIn);
        }

        public static bool NebulaIs32Bits() => nebula_icall_Internal_NebulaIs32Bits();

        public static bool NebulaIsRealTDouble() => nebula_icall_Internal_NebulaIsRealTDouble();

        public static void NebulaMainIteration() => nebula_icall_Internal_NebulaMainIteration();

        public static bool IsAssembliesReloadingNeeded() => nebula_icall_Internal_IsAssembliesReloadingNeeded();

        public static void ReloadAssemblies(bool softReload) => nebula_icall_Internal_ReloadAssemblies(softReload);

        public static void EditorDebuggerNodeReloadScripts() => nebula_icall_Internal_EditorDebuggerNodeReloadScripts();

        public static bool ScriptEditorEdit(Resource resource, int line, int col, bool grabFocus = true) =>
            nebula_icall_Internal_ScriptEditorEdit(resource.NativeInstance, line, col, grabFocus);

        public static void EditorNodeShowScriptScreen() => nebula_icall_Internal_EditorNodeShowScriptScreen();

        public static void EditorRunPlay() => nebula_icall_Internal_EditorRunPlay();

        public static void EditorRunStop() => nebula_icall_Internal_EditorRunStop();

        public static void EditorPlugin_AddControlToEditorRunBar(Control control) =>
            nebula_icall_Internal_EditorPlugin_AddControlToEditorRunBar(control.NativeInstance);

        public static void ScriptEditorDebugger_ReloadScripts() =>
            nebula_icall_Internal_ScriptEditorDebugger_ReloadScripts();

        public static string[] CodeCompletionRequest(CodeCompletionRequest.CompletionKind kind,
            string scriptFile)
        {
            using nebula_string scriptFileIn = Marshaling.ConvertStringToNative(scriptFile);
            nebula_icall_Internal_CodeCompletionRequest((int)kind, scriptFileIn, out nebula_packed_string_array res);
            using (res)
                return Marshaling.ConvertNativePackedStringArrayToSystemArray(res);
        }

        #region Internal

        private static bool initialized = false;

        // ReSharper disable once ParameterOnlyUsedForPreconditionCheck.Global
        internal static unsafe void Initialize(IntPtr unmanagedCallbacks, int unmanagedCallbacksSize)
        {
            if (initialized)
                throw new InvalidOperationException("Already initialized.");
            initialized = true;

            if (unmanagedCallbacksSize != sizeof(InternalUnmanagedCallbacks))
                throw new ArgumentException("Unmanaged callbacks size mismatch.", nameof(unmanagedCallbacksSize));

            _unmanagedCallbacks = Unsafe.AsRef<InternalUnmanagedCallbacks>((void*)unmanagedCallbacks);
        }

        private partial struct InternalUnmanagedCallbacks
        {
        }

        /*
         * IMPORTANT:
         * The order of the methods defined in NativeFuncs must match the order
         * in the array defined at the bottom of 'editor/editor_internal_calls.cpp'.
         */

        public static partial void nebula_icall_NebulaSharpDirs_ResMetadataDir(out nebula_string r_dest);

        public static partial void nebula_icall_NebulaSharpDirs_MonoUserDir(out nebula_string r_dest);

        public static partial void nebula_icall_NebulaSharpDirs_BuildLogsDirs(out nebula_string r_dest);

        public static partial void nebula_icall_NebulaSharpDirs_DataEditorToolsDir(out nebula_string r_dest);

        public static partial void nebula_icall_NebulaSharpDirs_CSharpProjectName(out nebula_string r_dest);

        public static partial void nebula_icall_EditorProgress_Create(in nebula_string task, in nebula_string label,
            int amount, bool canCancel);

        public static partial void nebula_icall_EditorProgress_Dispose(in nebula_string task);

        public static partial bool nebula_icall_EditorProgress_Step(in nebula_string task, in nebula_string state,
            int step,
            bool forceRefresh);

        private static partial void nebula_icall_Internal_FullExportTemplatesDir(out nebula_string dest);

        private static partial bool nebula_icall_Internal_IsMacOSAppBundleInstalled(in nebula_string bundleId);

        private static partial bool nebula_icall_Internal_NebulaIs32Bits();

        private static partial bool nebula_icall_Internal_NebulaIsRealTDouble();

        private static partial void nebula_icall_Internal_NebulaMainIteration();

        private static partial bool nebula_icall_Internal_IsAssembliesReloadingNeeded();

        private static partial void nebula_icall_Internal_ReloadAssemblies(bool softReload);

        private static partial void nebula_icall_Internal_EditorDebuggerNodeReloadScripts();

        private static partial bool nebula_icall_Internal_ScriptEditorEdit(IntPtr resource, int line, int col,
            bool grabFocus);

        private static partial void nebula_icall_Internal_EditorNodeShowScriptScreen();

        private static partial void nebula_icall_Internal_EditorRunPlay();

        private static partial void nebula_icall_Internal_EditorRunStop();

        private static partial void nebula_icall_Internal_EditorPlugin_AddControlToEditorRunBar(IntPtr p_control);

        private static partial void nebula_icall_Internal_ScriptEditorDebugger_ReloadScripts();

        private static partial void nebula_icall_Internal_CodeCompletionRequest(int kind, in nebula_string scriptFile,
            out nebula_packed_string_array res);

        public static partial float nebula_icall_Globals_EditorScale();

        public static partial void nebula_icall_Globals_GlobalDef(in nebula_string setting, in nebula_variant defaultValue,
            bool restartIfChanged, out nebula_variant result);

        public static partial void nebula_icall_Globals_EditorDef(in nebula_string setting, in nebula_variant defaultValue,
            bool restartIfChanged, out nebula_variant result);

        public static partial void
            nebula_icall_Globals_EditorDefShortcut(in nebula_string setting, in nebula_string name, Key keycode, nebula_bool physical, out nebula_variant result);

        public static partial void
            nebula_icall_Globals_EditorGetShortcut(in nebula_string setting, out nebula_variant result);

        public static partial void
            nebula_icall_Globals_EditorShortcutOverride(in nebula_string setting, in nebula_string feature, Key keycode, nebula_bool physical);

        public static partial void nebula_icall_Globals_TTR(in nebula_string text, out nebula_string dest);

        public static partial void nebula_icall_Utils_OS_GetPlatformName(out nebula_string dest);

        public static partial bool nebula_icall_Utils_OS_UnixFileHasExecutableAccess(in nebula_string filePath);

        #endregion
    }
}
