using Nebula;
using Nebula.NativeInterop;

partial struct OuterClass
{
partial class NestedClass
{
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override void SaveNebulaObjectData(global::Nebula.Bridge.NebulaSerializationInfo info)
    {
        base.SaveNebulaObjectData(info);
    }
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override void RestoreNebulaObjectData(global::Nebula.Bridge.NebulaSerializationInfo info)
    {
        base.RestoreNebulaObjectData(info);
    }
}
}
