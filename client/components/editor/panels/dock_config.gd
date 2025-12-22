class_name NebulaDockConfig
extends Resource

@export var name: String
@export var scene: PackedScene
@export var empty_scene: PackedScene
@export var hide_on_empty: bool = true
@export var bound_dock: NebulaEditor.Dock
@export var bound_types: Array[String]
