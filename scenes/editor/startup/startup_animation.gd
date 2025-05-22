extends Node

@export var animation_player: AnimationPlayer

func _ready() -> void:
	await owner.ready
	animation_player.play("startup")
	get_window().min_size = Vector2i(500, 300)
	
func change_to_project_list() -> void:
	get_tree().change_scene_to_file("res://scenes/editor/project_manager/project_manager.tscn")
