extends Node

var main_window: Node
var gamelist: Dictionary = {}

func _ready() -> void:
	var gamelist_file: String = FileAccess.open("res://gamelist.json", FileAccess.READ).get_as_text()
	gamelist = JSON.parse_string(gamelist_file)
