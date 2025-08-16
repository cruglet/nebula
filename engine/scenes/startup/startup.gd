extends Control

const SPLASH_MESSAGES: Array[String] = [
	"Loading...",
	"Getting things ready...",
	"Boot sequence initiated...",
	"Finding star coins...",
	"Preparing warp pipes...",
	"Consuming star bits...",
	"Hoarding 1-UPs...",
	"Calibrating warp zones...",
	"Aligning launch stars...",
	"Scanning for hidden blocks...",
	"Preparing cannons...",
	"Gathering Miis...",
	"Collecting red coins...",
	"Launching into space...",
	"Fighting with the camera...",
	"Winning by doing absolutely nothing...",
	"Breaking blocks...",
	"Placing blocks...",
	"Questioning blocks...",
	"Kicking shells...",
	"Debugging for hours...",
]

@export var animation_player: AnimationPlayer
@export var splash_texture: TextureRect
@export var splash_label: Label


func _ready() -> void:
	splash_label.text = SPLASH_MESSAGES.pick_random()
	animation_player.play(&"startup")


func _after_startup_animation() -> void:
	load_and_validate_modules()
	animation_player.play(&"startup_finished")


func load_and_validate_modules() -> void:
	var modules: Array = CoreSettings.get(CoreSettings.SETTING_MODULE_LIST)
	
	for i: int in range(modules.size()):
		var module_path: String = modules.get(i)
		
		if not FileAccess.file_exists(module_path):
			push_warning("Module file deleted or moved! (%s). Removing..." % module_path)
			modules.remove_at(i)
			continue
		
		var m: Module = Module.load(module_path)
		m.set_meta(&"path", module_path)
		Singleton.loaded_modules.set(m.id, m)
	
	CoreSettings.set(CoreSettings.SETTING_MODULE_LIST, modules)


func _after_startup_finished_animation() -> void:
	CoreSettings.apply_config()
	get_tree().change_scene_to_file("uid://b0vmxnd68vv2l")
