extends Node

@onready var file_dialog: FileDialog = %FileDialog

const NO_PROJECTS: PackedScene = preload("res://scenes/editor/project_manager/home/no_projects.scn")

signal file_selected(path: String)

func _ready() -> void:
	var content: Node
	if EngineConfig.projects.size() > 0:
		pass
	else:
		content = NO_PROJECTS.instantiate()
		owner.add_child.call_deferred(content)
	#content.create_button_pressed.connect(_on_create_button_pressed)

func _on_create_button_pressed() -> void:
	file_dialog.visible = true

func _on_file_selected(_path: String) -> void:
	file_selected.emit(_path)
