extends Node

enum {
	SETTINGS = 1,
}

@export var editor_popup: PopupMenu



func _ready() -> void:
	editor_popup.id_pressed.connect(_pressed)


func _pressed(i: int) -> void:
	match i:
		SETTINGS:
			Singleton.show_editor_settings()
