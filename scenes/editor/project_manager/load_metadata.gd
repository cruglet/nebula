extends Node

@export var version_label: Label


func _ready() -> void:
	version_label.text = "v" + Nebula.VERSION
