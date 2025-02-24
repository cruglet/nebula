#pragma warning disable CA1707 // Identifiers should not contain underscores
#pragma warning disable IDE1006 // Naming rule violation
// ReSharper disable InconsistentNaming

using System;
using System.Runtime.CompilerServices;
using Nebula.SourceGenerators.Internal;


namespace Nebula.NativeInterop
{
    /*
     * IMPORTANT:
     * The order of the methods defined in NativeFuncs must match the order
     * in the array defined at the bottom of 'glue/runtime_interop.cpp'.
     */

    [GenerateUnmanagedCallbacks(typeof(UnmanagedCallbacks))]
    public static unsafe partial class NativeFuncs
    {
        private static bool initialized;

        // ReSharper disable once ParameterOnlyUsedForPreconditionCheck.Global
        public static void Initialize(IntPtr unmanagedCallbacks, int unmanagedCallbacksSize)
        {
            if (initialized)
                throw new InvalidOperationException("Already initialized.");
            initialized = true;

            if (unmanagedCallbacksSize != sizeof(UnmanagedCallbacks))
                throw new ArgumentException("Unmanaged callbacks size mismatch.", nameof(unmanagedCallbacksSize));

            _unmanagedCallbacks = Unsafe.AsRef<UnmanagedCallbacks>((void*)unmanagedCallbacks);
        }

        private partial struct UnmanagedCallbacks
        {
        }

        // Custom functions

        internal static partial nebula_bool nebulasharp_dotnet_module_is_initialized();

        public static partial IntPtr nebulasharp_method_bind_get_method(in nebula_string_name p_classname,
            in nebula_string_name p_methodname);

        public static partial IntPtr nebulasharp_method_bind_get_method_with_compatibility(
            in nebula_string_name p_classname, in nebula_string_name p_methodname, ulong p_hash);

        public static partial delegate* unmanaged<IntPtr> nebulasharp_get_class_constructor(
            in nebula_string_name p_classname);

        public static partial IntPtr nebulasharp_engine_get_singleton(in nebula_string p_name);


        internal static partial Error nebulasharp_stack_info_vector_resize(
            ref DebuggingUtils.nebula_stack_info_vector p_stack_info_vector, int p_size);

        internal static partial void nebulasharp_stack_info_vector_destroy(
            ref DebuggingUtils.nebula_stack_info_vector p_stack_info_vector);

        internal static partial void nebulasharp_internal_editor_file_system_update_files(in nebula_packed_string_array p_script_paths);

        internal static partial void nebulasharp_internal_script_debugger_send_error(in nebula_string p_func,
            in nebula_string p_file, int p_line, in nebula_string p_err, in nebula_string p_descr,
            nebula_error_handler_type p_type, in DebuggingUtils.nebula_stack_info_vector p_stack_info_vector);

        internal static partial nebula_bool nebulasharp_internal_script_debugger_is_active();

        internal static partial IntPtr nebulasharp_internal_object_get_associated_gchandle(IntPtr ptr);

        internal static partial void nebulasharp_internal_object_disposed(IntPtr ptr, IntPtr gcHandleToFree);

        internal static partial void nebulasharp_internal_refcounted_disposed(IntPtr ptr, IntPtr gcHandleToFree,
            nebula_bool isFinalizer);

        internal static partial Error nebulasharp_internal_signal_awaiter_connect(IntPtr source,
            in nebula_string_name signal,
            IntPtr target, IntPtr awaiterHandlePtr);

        internal static partial void nebulasharp_internal_tie_native_managed_to_unmanaged(IntPtr gcHandleIntPtr,
            IntPtr unmanaged, in nebula_string_name nativeName, nebula_bool refCounted);

        internal static partial void nebulasharp_internal_tie_user_managed_to_unmanaged(IntPtr gcHandleIntPtr,
            IntPtr unmanaged, nebula_ref* scriptPtr, nebula_bool refCounted);

        internal static partial void nebulasharp_internal_tie_managed_to_unmanaged_with_pre_setup(
            IntPtr gcHandleIntPtr, IntPtr unmanaged);

        internal static partial IntPtr nebulasharp_internal_unmanaged_get_script_instance_managed(IntPtr p_unmanaged,
            out nebula_bool r_has_cs_script_instance);

        internal static partial IntPtr nebulasharp_internal_unmanaged_get_instance_binding_managed(IntPtr p_unmanaged);

