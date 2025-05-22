extends Node

@export var project_preview: ProjectItem


func _on_create_button_pressed() -> void:
	owner.get_parent().hide()
	get_tree().root.emit_signal(&"create_project", project_preview.project_name, project_preview.project_path)
