using Nebula;
using Nebula.NativeInterop;

partial class EventSignals
{
#pragma warning disable CS0109 // Disable warning about redundant 'new' keyword
    /// <summary>
    /// Cached StringNames for the signals contained in this class, for fast lookup.
    /// </summary>
    public new class SignalName : global::Nebula.NebulaObject.SignalName {
        /// <summary>
        /// Cached name for the 'MySignal' signal.
        /// </summary>
        public new static readonly global::Nebula.StringName @MySignal = "MySignal";
    }
    /// <summary>
    /// Get the signal information for all the signals declared in this class.
    /// This method is used by Nebula to register the available signals in the editor.
    /// Do not call this method.
    /// </summary>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    internal new static global::System.Collections.Generic.List<global::Nebula.Bridge.MethodInfo> GetNebulaSignalList()
    {
        var signals = new global::System.Collections.Generic.List<global::Nebula.Bridge.MethodInfo>(1);
        signals.Add(new(name: SignalName.@MySignal, returnVal: new(type: (global::Nebula.Variant.Type)0, name: "", hint: (global::Nebula.PropertyHint)0, hintString: "", usage: (global::Nebula.PropertyUsageFlags)6, exported: false), flags: (global::Nebula.MethodFlags)1, arguments: new() { new(type: (global::Nebula.Variant.Type)4, name: "str", hint: (global::Nebula.PropertyHint)0, hintString: "", usage: (global::Nebula.PropertyUsageFlags)6, exported: false), new(type: (global::Nebula.Variant.Type)2, name: "num", hint: (global::Nebula.PropertyHint)0, hintString: "", usage: (global::Nebula.PropertyUsageFlags)6, exported: false),  }, defaultArguments: null));
        return signals;
    }
#pragma warning restore CS0109
    private global::EventSignals.MySignalEventHandler backing_MySignal;
    /// <inheritdoc cref="global::EventSignals.MySignalEventHandler"/>
    public event global::EventSignals.MySignalEventHandler @MySignal {
        add => backing_MySignal += value;
        remove => backing_MySignal -= value;
}
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override void RaiseNebulaClassSignalCallbacks(in nebula_string_name signal, NativeVariantPtrArgs args)
    {
        if (signal == SignalName.@MySignal && args.Count == 2) {
            backing_MySignal?.Invoke(global::Nebula.NativeInterop.VariantUtils.ConvertTo<string>(args[0]), global::Nebula.NativeInterop.VariantUtils.ConvertTo<int>(args[1]));
            return;
        }
        base.RaiseNebulaClassSignalCallbacks(signal, args);
    }
    /// <inheritdoc/>
    [global::System.ComponentModel.EditorBrowsable(global::System.ComponentModel.EditorBrowsableState.Never)]
    protected override bool HasNebulaClassSignal(in nebula_string_name signal)
    {
        if (signal == SignalName.@MySignal) {
           return true;
        }
        return base.HasNebulaClassSignal(signal);
    }
}
