extends Node

@export var empty_project_list_container: Control
@export var project_list_container: Control


func _ready() -> void:
	check_project_list()


func check_project_list() -> void:
	if NebulaConfig.projects.size() > 0:
		project_list_container.show()
		empty_project_list_container.hide()
	else:
		project_list_container.hide()
		empty_project_list_container.show()
