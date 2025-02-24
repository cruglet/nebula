using System;
using System.Runtime.InteropServices;
using Nebula.NativeInterop;

namespace Nebula.Bridge
{
    internal static class CSharpInstanceBridge
    {
        [UnmanagedCallersOnly]
        internal static unsafe nebula_bool Call(IntPtr nebulaObjectGCHandle, nebula_string_name* method,
            nebula_variant** args, int argCount, nebula_variant_call_error* refCallError, nebula_variant* ret)
        {
            try
            {
                var nebulaObject = (NebulaObject)GCHandle.FromIntPtr(nebulaObjectGCHandle).Target;

                if (nebulaObject == null)
                {
                    *ret = default;
                    (*refCallError).Error = nebula_variant_call_error_error.NEBULA_CALL_ERROR_CALL_ERROR_INSTANCE_IS_NULL;
                    return nebula_bool.False;
                }

                bool methodInvoked = nebulaObject.InvokeNebulaClassMethod(CustomUnsafe.AsRef(method),
                    new NativeVariantPtrArgs(args, argCount), out nebula_variant retValue);

                if (!methodInvoked)
                {
                    *ret = default;
                    // This is important, as it tells Object::call that no method was called.
                    // Otherwise, it would prevent Object::call from calling native methods.
                    (*refCallError).Error = nebula_variant_call_error_error.NEBULA_CALL_ERROR_CALL_ERROR_INVALID_METHOD;
                    return nebula_bool.False;
                }

                *ret = retValue;
                return nebula_bool.True;
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
                *ret = default;
                return nebula_bool.False;
            }
        }

        [UnmanagedCallersOnly]
        internal static unsafe nebula_bool Set(IntPtr nebulaObjectGCHandle, nebula_string_name* name, nebula_variant* value)
        {
            try
            {
                var nebulaObject = (NebulaObject)GCHandle.FromIntPtr(nebulaObjectGCHandle).Target;

                if (nebulaObject == null)
                    throw new InvalidOperationException();

                if (nebulaObject.SetNebulaClassPropertyValue(CustomUnsafe.AsRef(name), CustomUnsafe.AsRef(value)))
                {
                    return nebula_bool.True;
                }

                var nameManaged = StringName.CreateTakingOwnershipOfDisposableValue(
                    NativeFuncs.nebulasharp_string_name_new_copy(CustomUnsafe.AsRef(name)));

                Variant valueManaged = Variant.CreateCopyingBorrowed(*value);

                return nebulaObject._Set(nameManaged, valueManaged).ToNebulaBool();
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
                return nebula_bool.False;
            }
        }

        [UnmanagedCallersOnly]
        internal static unsafe nebula_bool Get(IntPtr nebulaObjectGCHandle, nebula_string_name* name,
            nebula_variant* outRet)
        {
            try
            {
                var nebulaObject = (NebulaObject)GCHandle.FromIntPtr(nebulaObjectGCHandle).Target;

                if (nebulaObject == null)
                    throw new InvalidOperationException();

                // Properties
                if (nebulaObject.GetNebulaClassPropertyValue(CustomUnsafe.AsRef(name), out nebula_variant outRetValue))
                {
                    *outRet = outRetValue;
                    return nebula_bool.True;
                }

                // Signals
                if (nebulaObject.HasNebulaClassSignal(CustomUnsafe.AsRef(name)))
                {
                    nebula_signal signal = new nebula_signal(NativeFuncs.nebulasharp_string_name_new_copy(*name), nebulaObject.GetInstanceId());
                    *outRet = VariantUtils.CreateFromSignalTakingOwnershipOfDisposableValue(signal);
                    return nebula_bool.True;
                }

                // Methods
                if (nebulaObject.HasNebulaClassMethod(CustomUnsafe.AsRef(name)))
                {
                    nebula_callable method = new nebula_callable(NativeFuncs.nebulasharp_string_name_new_copy(*name), nebulaObject.GetInstanceId());
                    *outRet = VariantUtils.CreateFromCallableTakingOwnershipOfDisposableValue(method);
                    return nebula_bool.True;
                }

                var nameManaged = StringName.CreateTakingOwnershipOfDisposableValue(
                    NativeFuncs.nebulasharp_string_name_new_copy(CustomUnsafe.AsRef(name)));

                Variant ret = nebulaObject._Get(nameManaged);

                if (ret.VariantType == Variant.Type.Nil)
                {
                    *outRet = default;
                    return nebula_bool.False;
                }

                *outRet = ret.CopyNativeVariant();
                return nebula_bool.True;
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
                *outRet = default;
                return nebula_bool.False;
            }
        }