        internal static partial IntPtr nebulasharp_internal_unmanaged_instance_binding_create_managed(IntPtr p_unmanaged,
            IntPtr oldGCHandlePtr);

        internal static partial void nebulasharp_internal_new_csharp_script(nebula_ref* r_dest);

        internal static partial nebula_bool nebulasharp_internal_script_load(in nebula_string p_path, nebula_ref* r_dest);

        internal static partial void nebulasharp_internal_reload_registered_script(IntPtr scriptPtr);

        internal static partial void nebulasharp_array_filter_nebula_objects_by_native(in nebula_string_name p_native_name,
            in nebula_array p_input, out nebula_array r_output);

        internal static partial void nebulasharp_array_filter_nebula_objects_by_non_native(in nebula_array p_input,
            out nebula_array r_output);

        public static partial void nebulasharp_ref_new_from_ref_counted_ptr(out nebula_ref r_dest,
            IntPtr p_ref_counted_ptr);

        public static partial void nebulasharp_ref_destroy(ref nebula_ref p_instance);

        public static partial void nebulasharp_string_name_new_from_string(out nebula_string_name r_dest,
            in nebula_string p_name);

        public static partial void nebulasharp_node_path_new_from_string(out nebula_node_path r_dest,
            in nebula_string p_name);

        public static partial void
            nebulasharp_string_name_as_string(out nebula_string r_dest, in nebula_string_name p_name);

        public static partial void nebulasharp_node_path_as_string(out nebula_string r_dest, in nebula_node_path p_np);

        public static partial nebula_packed_byte_array nebulasharp_packed_byte_array_new_mem_copy(byte* p_src,
            int p_length);

        public static partial nebula_packed_int32_array nebulasharp_packed_int32_array_new_mem_copy(int* p_src,
            int p_length);

        public static partial nebula_packed_int64_array nebulasharp_packed_int64_array_new_mem_copy(long* p_src,
            int p_length);

        public static partial nebula_packed_float32_array nebulasharp_packed_float32_array_new_mem_copy(float* p_src,
            int p_length);

        public static partial nebula_packed_float64_array nebulasharp_packed_float64_array_new_mem_copy(double* p_src,
            int p_length);

        public static partial nebula_packed_vector2_array nebulasharp_packed_vector2_array_new_mem_copy(Vector2* p_src,
            int p_length);

        public static partial nebula_packed_vector3_array nebulasharp_packed_vector3_array_new_mem_copy(Vector3* p_src,
            int p_length);

        public static partial nebula_packed_vector4_array nebulasharp_packed_vector4_array_new_mem_copy(Vector4* p_src,
            int p_length);

        public static partial nebula_packed_color_array nebulasharp_packed_color_array_new_mem_copy(Color* p_src,
            int p_length);

        public static partial void nebulasharp_packed_string_array_add(ref nebula_packed_string_array r_dest,
            in nebula_string p_element);

        public static partial void nebulasharp_callable_new_with_delegate(IntPtr p_delegate_handle, IntPtr p_trampoline,
            IntPtr p_object, out nebula_callable r_callable);

        internal static partial nebula_bool nebulasharp_callable_get_data_for_marshalling(in nebula_callable p_callable,
            out IntPtr r_delegate_handle, out IntPtr r_trampoline, out IntPtr r_object, out nebula_string_name r_name);

        internal static partial nebula_variant nebulasharp_callable_call(in nebula_callable p_callable,
            nebula_variant** p_args, int p_arg_count, out nebula_variant_call_error p_call_error);

        internal static partial void nebulasharp_callable_call_deferred(in nebula_callable p_callable,
            nebula_variant** p_args, int p_arg_count);

        internal static partial Color nebulasharp_color_from_ok_hsl(float p_h, float p_s, float p_l, float p_alpha);

        // GDNative functions

        // gdnative.h

        public static partial void nebulasharp_method_bind_ptrcall(IntPtr p_method_bind, IntPtr p_instance, void** p_args,
            void* p_ret);

        public static partial nebula_variant nebulasharp_method_bind_call(IntPtr p_method_bind, IntPtr p_instance,
            nebula_variant** p_args, int p_arg_count, out nebula_variant_call_error p_call_error);

        // variant.h

        public static partial void
            nebulasharp_variant_new_string_name(out nebula_variant r_dest, in nebula_string_name p_s);

