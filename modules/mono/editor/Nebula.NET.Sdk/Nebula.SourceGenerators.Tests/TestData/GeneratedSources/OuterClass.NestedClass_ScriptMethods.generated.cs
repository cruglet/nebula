using Nebula;
using Nebula.NativeInterop;

partial struct OuterClass
{
partial class NestedClass
{
#pragma warning disable CS0109 // Disable warning about redundant 'new' keyword
    /// <summary>
    /// Cached StringNames for the methods contained in this class, for fast lookup.
    /// </summary>
    public new class MethodName : global::Nebula.RefCounted.MethodName {
        /// <summary>
        /// Cached name for the '_Get' method.
        /// </summary>
        public new static readonly global::Nebula.StringName @_Get = "_Get";
    }
    /// <summary>
    /// Get the method information for all the methods declared in this class.
    /// This method is used by Nebula to register the available methods in the editor.
    /// Do not call this method.
    /// </summary>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    internal new static global::System.Collections.Generic.List<global::Nebula.Bridge.MethodInfo> GetNebulaMethodList()
    {
        var methods = new global::System.Collections.Generic.List<global::Nebula.Bridge.MethodInfo>(1);
        methods.Add(new(name: MethodName.@_Get, returnVal: new(type: (global::Nebula.Variant.Type)0, name: "", hint: (global::Nebula.PropertyHint)0, hintString: "", usage: (global::Nebula.PropertyUsageFlags)131078, exported: false), flags: (global::Nebula.MethodFlags)1, arguments: new() { new(type: (global::Nebula.Variant.Type)21, name: "property", hint: (global::Nebula.PropertyHint)0, hintString: "", usage: (global::Nebula.PropertyUsageFlags)6, exported: false),  }, defaultArguments: null));
        return methods;
    }
#pragma warning restore CS0109
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override bool InvokeNebulaClassMethod(in nebula_string_name method, NativeVariantPtrArgs args, out nebula_variant ret)
    {
        if (method == MethodName.@_Get && args.Count == 1) {
            var callRet = @_Get(global::Nebula.NativeInterop.VariantUtils.ConvertTo<global::Nebula.StringName>(args[0]));
            ret = global::Nebula.NativeInterop.VariantUtils.CreateFrom<global::Nebula.Variant>(callRet);
            return true;
        }
        return base.InvokeNebulaClassMethod(method, args, out ret);
    }
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override bool HasNebulaClassMethod(in nebula_string_name method)
    {
        if (method == MethodName.@_Get) {
           return true;
        }
        return base.HasNebulaClassMethod(method);
    }
}
}
