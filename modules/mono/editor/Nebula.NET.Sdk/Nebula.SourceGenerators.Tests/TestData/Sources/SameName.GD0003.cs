using Nebula;

namespace NamespaceA
{
    partial class SameName : NebulaObject
    {
        private int _field;
    }
}

// SameName again but different namespace
namespace NamespaceB
{
    partial class {|GD0003:SameName|} : NebulaObject
    {
        private int _field;
    }
}
