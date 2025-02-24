namespace Nebula.SourceGenerators.Sample;

public partial class EventSignals : NebulaObject
{
    [Signal]
    public delegate void MySignalEventHandler(string str, int num);
}
