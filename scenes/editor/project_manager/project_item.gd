class_name ProjectItem
extends Button

@export var project_name_label: Label
@export var project_path_label: Label
@export var project_banner: TextureRect
## This determines whether the item is clickable
@export var enabled: bool = true:
	set(e):
		enabled = e
		if not enabled:
			process_mode = Node.PROCESS_MODE_DISABLED
			mouse_filter = Control.MOUSE_FILTER_IGNORE
		else:
			process_mode = Node.PROCESS_MODE_INHERIT
			mouse_filter = Control.MOUSE_FILTER_STOP

var custom_project_path: String

var project_name: String:
	set(new_name):
		if new_name.length() == 0:
			project_name_label.text = "My Project"
			project_path = "my-project"
		else:
			project_name_label.text = new_name
			project_path = new_name.to_kebab_case()
		project_name = new_name

var project_path: String:
	set(new_path):
		if new_path and !custom_project_path:
			project_path = Nebula.Config.Editor.default_project_path.path_join(new_path) + '/'
		elif custom_project_path:
			project_path = custom_project_path.path_join(new_path) + '/'
		else:
			project_path = Nebula.Config.Editor.default_project_path.path_join("my-project/")
		project_path_label.text = project_path


func _on_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.double_click:
		var project: Nebula.Project = Nebula.Project.load(project_path.path_join("project.nebula"))
		var editor_path: String = Nebula.GAME_LIST.get(Nebula.find_in_game_list(project.type)).editor
		
		get_tree().change_scene_to_file(editor_path)
