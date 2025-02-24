using Nebula;
using Nebula.NativeInterop;

partial class ScriptBoilerplate
{
#pragma warning disable CS0109 // Disable warning about redundant 'new' keyword
    /// <summary>
    /// Cached StringNames for the properties and fields contained in this class, for fast lookup.
    /// </summary>
    public new class PropertyName : global::Nebula.Node.PropertyName {
        /// <summary>
        /// Cached name for the '_nodePath' field.
        /// </summary>
        public new static readonly global::Nebula.StringName @_nodePath = "_nodePath";
        /// <summary>
        /// Cached name for the '_velocity' field.
        /// </summary>
        public new static readonly global::Nebula.StringName @_velocity = "_velocity";
    }
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override bool SetNebulaClassPropertyValue(in nebula_string_name name, in nebula_variant value)
    {
        if (name == PropertyName.@_nodePath) {
            this.@_nodePath = global::Nebula.NativeInterop.VariantUtils.ConvertTo<global::Nebula.NodePath>(value);
            return true;
        }
        if (name == PropertyName.@_velocity) {
            this.@_velocity = global::Nebula.NativeInterop.VariantUtils.ConvertTo<int>(value);
            return true;
        }
        return base.SetNebulaClassPropertyValue(name, value);
    }
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override bool GetNebulaClassPropertyValue(in nebula_string_name name, out nebula_variant value)
    {
        if (name == PropertyName.@_nodePath) {
            value = global::Nebula.NativeInterop.VariantUtils.CreateFrom<global::Nebula.NodePath>(this.@_nodePath);
            return true;
        }
        if (name == PropertyName.@_velocity) {
            value = global::Nebula.NativeInterop.VariantUtils.CreateFrom<int>(this.@_velocity);
            return true;
        }
        return base.GetNebulaClassPropertyValue(name, out value);
    }
    /// <summary>
    /// Get the property information for all the properties declared in this class.
    /// This method is used by Nebula to register the available properties in the editor.
    /// Do not call this method.
    /// </summary>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    internal new static global::System.Collections.Generic.List<global::Nebula.Bridge.PropertyInfo> GetNebulaPropertyList()
    {
        var properties = new global::System.Collections.Generic.List<global::Nebula.Bridge.PropertyInfo>();
        properties.Add(new(type: (global::Nebula.Variant.Type)22, name: PropertyName.@_nodePath, hint: (global::Nebula.PropertyHint)0, hintString: "", usage: (global::Nebula.PropertyUsageFlags)4096, exported: false));
        properties.Add(new(type: (global::Nebula.Variant.Type)2, name: PropertyName.@_velocity, hint: (global::Nebula.PropertyHint)0, hintString: "", usage: (global::Nebula.PropertyUsageFlags)4096, exported: false));
        return properties;
    }
#pragma warning restore CS0109
}
