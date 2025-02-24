using Nebula;
using Nebula.NativeInterop;

partial class AbstractGenericNode<T>
{
#pragma warning disable CS0109 // Disable warning about redundant 'new' keyword
    /// <summary>
    /// Cached StringNames for the properties and fields contained in this class, for fast lookup.
    /// </summary>
    public new class PropertyName : global::Nebula.Node.PropertyName {
        /// <summary>
        /// Cached name for the 'MyArray' property.
        /// </summary>
        public new static readonly global::Nebula.StringName @MyArray = "MyArray";
    }
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override bool SetNebulaClassPropertyValue(in nebula_string_name name, in nebula_variant value)
    {
        if (name == PropertyName.@MyArray) {
            this.@MyArray = global::Nebula.NativeInterop.VariantUtils.ConvertToArray<T>(value);
            return true;
        }
        return base.SetNebulaClassPropertyValue(name, value);
    }
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override bool GetNebulaClassPropertyValue(in nebula_string_name name, out nebula_variant value)
    {
        if (name == PropertyName.@MyArray) {
            value = global::Nebula.NativeInterop.VariantUtils.CreateFromArray(this.@MyArray);
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
        properties.Add(new(type: (global::Nebula.Variant.Type)28, name: PropertyName.@MyArray, hint: (global::Nebula.PropertyHint)0, hintString: "", usage: (global::Nebula.PropertyUsageFlags)4102, exported: true));
        return properties;
    }
#pragma warning restore CS0109
}
