extends Node

@export var settings_window: Window


func _ready() -> void:
	Singleton.scale_changed.connect(_on_scale_changed)


func _on_scale_changed() -> void:
	settings_window.content_scale_factor = Nebula.Config.Editor.scale


func _on_settings_window_close_requested() -> void:
	settings_window.hide()
