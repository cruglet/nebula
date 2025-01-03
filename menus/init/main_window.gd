extends Node

const MAIN_MENU: PackedScene = preload("res://menus/main_menu/main_menu.tscn")

const MIN_WINDOW_SIZE: Vector2i = Vector2i(760, 500)


func _ready() -> void:
	Singleton.main_window = self
	set_window_config()
	add_child(MAIN_MENU.instantiate())

func set_window_config() -> void:
	DisplayServer.window_set_min_size(MIN_WINDOW_SIZE)
	get_window().min_size = MIN_WINDOW_SIZE