        public static partial void nebulasharp_variant_new_copy(out nebula_variant r_dest, in nebula_variant p_src);

        public static partial void nebulasharp_variant_new_node_path(out nebula_variant r_dest, in nebula_node_path p_np);

        public static partial void nebulasharp_variant_new_object(out nebula_variant r_dest, IntPtr p_obj);

        public static partial void nebulasharp_variant_new_transform2d(out nebula_variant r_dest, in Transform2D p_t2d);

        public static partial void nebulasharp_variant_new_basis(out nebula_variant r_dest, in Basis p_basis);

        public static partial void nebulasharp_variant_new_transform3d(out nebula_variant r_dest, in Transform3D p_trans);

        public static partial void nebulasharp_variant_new_projection(out nebula_variant r_dest, in Projection p_proj);

        public static partial void nebulasharp_variant_new_aabb(out nebula_variant r_dest, in Aabb p_aabb);

        public static partial void nebulasharp_variant_new_dictionary(out nebula_variant r_dest,
            in nebula_dictionary p_dict);

        public static partial void nebulasharp_variant_new_array(out nebula_variant r_dest, in nebula_array p_arr);

        public static partial void nebulasharp_variant_new_packed_byte_array(out nebula_variant r_dest,
            in nebula_packed_byte_array p_pba);

        public static partial void nebulasharp_variant_new_packed_int32_array(out nebula_variant r_dest,
            in nebula_packed_int32_array p_pia);

        public static partial void nebulasharp_variant_new_packed_int64_array(out nebula_variant r_dest,
            in nebula_packed_int64_array p_pia);

        public static partial void nebulasharp_variant_new_packed_float32_array(out nebula_variant r_dest,
            in nebula_packed_float32_array p_pra);

        public static partial void nebulasharp_variant_new_packed_float64_array(out nebula_variant r_dest,
            in nebula_packed_float64_array p_pra);

        public static partial void nebulasharp_variant_new_packed_string_array(out nebula_variant r_dest,
            in nebula_packed_string_array p_psa);

        public static partial void nebulasharp_variant_new_packed_vector2_array(out nebula_variant r_dest,
            in nebula_packed_vector2_array p_pv2a);

        public static partial void nebulasharp_variant_new_packed_vector3_array(out nebula_variant r_dest,
            in nebula_packed_vector3_array p_pv3a);

        public static partial void nebulasharp_variant_new_packed_vector4_array(out nebula_variant r_dest,
            in nebula_packed_vector4_array p_pv4a);

        public static partial void nebulasharp_variant_new_packed_color_array(out nebula_variant r_dest,
            in nebula_packed_color_array p_pca);

        public static partial nebula_bool nebulasharp_variant_as_bool(in nebula_variant p_self);

        public static partial Int64 nebulasharp_variant_as_int(in nebula_variant p_self);

        public static partial double nebulasharp_variant_as_float(in nebula_variant p_self);

        public static partial nebula_string nebulasharp_variant_as_string(in nebula_variant p_self);

        public static partial Vector2 nebulasharp_variant_as_vector2(in nebula_variant p_self);

        public static partial Vector2I nebulasharp_variant_as_vector2i(in nebula_variant p_self);

        public static partial Rect2 nebulasharp_variant_as_rect2(in nebula_variant p_self);

        public static partial Rect2I nebulasharp_variant_as_rect2i(in nebula_variant p_self);

        public static partial Vector3 nebulasharp_variant_as_vector3(in nebula_variant p_self);

        public static partial Vector3I nebulasharp_variant_as_vector3i(in nebula_variant p_self);

        public static partial Transform2D nebulasharp_variant_as_transform2d(in nebula_variant p_self);

        public static partial Vector4 nebulasharp_variant_as_vector4(in nebula_variant p_self);

        public static partial Vector4I nebulasharp_variant_as_vector4i(in nebula_variant p_self);

        public static partial Plane nebulasharp_variant_as_plane(in nebula_variant p_self);

        public static partial Quaternion nebulasharp_variant_as_quaternion(in nebula_variant p_self);

        public static partial Aabb nebulasharp_variant_as_aabb(in nebula_variant p_self);

        public static partial Basis nebulasharp_variant_as_basis(in nebula_variant p_self);

        public static partial Transform3D nebulasharp_variant_as_transform3d(in nebula_variant p_self);

        public static partial Projection nebulasharp_variant_as_projection(in nebula_variant p_self);

