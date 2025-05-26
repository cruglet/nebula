extends Node


func _on_close_button_pressed() -> void:
	Nebula.Config.save()
	Singleton.editor_settings_window.hide()
