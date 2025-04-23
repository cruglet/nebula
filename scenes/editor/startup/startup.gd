extends Control

@onready var animation_player: AnimationPlayer = $AnimationPlayer

func _ready() -> void:
	animation_player.play("startup")
	get_window().min_size = Vector2i(500, 300)

func load_config() -> void:
	if EngineConfig.exists():
		EngineConfig.load()
		Singleton._apply_config()
	else:
		EngineConfig.save()
	
	animation_player.play("startup_finished")
	
func change_to_project_list() -> void:
	get_tree().change_scene_to_file("res://scenes/editor/project_manager/project_manager.tscn")
