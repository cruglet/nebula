extends Node

enum {
	OPEN_LEVEL = 1,
	CLOSE_PROJECT = 2,
}

@export var project_popup: PopupMenu



func _ready() -> void:
	project_popup.id_pressed.connect(_pressed)


func _pressed(i: int) -> void:
	match i:
		OPEN_LEVEL:
			get_tree().root.emit_signal(&"nsmbw_open_level")
		CLOSE_PROJECT:
			get_tree().change_scene_to_file("uid://c14cenqcdq6h4")
