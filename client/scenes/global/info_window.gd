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
@export var debug_label: RichTextLabel


static func get_instance() -> NebulaInfoWindow:
	if not is_instance_valid(_ref):
		_ref = INFO_WINDOW.instantiate()
		_ref.version_label.text = "dev " if OS.is_debug_build() else "v"
		_ref.version_label.text += Nebula.get_version_string()
		Singleton.add_child(_ref)
	return _ref


func _on_ready() -> void:
	if OS.is_debug_build():
		debug_label.show()
		fetch_debug_info()


func fetch_debug_info() -> void:
	var debug_text: String = "nebula.%s.%s.%s" % [OS.get_name().to_lower(), Git.get_current_branch(), Git.get_current_short_hash()]
	debug_label.text = debug_text
	debug_label.text += "\n" + OS.get_distribution_name()


func _on_project_list_button_pressed() -> void:
	hide()
	in_dashboard = true
	get_tree().change_scene_to_file("res://scenes/dashboard/dashboard.tscn")
