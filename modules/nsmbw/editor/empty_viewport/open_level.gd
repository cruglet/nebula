extends Node


func _on_open_level_button_pressed() -> void:
	get_tree().root.emit_signal(&"nsmbw_open_level")