        public static partial Color nebulasharp_variant_as_color(in nebula_variant p_self);

        public static partial nebula_string_name nebulasharp_variant_as_string_name(in nebula_variant p_self);

        public static partial nebula_node_path nebulasharp_variant_as_node_path(in nebula_variant p_self);

        public static partial Rid nebulasharp_variant_as_rid(in nebula_variant p_self);

        public static partial nebula_callable nebulasharp_variant_as_callable(in nebula_variant p_self);

        public static partial nebula_signal nebulasharp_variant_as_signal(in nebula_variant p_self);

        public static partial nebula_dictionary nebulasharp_variant_as_dictionary(in nebula_variant p_self);

        public static partial nebula_array nebulasharp_variant_as_array(in nebula_variant p_self);

        public static partial nebula_packed_byte_array nebulasharp_variant_as_packed_byte_array(in nebula_variant p_self);

        public static partial nebula_packed_int32_array nebulasharp_variant_as_packed_int32_array(in nebula_variant p_self);

        public static partial nebula_packed_int64_array nebulasharp_variant_as_packed_int64_array(in nebula_variant p_self);

        public static partial nebula_packed_float32_array nebulasharp_variant_as_packed_float32_array(
            in nebula_variant p_self);

        public static partial nebula_packed_float64_array nebulasharp_variant_as_packed_float64_array(
            in nebula_variant p_self);

        public static partial nebula_packed_string_array nebulasharp_variant_as_packed_string_array(
            in nebula_variant p_self);

        public static partial nebula_packed_vector2_array nebulasharp_variant_as_packed_vector2_array(
            in nebula_variant p_self);

        public static partial nebula_packed_vector3_array nebulasharp_variant_as_packed_vector3_array(
            in nebula_variant p_self);

        public static partial nebula_packed_vector4_array nebulasharp_variant_as_packed_vector4_array(
            in nebula_variant p_self);

        public static partial nebula_packed_color_array nebulasharp_variant_as_packed_color_array(in nebula_variant p_self);

        public static partial nebula_bool nebulasharp_variant_equals(in nebula_variant p_a, in nebula_variant p_b);

        // string.h

        public static partial void nebulasharp_string_new_with_utf16_chars(out nebula_string r_dest, char* p_contents);

        // string_name.h

        public static partial void nebulasharp_string_name_new_copy(out nebula_string_name r_dest,
            in nebula_string_name p_src);

        // node_path.h

        public static partial void nebulasharp_node_path_new_copy(out nebula_node_path r_dest, in nebula_node_path p_src);

        // array.h

        public static partial void nebulasharp_array_new(out nebula_array r_dest);

        public static partial void nebulasharp_array_new_copy(out nebula_array r_dest, in nebula_array p_src);

        public static partial nebula_variant* nebulasharp_array_ptrw(ref nebula_array p_self);

        // dictionary.h

        public static partial void nebulasharp_dictionary_new(out nebula_dictionary r_dest);

        public static partial void nebulasharp_dictionary_new_copy(out nebula_dictionary r_dest,
            in nebula_dictionary p_src);

        // destroy functions

        public static partial void nebulasharp_packed_byte_array_destroy(ref nebula_packed_byte_array p_self);

        public static partial void nebulasharp_packed_int32_array_destroy(ref nebula_packed_int32_array p_self);

        public static partial void nebulasharp_packed_int64_array_destroy(ref nebula_packed_int64_array p_self);

        public static partial void nebulasharp_packed_float32_array_destroy(ref nebula_packed_float32_array p_self);

        public static partial void nebulasharp_packed_float64_array_destroy(ref nebula_packed_float64_array p_self);

        public static partial void nebulasharp_packed_string_array_destroy(ref nebula_packed_string_array p_self);

        public static partial void nebulasharp_packed_vector2_array_destroy(ref nebula_packed_vector2_array p_self);

        public static partial void nebulasharp_packed_vector3_array_destroy(ref nebula_packed_vector3_array p_self);

        public static partial void nebulasharp_packed_vector4_array_destroy(ref nebula_packed_vector4_array p_self);

        public static partial void nebulasharp_packed_color_array_destroy(ref nebula_packed_color_array p_self);

        public static partial void nebulasharp_variant_destroy(ref nebula_variant p_self);

        public static partial void nebulasharp_string_destroy(ref nebula_string p_self);

        public static partial void nebulasharp_string_name_destroy(ref nebula_string_name p_self);

