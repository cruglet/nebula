extends Node

@export var engine_settings_window: Window
@export var engine_settings: PanelContainer

const ENGINE_SETTINGS: PackedScene = preload("res://scenes/editor/project_manager/window/settings/engine_settings.scn")


func _on_engine_settings_button_pressed() -> void:
	Singleton.popup.emit(ENGINE_SETTINGS.instantiate())

func _on_engine_settings_window_close_requested() -> void:
	engine_settings_window.visible = false

func _on_file_selected(_path: String) -> void:
	var disc: WiiDisc = WiiDisc.open(_path)
