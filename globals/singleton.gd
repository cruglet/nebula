@tool
extends Node

static var theme: Theme = load("uid://c4pym03mn38io")

var game_list: Array[String] = ["SMN*01"]
var time: String:
	get:
		return Time.get_datetime_string_from_system().replace(":", "-").replace("T", "-")

#region Debug
const VERSION: String = "v0.0.0"
const BRANCH: String = "dev"
var COMMIT: String = "c1321vr1c23"

var formatted_version: String = VERSION + "-" + BRANCH 

var DEBUG_HEADER: String = \
"""\
<=== Nebula Engine ===>
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

#region Error Window
var error_dialog: Node = preload("uid://bw1atuniksvwj").instantiate()
var error_window: Window
signal error(message: String, conf_message: String, log_err: bool)
#endregion

#region Popup Window
var popup_window: Window
signal popup(content: Control)
signal close_popup()
#endregion

signal scale_changed

func _ready() -> void:
	print_rich(DEBUG_HEADER)
	_add_error_window()
	_add_popup_window()
	
	close_popup.connect(func() -> void:
		popup_window.visible = false
	)
	
	scale_changed.connect(_apply_config)

func _apply_config() -> void:
	get_window().content_scale_factor = EngineConfig.scale
	ProjectSettings.set("debug/file_logging/max_log_files", EngineConfig.max_logs)

func _add_error_window() -> void:
	error_window = Window.new()
	error_window.visible = false
	error_window.wrap_controls = true
	error_window.transient = true
	error_window.exclusive = true
	error_window.unresizable = true
	error_window.popup_window = true
	error_window.force_native = true
	error_window.borderless = true
	
	error_window.add_child(error_dialog)
	error_dialog.ok_button.pressed.connect(_hide_error)
	
	
	error.connect(_show_error)
	scale_changed.connect(func() -> void:
		error_window.size = error_window.content_scale_size * EngineConfig.scale
		error_window.content_scale_factor = EngineConfig.scale
	)
	
	add_child(error_window)

func _show_error(message: String, conf_message: String = "Ok", log_err: bool = true) -> void:
	
	if error_window.visible:
		return
	
	if log_err:
		_print_err(message)
	
	error_window.visible = true
	error_dialog.error_message.text = message
	error_dialog.ok_button.text = conf_message
	
	# This allows the window to resize properly
	error_window.size = Vector2.ZERO
	
	error_window.size.x = (error_dialog.error_message.size.x + 50)
	error_window.size.y = (error_dialog.error_message.size.y + 110)
	error_window.content_scale_size = error_window.size
	
	error_window.content_scale_factor = EngineConfig.scale
	error_window.size *= EngineConfig.scale

func _hide_error() -> void:
	error_window.visible = false

func _print_err(error_message: String, _t: String = "") -> void:
	print_rich("""
<< Error >>
%s

Stack:
%s
""" % [error_message, get_stack()])


func _add_popup_window() -> void:
	popup_window = Window.new()
	popup_window.visible = false
	popup_window.wrap_controls = true
	popup_window.transient = true
	popup_window.exclusive = true
	popup_window.unresizable = true
	popup_window.popup_window = true
	popup_window.force_native = true
	popup_window.borderless = true
	
	popup.connect(_show_popup)
	scale_changed.connect(func() -> void:
		popup_window.size = popup_window.content_scale_size * EngineConfig.scale
		popup_window.content_scale_factor = EngineConfig.scale
	)
	add_child(popup_window)

func _show_popup(content: Control) -> void:
	for child: Node in popup_window.get_children():
		child.free()

	popup_window.add_child(content)
	popup_window.visible = true
	
	# This allows the window to resize properly
	popup_window.size = Vector2.ZERO
	await get_tree().process_frame
	await get_tree().process_frame
	
	popup_window.size = content.size * EngineConfig.scale
	popup_window.content_scale_size = popup_window.size