        [UnmanagedCallersOnly]
        internal static void CallDispose(IntPtr nebulaObjectGCHandle, nebula_bool okIfNull)
        {
            try
            {
                var nebulaObject = (NebulaObject)GCHandle.FromIntPtr(nebulaObjectGCHandle).Target;

                if (okIfNull.ToBool())
                    nebulaObject?.Dispose();
                else
                    nebulaObject!.Dispose();
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
            }
        }

        [UnmanagedCallersOnly]
        internal static unsafe void CallToString(IntPtr nebulaObjectGCHandle, nebula_string* outRes, nebula_bool* outValid)
        {
            try
            {
                var self = (NebulaObject)GCHandle.FromIntPtr(nebulaObjectGCHandle).Target;

                if (self == null)
                {
                    *outRes = default;
                    *outValid = nebula_bool.False;
                    return;
                }

                var resultStr = self.ToString();

                if (resultStr == null)
                {
                    *outRes = default;
                    *outValid = nebula_bool.False;
                    return;
                }

                *outRes = Marshaling.ConvertStringToNative(resultStr);
                *outValid = nebula_bool.True;
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
                *outRes = default;
                *outValid = nebula_bool.False;
            }
        }

        [UnmanagedCallersOnly]
        internal static unsafe nebula_bool HasMethodUnknownParams(IntPtr nebulaObjectGCHandle, nebula_string_name* method)
        {
            try
            {
                var nebulaObject = (NebulaObject)GCHandle.FromIntPtr(nebulaObjectGCHandle).Target;

                if (nebulaObject == null)
                    return nebula_bool.False;

                return nebulaObject.HasNebulaClassMethod(CustomUnsafe.AsRef(method)).ToNebulaBool();
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
                return nebula_bool.False;
            }
        }

        [UnmanagedCallersOnly]
        internal static unsafe void SerializeState(
            IntPtr nebulaObjectGCHandle,
            nebula_dictionary* propertiesState,
            nebula_dictionary* signalEventsState
        )
        {
            try
            {
                var nebulaObject = (NebulaObject)GCHandle.FromIntPtr(nebulaObjectGCHandle).Target;

                if (nebulaObject == null)
                    return;

                // Call OnBeforeSerialize

                // ReSharper disable once SuspiciousTypeConversion.Global
                if (nebulaObject is ISerializationListener serializationListener)
                    serializationListener.OnBeforeSerialize();

                // Save instance state

                using var info = NebulaSerializationInfo.CreateCopyingBorrowed(
                    *propertiesState, *signalEventsState);

                nebulaObject.SaveNebulaObjectData(info);
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
            }
        }

        [UnmanagedCallersOnly]
        internal static unsafe void DeserializeState(
            IntPtr nebulaObjectGCHandle,
            nebula_dictionary* propertiesState,
            nebula_dictionary* signalEventsState
        )
        {
            try
            {
                var nebulaObject = (NebulaObject)GCHandle.FromIntPtr(nebulaObjectGCHandle).Target;

                if (nebulaObject == null)
                    return;

                // Restore instance state

                using var info = NebulaSerializationInfo.CreateCopyingBorrowed(
                    *propertiesState, *signalEventsState);

                nebulaObject.RestoreNebulaObjectData(info);

                // Call OnAfterDeserialize

                // ReSharper disable once SuspiciousTypeConversion.Global
                if (nebulaObject is ISerializationListener serializationListener)
                    serializationListener.OnAfterDeserialize();
            }
            catch (Exception e)
            {
                ExceptionUtils.LogException(e);
            }
        }
    }
}
