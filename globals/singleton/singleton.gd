@tool
extends Node

signal scale_changed

#region Debug
static var theme: Theme = load("uid://c4pym03mn38io")

@export var editor_settings_window: Window
@export var canvas_layer: CanvasLayer

var time: String:
	get:
		return Time.get_datetime_string_from_system().replace(":", "-").replace("T", "-")
var COMMIT: String:
	get:
		return ""
var formatted_version: String = Nebula.VERSION + "-" + Nebula.BRANCH
var current_project: Nebula.Project
var DEBUG_HEADER: String = \
"""\
<=== Nebula Editor ===>
[color=#656565]
Author: Cruglet
Version: %s
HEAD: %s
[/color]
<=====================>
[color=#808080]
--- Debugging process started ---[/color] \
""" % [formatted_version, COMMIT]
#endregion
var opened_disc: WiiDisc


func _ready() -> void:
	scale_changed.connect(_apply_config)


func show_editor_settings() -> void:
	editor_settings_window.show()


func toast_notification(header: String, description: String, timer: float = 4.0) -> void:
	var new_toast: ToastNotification = load("uid://bcbgbl22p46fl").instantiate()
	
	new_toast.toast_header = header
	new_toast.toast_description = description
	new_toast.time = timer
	
	canvas_layer.add_child(new_toast)


func apply_scale(window: Window) -> void:
	window.reset_size()
	window.content_scale_factor = Nebula.Config.Editor.scale


func _apply_config() -> void:
	get_window().content_scale_factor = Nebula.Config.Editor.scale
	ProjectSettings.set("debug/file_logging/max_log_files", Nebula.Config.Debug.max_logs)


func _print_err(error_message: String, _t: String = "") -> void:
	print_rich("""
<< Error >>
%s

Stack:
%s
""" % [error_message, get_stack()])
