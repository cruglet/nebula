extends Control

@onready var panel: Button = $Panel

@onready var preview: Control = $"."
@onready var preview_name: Label = %ProjName
@onready var preview_dir: Label = %ProjDir
@onready var preview_image: TextureRect = $Panel/HBoxContainer/Right/PreviewImage

@onready var buttons_container: HBoxContainer = $Panel/HBoxContainer/Right/Buttons

var _name: String = ""
var _dir: String = ""
var _image: String = ""
var real: bool = false

signal delete_project_pressed(preview_node: Control)

func _ready() -> void:
	preview_name.text = _name
	preview_dir.text = _dir
	preview_image.texture = load(_image)
	if real:
		_enable_project_buttons()
	else:
		panel.mouse_default_cursor_shape = Control.CURSOR_ARROW
		panel.disabled = true
		buttons_container.visible = false

func _enable_project_buttons() -> void:
	buttons_container.visible = true
	panel.disabled = false
	panel.mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND

func _on_panel_pressed() -> void:
	var project_file: FileAccess = FileAccess.open(preview_dir.text + "/project.nebula", FileAccess.READ)
	var project_info: Dictionary = project_file.get_var(true)
	
	var editor_path: String = "res://engines/%s/editor/editor.tscn" % project_info.get("game_info").get("engine_id")
	if load(editor_path):
		Singleton.change_scene(editor_path)


func _on_delete_project_pressed() -> void:
	delete_project_pressed.emit(preview)