        public static partial void nebulasharp_node_path_destroy(ref nebula_node_path p_self);

        public static partial void nebulasharp_signal_destroy(ref nebula_signal p_self);

        public static partial void nebulasharp_callable_destroy(ref nebula_callable p_self);

        public static partial void nebulasharp_array_destroy(ref nebula_array p_self);

        public static partial void nebulasharp_dictionary_destroy(ref nebula_dictionary p_self);

        // Array

        public static partial int nebulasharp_array_add(ref nebula_array p_self, in nebula_variant p_item);

        public static partial int nebulasharp_array_add_range(ref nebula_array p_self, in nebula_array p_collection);

        public static partial int nebulasharp_array_binary_search(ref nebula_array p_self, int p_index, int p_count, in nebula_variant p_value);

        public static partial void
            nebulasharp_array_duplicate(ref nebula_array p_self, nebula_bool p_deep, out nebula_array r_dest);

        public static partial void nebulasharp_array_fill(ref nebula_array p_self, in nebula_variant p_value);

        public static partial int nebulasharp_array_index_of(ref nebula_array p_self, in nebula_variant p_item, int p_index = 0);

        public static partial void nebulasharp_array_insert(ref nebula_array p_self, int p_index, in nebula_variant p_item);

        public static partial int nebulasharp_array_last_index_of(ref nebula_array p_self, in nebula_variant p_item, int p_index);

        public static partial void nebulasharp_array_make_read_only(ref nebula_array p_self);

        public static partial void nebulasharp_array_max(ref nebula_array p_self, out nebula_variant r_value);

        public static partial void nebulasharp_array_min(ref nebula_array p_self, out nebula_variant r_value);

        public static partial void nebulasharp_array_pick_random(ref nebula_array p_self, out nebula_variant r_value);

        public static partial nebula_bool nebulasharp_array_recursive_equal(ref nebula_array p_self, in nebula_array p_other);

        public static partial void nebulasharp_array_remove_at(ref nebula_array p_self, int p_index);

        public static partial Error nebulasharp_array_resize(ref nebula_array p_self, int p_new_size);

        public static partial void nebulasharp_array_reverse(ref nebula_array p_self);

        public static partial void nebulasharp_array_shuffle(ref nebula_array p_self);

        public static partial void nebulasharp_array_slice(ref nebula_array p_self, int p_start, int p_end,
            int p_step, nebula_bool p_deep, out nebula_array r_dest);

        public static partial void nebulasharp_array_sort(ref nebula_array p_self);

        public static partial void nebulasharp_array_to_string(ref nebula_array p_self, out nebula_string r_str);

        // Dictionary

        public static partial nebula_bool nebulasharp_dictionary_try_get_value(ref nebula_dictionary p_self,
            in nebula_variant p_key,
            out nebula_variant r_value);

        public static partial void nebulasharp_dictionary_set_value(ref nebula_dictionary p_self, in nebula_variant p_key,
            in nebula_variant p_value);

        public static partial void nebulasharp_dictionary_keys(ref nebula_dictionary p_self, out nebula_array r_dest);

        public static partial void nebulasharp_dictionary_values(ref nebula_dictionary p_self, out nebula_array r_dest);

        public static partial int nebulasharp_dictionary_count(ref nebula_dictionary p_self);

        public static partial void nebulasharp_dictionary_key_value_pair_at(ref nebula_dictionary p_self, int p_index,
            out nebula_variant r_key, out nebula_variant r_value);

        public static partial void nebulasharp_dictionary_add(ref nebula_dictionary p_self, in nebula_variant p_key,
            in nebula_variant p_value);

        public static partial void nebulasharp_dictionary_clear(ref nebula_dictionary p_self);

        public static partial nebula_bool nebulasharp_dictionary_contains_key(ref nebula_dictionary p_self,
            in nebula_variant p_key);

        public static partial void nebulasharp_dictionary_duplicate(ref nebula_dictionary p_self, nebula_bool p_deep,
            out nebula_dictionary r_dest);

        public static partial void nebulasharp_dictionary_merge(ref nebula_dictionary p_self, in nebula_dictionary p_dictionary, nebula_bool p_overwrite);

        public static partial nebula_bool nebulasharp_dictionary_recursive_equal(ref nebula_dictionary p_self, in nebula_dictionary p_other);

