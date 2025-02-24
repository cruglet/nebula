using Nebula;
using Nebula.Collections;

public partial class ExportDiagnostics_GD0107_OK : Node
{
    [Export]
    public Node NodeField;

    [Export]
    public Node[] SystemArrayOfNodesField;

    [Export]
    public Array<Node> NebulaArrayOfNodesField;

    [Export]
    public Dictionary<Node, string> NebulaDictionaryWithNodeAsKeyField;

    [Export]
    public Dictionary<string, Node> NebulaDictionaryWithNodeAsValueField;

    [Export]
    public Node NodeProperty { get; set; }

    [Export]
    public Node[] SystemArrayOfNodesProperty { get; set; }

    [Export]
    public Array<Node> NebulaArrayOfNodesProperty { get; set; }

    [Export]
    public Dictionary<Node, string> NebulaDictionaryWithNodeAsKeyProperty { get; set; }

    [Export]
    public Dictionary<string, Node> NebulaDictionaryWithNodeAsValueProperty { get; set; }
}

public partial class ExportDiagnostics_GD0107_KO : Resource
{
    [Export]
    public Node {|GD0107:NodeField|};

    [Export]
    public Node[] {|GD0107:SystemArrayOfNodesField|};

    [Export]
    public Array<Node> {|GD0107:NebulaArrayOfNodesField|};

    [Export]
    public Dictionary<Node, string> {|GD0107:NebulaDictionaryWithNodeAsKeyField|};

    [Export]
    public Dictionary<string, Node> {|GD0107:NebulaDictionaryWithNodeAsValueField|};

    [Export]
    public Node {|GD0107:NodeProperty|} { get; set; }

    [Export]
    public Node[] {|GD0107:SystemArrayOfNodesProperty|} { get; set; }

    [Export]
    public Array<Node> {|GD0107:NebulaArrayOfNodesProperty|} { get; set; }

    [Export]
    public Dictionary<Node, string> {|GD0107:NebulaDictionaryWithNodeAsKeyProperty|} { get; set; }

    [Export]
    public Dictionary<string, Node> {|GD0107:NebulaDictionaryWithNodeAsValueProperty|} { get; set; }
}
