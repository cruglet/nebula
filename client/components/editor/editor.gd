@abstract class_name NebulaEditor
extends Node

const EDITOR_INTERFACE: PackedScene = preload("uid://c0qfd8wovs2p1")

const FILESYSTEM_PANEL: PackedScene = preload("uid://bc2m8a8pmx73i")
const TEXT_EDITOR: PackedScene = preload("uid://cjs0c72eukt3x")
const PROPERTIES: PackedScene = preload("uid://dwoslainqrwl6")

enum Dock {
	NONE,
	MAIN,
	TOP_RIGHT,
	BOTTOM_RIGHT,
}

var loaded_project_path: String
var interface: NebulaEditorInterface


func _ready() -> void:
	loaded_project_path = ProjectData.get_path().get_base_dir()
	interface = EDITOR_INTERFACE.instantiate()
	add_child(interface)
	_prepare_menu()
	_prepare_docks()
	_on_finish()


@abstract func _prepare_menu() -> void
@abstract func _prepare_docks() -> void

func _on_finish() -> void:
	interface.done()
