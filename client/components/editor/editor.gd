@abstract class_name NebulaEditor
extends Node

var loaded_project_path: String

func _ready() -> void:
	loaded_project_path = ProjectData.get_path().get_base_dir()
	_prepare_menu()
	_prepare_docks()
	_on_finish()


@abstract func _prepare_menu() -> void
@abstract func _prepare_docks() -> void
@abstract func _on_finish() -> void



func bind_save_status() -> void:
	pass
