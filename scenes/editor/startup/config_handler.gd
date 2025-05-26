extends Node

@export var animation_player: AnimationPlayer

func load_config() -> void:
	if Nebula.Config.exists():
		Nebula.Config.load()
	else:
		Nebula.Config.save()
	
	animation_player.play("startup_finished")

func apply_config() -> void:
	Singleton._apply_config()
