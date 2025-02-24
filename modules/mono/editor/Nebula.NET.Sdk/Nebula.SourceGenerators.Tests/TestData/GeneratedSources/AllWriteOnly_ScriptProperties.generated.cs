using Nebula;
using Nebula.NativeInterop;

partial class AllWriteOnly
{
#pragma warning disable CS0109 // Disable warning about redundant 'new' keyword
    /// <summary>
    /// Cached StringNames for the properties and fields contained in this class, for fast lookup.
    /// </summary>
    public new class PropertyName : global::Nebula.NebulaObject.PropertyName {
        /// <summary>
        /// Cached name for the 'WriteOnlyProperty' property.
        /// </summary>
        public new static readonly global::Nebula.StringName @WriteOnlyProperty = "WriteOnlyProperty";
        /// <summary>
        /// Cached name for the '_writeOnlyBackingField' field.
        /// </summary>
        public new static readonly global::Nebula.StringName @_writeOnlyBackingField = "_writeOnlyBackingField";
    }
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override bool SetNebulaClassPropertyValue(in nebula_string_name name, in nebula_variant value)
    {
        if (name == PropertyName.@WriteOnlyProperty) {
            this.@WriteOnlyProperty = global::Nebula.NativeInterop.VariantUtils.ConvertTo<bool>(value);
            return true;
        }
        if (name == PropertyName.@_writeOnlyBackingField) {
            this.@_writeOnlyBackingField = global::Nebula.NativeInterop.VariantUtils.ConvertTo<bool>(value);
            return true;
        }
        return base.SetNebulaClassPropertyValue(name, value);
    }
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override bool GetNebulaClassPropertyValue(in nebula_string_name name, out nebula_variant value)
    {
        if (name == PropertyName.@_writeOnlyBackingField) {
            value = global::Nebula.NativeInterop.VariantUtils.CreateFrom<bool>(this.@_writeOnlyBackingField);
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
        properties.Add(new(type: (global::Nebula.Variant.Type)1, name: PropertyName.@_writeOnlyBackingField, hint: (global::Nebula.PropertyHint)0, hintString: "", usage: (global::Nebula.PropertyUsageFlags)4096, exported: false));
        properties.Add(new(type: (global::Nebula.Variant.Type)1, name: PropertyName.@WriteOnlyProperty, hint: (global::Nebula.PropertyHint)0, hintString: "", usage: (global::Nebula.PropertyUsageFlags)4096, exported: false));
        return properties;
    }
#pragma warning restore CS0109
}
