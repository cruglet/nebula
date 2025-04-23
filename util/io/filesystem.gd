class_name Filesystem extends Node

static func move(from: String, to: String) -> void:
	DirAccess.rename_absolute(from, to)
