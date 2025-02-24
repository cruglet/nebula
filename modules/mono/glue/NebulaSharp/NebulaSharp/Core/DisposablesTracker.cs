using System;
using System.Collections.Concurrent;
using System.Runtime.InteropServices;
using Nebula.NativeInterop;

#nullable enable

namespace Nebula
{
    internal static class DisposablesTracker
    {
        [UnmanagedCallersOnly]
        internal static void OnNebulaShuttingDown()
        {
            try
            {
                OnNebulaShuttingDownImpl();
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
            }
        }

        private static void OnNebulaShuttingDownImpl()
        {
            bool isStdoutVerbose;

            try
            {
                isStdoutVerbose = OS.IsStdOutVerbose();
            }
            catch (ObjectDisposedException)
            {
                // OS singleton already disposed. Maybe OnUnloading was called twice.
                isStdoutVerbose = false;
            }

            if (isStdoutVerbose)
                GD.Print("Unloading: Disposing tracked instances...");

            // Dispose Nebula Objects first, and only then dispose other disposables
            // like StringName, NodePath, Nebula.Collections.Array/Dictionary, etc.
            // The Nebula Object Dispose() method may need any of the later instances.

            foreach (WeakReference<NebulaObject> item in NebulaObjectInstances.Keys)
            {
                if (item.TryGetTarget(out NebulaObject? self))
                    self.Dispose();
            }

            foreach (WeakReference<IDisposable> item in OtherInstances.Keys)
            {
                if (item.TryGetTarget(out IDisposable? self))
                    self.Dispose();
            }

            if (isStdoutVerbose)
                GD.Print("Unloading: Finished disposing tracked instances.");
        }

        private static ConcurrentDictionary<WeakReference<NebulaObject>, byte> NebulaObjectInstances { get; } =
            new();

        private static ConcurrentDictionary<WeakReference<IDisposable>, byte> OtherInstances { get; } =
            new();

        public static WeakReference<NebulaObject> RegisterNebulaObject(NebulaObject nebulaObject)
        {
            var weakReferenceToSelf = new WeakReference<NebulaObject>(nebulaObject);
            NebulaObjectInstances.TryAdd(weakReferenceToSelf, 0);
            return weakReferenceToSelf;
        }

        public static WeakReference<IDisposable> RegisterDisposable(IDisposable disposable)
        {
            var weakReferenceToSelf = new WeakReference<IDisposable>(disposable);
            OtherInstances.TryAdd(weakReferenceToSelf, 0);
            return weakReferenceToSelf;
        }

        public static void UnregisterNebulaObject(NebulaObject nebulaObject, WeakReference<NebulaObject> weakReferenceToSelf)
        {
            if (!NebulaObjectInstances.TryRemove(weakReferenceToSelf, out _))
                throw new ArgumentException("Nebula Object not registered.", nameof(weakReferenceToSelf));
        }

        public static void UnregisterDisposable(WeakReference<IDisposable> weakReference)
        {
            if (!OtherInstances.TryRemove(weakReference, out _))
                throw new ArgumentException("Disposable not registered.", nameof(weakReference));
        }
    }
}
