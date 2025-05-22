extends Node

@export var editor_settings_window: Window
@export var editor_settings: PanelContainer


func _on_editor_settings_button_pressed() -> void:
	Singleton.show_editor_settings()
