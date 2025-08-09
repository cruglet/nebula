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
	animation_player.play(&"startup_finished")


func _after_startup_finished_animation() -> void:
	CoreSettings.apply_config()
	get_tree().change_scene_to_file("uid://b0vmxnd68vv2l")
