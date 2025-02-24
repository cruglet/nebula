using Nebula;

// This works because it inherits from NebulaObject.
[GlobalClass]
public partial class CustomGlobalClass1 : NebulaObject
{

}

// This works because it inherits from an object that inherits from NebulaObject
[GlobalClass]
public partial class CustomGlobalClass2 : Node
{

}

// This raises a GD0401 diagnostic error: global classes must inherit from NebulaObject
[GlobalClass]
public partial class {|GD0401:CustomGlobalClass3|}
{

}
