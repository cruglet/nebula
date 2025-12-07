class_name NebulaInfoWindow
extends NebulaWindow

const INFO_WINDOW: PackedScene = preload("uid://doh2gawt7opik")

static var in_dashboard: bool = true:
	set(id):
		if get_instance():
			get_instance().project_list_button.visible = not id
		in_dashboard = id

static var _ref: NebulaInfoWindow

@export var version_label: Label
@export var project_list_button: Button


static func get_instance() -> NebulaInfoWindow:
	if not is_instance_valid(_ref):
		_ref = INFO_WINDOW.instantiate()
		_ref.version_label.text = "v" + Nebula.get_version_string()
		Singleton.add_child(_ref)
	return _ref


func _on_project_list_button_pressed() -> void:
	hide()
	in_dashboard = true
	get_tree().change_scene_to_file("res://scenes/dashboard/dashboard.tscn")
