class_name ProjectManager
extends Node


static var DEFAULT_PROJECT_DATA: Dictionary = {
	"SMN#01": {
		"project": {
			"type": "SMN#01",
			"name": "",
			"path": "",
		},
		"editor": {
			"instances_opened": [],
		},
		"metadata": {
			"version": Nebula.VERSION,
			"created": Singleton.time,
			"last_opened": Singleton.time
		},
	}
}

static func get_default_project_data(editor_id: String, project_name: String, project_path: String) -> Dictionary:
	var data: Dictionary = DEFAULT_PROJECT_DATA.get(editor_id).duplicate(true)
	
	data.project.name = project_name
	data.project.path = project_path
	
	return data
