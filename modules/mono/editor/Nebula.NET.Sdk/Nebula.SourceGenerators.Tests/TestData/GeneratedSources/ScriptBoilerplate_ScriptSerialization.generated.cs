using Nebula;
using Nebula.NativeInterop;

partial class ScriptBoilerplate
{
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override void SaveNebulaObjectData(global::Nebula.Bridge.NebulaSerializationInfo info)
    {
        base.SaveNebulaObjectData(info);
        info.AddProperty(PropertyName.@_nodePath, global::Nebula.Variant.From<global::Nebula.NodePath>(this.@_nodePath));
        info.AddProperty(PropertyName.@_velocity, global::Nebula.Variant.From<int>(this.@_velocity));
    }
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override void RestoreNebulaObjectData(global::Nebula.Bridge.NebulaSerializationInfo info)
    {
        base.RestoreNebulaObjectData(info);
        if (info.TryGetProperty(PropertyName.@_nodePath, out var _value__nodePath))
            this.@_nodePath = _value__nodePath.As<global::Nebula.NodePath>();
        if (info.TryGetProperty(PropertyName.@_velocity, out var _value__velocity))
            this.@_velocity = _value__velocity.As<int>();
    }
}
