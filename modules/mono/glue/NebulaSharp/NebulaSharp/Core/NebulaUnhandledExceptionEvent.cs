using System;
using System.Runtime.InteropServices;
using Nebula.NativeInterop;

namespace Nebula
{
    public static partial class GD
    {
        [UnmanagedCallersOnly]
        internal static void OnCoreApiAssemblyLoaded(nebula_bool isDebug)
        {
            try
            {
                Dispatcher.InitializeDefaultNebulaTaskScheduler();

                if (isDebug.ToBool())
                {
                    DebuggingUtils.InstallTraceListener();

                    AppDomain.CurrentDomain.UnhandledException += (_, e) =>
                    {
                        // Exception.ToString() includes the inner exception
                        ExceptionUtils.LogUnhandledException((Exception)e.ExceptionObject);
                    };
                }
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
            }
        }
    }
}
