using Nebula;

public partial class Bar : NebulaObject
{
}

// Foo in another file
public partial class Foo
{
}

public partial class NotSameNameAsFile : NebulaObject
{
}