        public static partial nebula_bool nebulasharp_dictionary_remove_key(ref nebula_dictionary p_self,
            in nebula_variant p_key);

        public static partial void nebulasharp_dictionary_make_read_only(ref nebula_dictionary p_self);

        public static partial void nebulasharp_dictionary_to_string(ref nebula_dictionary p_self, out nebula_string r_str);

        // StringExtensions

        public static partial void nebulasharp_string_simplify_path(in nebula_string p_self,
            out nebula_string r_simplified_path);

        public static partial void nebulasharp_string_to_camel_case(in nebula_string p_self,
            out nebula_string r_camel_case);

        public static partial void nebulasharp_string_to_pascal_case(in nebula_string p_self,
            out nebula_string r_pascal_case);

        public static partial void nebulasharp_string_to_snake_case(in nebula_string p_self,
            out nebula_string r_snake_case);

        // NodePath

        public static partial void nebulasharp_node_path_get_as_property_path(in nebula_node_path p_self,
            ref nebula_node_path r_dest);

        public static partial void nebulasharp_node_path_get_concatenated_names(in nebula_node_path p_self,
            out nebula_string r_names);

        public static partial void nebulasharp_node_path_get_concatenated_subnames(in nebula_node_path p_self,
            out nebula_string r_subnames);

        public static partial void nebulasharp_node_path_get_name(in nebula_node_path p_self, int p_idx,
            out nebula_string r_name);

        public static partial int nebulasharp_node_path_get_name_count(in nebula_node_path p_self);

        public static partial void nebulasharp_node_path_get_subname(in nebula_node_path p_self, int p_idx,
            out nebula_string r_subname);

        public static partial int nebulasharp_node_path_get_subname_count(in nebula_node_path p_self);

        public static partial nebula_bool nebulasharp_node_path_is_absolute(in nebula_node_path p_self);

        public static partial nebula_bool nebulasharp_node_path_equals(in nebula_node_path p_self, in nebula_node_path p_other);

        public static partial int nebulasharp_node_path_hash(in nebula_node_path p_self);

        // GD, etc

        internal static partial void nebulasharp_bytes_to_var(in nebula_packed_byte_array p_bytes,
            nebula_bool p_allow_objects,
            out nebula_variant r_ret);

        internal static partial void nebulasharp_convert(in nebula_variant p_what, int p_type,
            out nebula_variant r_ret);

        internal static partial int nebulasharp_hash(in nebula_variant p_var);

        internal static partial IntPtr nebulasharp_instance_from_id(ulong p_instance_id);

        internal static partial void nebulasharp_print(in nebula_string p_what);

        public static partial void nebulasharp_print_rich(in nebula_string p_what);

        internal static partial void nebulasharp_printerr(in nebula_string p_what);

        internal static partial void nebulasharp_printraw(in nebula_string p_what);

        internal static partial void nebulasharp_prints(in nebula_string p_what);

        internal static partial void nebulasharp_printt(in nebula_string p_what);

        internal static partial float nebulasharp_randf();

        internal static partial uint nebulasharp_randi();

        internal static partial void nebulasharp_randomize();

        internal static partial double nebulasharp_randf_range(double from, double to);

        internal static partial double nebulasharp_randfn(double mean, double deviation);

        internal static partial int nebulasharp_randi_range(int from, int to);

        internal static partial uint nebulasharp_rand_from_seed(ulong seed, out ulong newSeed);

        internal static partial void nebulasharp_seed(ulong seed);

        internal static partial void nebulasharp_weakref(IntPtr p_obj, out nebula_ref r_weak_ref);

        internal static partial void nebulasharp_str_to_var(in nebula_string p_str, out nebula_variant r_ret);

        internal static partial void nebulasharp_var_to_bytes(in nebula_variant p_what, nebula_bool p_full_objects,
            out nebula_packed_byte_array r_bytes);

        internal static partial void nebulasharp_var_to_str(in nebula_variant p_var, out nebula_string r_ret);

        internal static partial void nebulasharp_err_print_error(in nebula_string p_function, in nebula_string p_file, int p_line, in nebula_string p_error, in nebula_string p_message = default, nebula_bool p_editor_notify = nebula_bool.False, nebula_error_handler_type p_type = nebula_error_handler_type.ERR_HANDLER_ERROR);

        // Object

        public static partial void nebulasharp_object_to_string(IntPtr ptr, out nebula_string r_str);
    }
}
