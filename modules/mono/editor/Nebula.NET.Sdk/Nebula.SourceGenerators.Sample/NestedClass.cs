using System;

namespace Nebula.SourceGenerators.Sample;

public partial class NestedClass : NebulaObject
{
    public partial class NestedClass2 : NebulaObject
    {
        public partial class NestedClass3 : NebulaObject
        {
            [Signal]
            public delegate void MySignalEventHandler(string str, int num);

            [Export] private String _fieldString = "foo";
            [Export] private String PropertyString { get; set; } = "foo";

            private void Method()
            {
            }
        }
    }
}
