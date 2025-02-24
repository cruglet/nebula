using System;
using System.Runtime.InteropServices;
using Nebula.NativeInterop;

namespace Nebula
{
    public class SignalAwaiter : IAwaiter<Variant[]>, IAwaitable<Variant[]>
    {
        private bool _completed;
        private Variant[] _result;
        private Action _continuation;

        public SignalAwaiter(NebulaObject source, StringName signal, NebulaObject target)
        {
            var awaiterGcHandle = CustomGCHandle.AllocStrong(this);
            using nebula_string_name signalSrc = NativeFuncs.nebulasharp_string_name_new_copy(
                (nebula_string_name)(signal?.NativeValue ?? default));
            NativeFuncs.nebulasharp_internal_signal_awaiter_connect(NebulaObject.GetPtr(source), in signalSrc,
                NebulaObject.GetPtr(target), GCHandle.ToIntPtr(awaiterGcHandle));
        }

        public bool IsCompleted => _completed;

        public void OnCompleted(Action continuation)
        {
            _continuation = continuation;
        }

        public Variant[] GetResult() => _result;

        public IAwaiter<Variant[]> GetAwaiter() => this;

        [UnmanagedCallersOnly]
        internal static unsafe void SignalCallback(IntPtr awaiterGCHandlePtr, nebula_variant** args, int argCount,
            nebula_bool* outAwaiterIsNull)
        {
            try
            {
                var awaiter = (SignalAwaiter)GCHandle.FromIntPtr(awaiterGCHandlePtr).Target;

                if (awaiter == null)
                {
                    *outAwaiterIsNull = nebula_bool.True;
                    return;
                }

                *outAwaiterIsNull = nebula_bool.False;

                awaiter._completed = true;

                Variant[] signalArgs = new Variant[argCount];

                for (int i = 0; i < argCount; i++)
                    signalArgs[i] = Variant.CreateCopyingBorrowed(*args[i]);

                awaiter._result = signalArgs;

                awaiter._continuation?.Invoke();
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
                *outAwaiterIsNull = nebula_bool.False;
            }
        }
    }
}
