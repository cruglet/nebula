using Nebula;

// This works because it inherits from NebulaObject and it doesn't have any generic type parameter.
[GlobalClass]
public partial class CustomGlobalClass : NebulaObject
{

}

// This raises a GD0402 diagnostic error: global classes can't have any generic type parameter
[GlobalClass]
public partial class {|GD0402:CustomGlobalClass|}<T> : NebulaObject
{

}
