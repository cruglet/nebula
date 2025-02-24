using System;
using System.Runtime.InteropServices;
using Nebula.NativeInterop;

namespace Nebula
{
    public static class Dispatcher
    {
        internal static NebulaTaskScheduler DefaultNebulaTaskScheduler;

        internal static void InitializeDefaultNebulaTaskScheduler()
        {
            DefaultNebulaTaskScheduler?.Dispose();
            DefaultNebulaTaskScheduler = new NebulaTaskScheduler();
        }

        public static NebulaSynchronizationContext SynchronizationContext => DefaultNebulaTaskScheduler.Context;
    }
}
