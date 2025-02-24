using System;

namespace Nebula.SourceGenerators.Sample
{
    public partial class AllWriteOnly : NebulaObject
    {
        private bool _writeOnlyBackingField = false;
        public bool WriteOnlyProperty { set => _writeOnlyBackingField = value; }
    }
}
