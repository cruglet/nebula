extends Node

const MAIN_MENU: PackedScene = preload("res://menus/main_menu/main_menu.tscn")

const MIN_WINDOW_SIZE: Vector2i = Vector2i(760, 500)


func _ready() -> void:
	Singleton.main_window = self
	_ensure_file_integrity()
	set_window_config()
	add_child(MAIN_MENU.instantiate())

func _ensure_file_integrity() -> void:
	if !FileAccess.file_exists("res://projectlist.json"):
		var file: FileAccess = FileAccess.open("res://projectlist.json", FileAccess.WRITE)
		file.close()

func set_window_config() -> void:
	DisplayServer.window_set_min_size(MIN_WINDOW_SIZE)
	get_window().min_size = MIN_WINDOW_SIZE
