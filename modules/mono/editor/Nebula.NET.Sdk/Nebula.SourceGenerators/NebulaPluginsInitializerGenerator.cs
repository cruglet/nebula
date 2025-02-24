using System.Text;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.Text;

namespace Nebula.SourceGenerators
{
    [Generator]
    public class NebulaPluginsInitializerGenerator : ISourceGenerator
    {
        public void Initialize(GeneratorInitializationContext context)
        {
        }

        public void Execute(GeneratorExecutionContext context)
        {
            if (context.IsNebulaToolsProject() || context.IsNebulaSourceGeneratorDisabled("NebulaPluginsInitializer"))
                return;

            string source =
                @"using System;
using System.Runtime.InteropServices;
using Nebula.Bridge;
using Nebula.NativeInterop;

namespace NebulaPlugins.Game
{
    internal static partial class Main
    {
        [UnmanagedCallersOnly(EntryPoint = ""nebulasharp_game_main_init"")]
        private static nebula_bool InitializeFromGameProject(IntPtr nebulaDllHandle, IntPtr outManagedCallbacks,
            IntPtr unmanagedCallbacks, int unmanagedCallbacksSize)
        {
            try
            {
                DllImportResolver dllImportResolver = new NebulaDllImportResolver(nebulaDllHandle).OnResolveDllImport;

                var coreApiAssembly = typeof(global::Nebula.NebulaObject).Assembly;

                NativeLibrary.SetDllImportResolver(coreApiAssembly, dllImportResolver);

                NativeFuncs.Initialize(unmanagedCallbacks, unmanagedCallbacksSize);

                ManagedCallbacks.Create(outManagedCallbacks);

                ScriptManagerBridge.LookupScriptsInAssembly(typeof(global::NebulaPlugins.Game.Main).Assembly);

                return nebula_bool.True;
            }
            catch (Exception e)
            {
                global::System.Console.Error.WriteLine(e);
                return false.ToNebulaBool();
            }
        }
    }
}
";

            context.AddSource("NebulaPlugins.Game.generated",
                SourceText.From(source, Encoding.UTF8));
        }
    }
}
