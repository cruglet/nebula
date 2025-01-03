extends Node

var main_window: Node
var gamelist: Dictionary = {}
var scale_factor: float = 1:
	set(new_scale_factor):
		get_tree().root.content_scale_factor = new_scale_factor
		scale_factor = new_scale_factor
	

func _ready() -> void:
	var gamelist_file: String = FileAccess.open("res://gamelist.json", FileAccess.READ).get_as_text()
	gamelist = JSON.parse_string(gamelist_file)

func change_scene(path: String) -> void:
	main_window.remove_child(main_window.get_child(0))
	main_window.add_child(load(path).instantiate())
