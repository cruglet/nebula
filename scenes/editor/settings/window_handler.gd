extends Node


func _on_close_button_pressed() -> void:
	NebulaConfig.save()
	Singleton.editor_settings_window.hide()
