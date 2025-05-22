class_name ProjectItem
extends PanelContainer

@export var project_name_label: Label
@export var project_path_label: Label
@export var project_banner: TextureRect

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
			project_path = NebulaConfig.default_project_path.path_join(new_path) + '/'
		elif custom_project_path:
			project_path = custom_project_path.path_join(new_path) + '/'
		else:
			project_path = NebulaConfig.default_project_path.path_join("my-project/")
		project_path_label.text = project_path
