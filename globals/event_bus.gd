extends Node

signal create_project_button_pressed()
signal create_project_request(project_name: String, project_path: String)
signal project_ready


func _connect() -> void:
	pass


func _emit(signal_name: StringName) -> void:
	pass
