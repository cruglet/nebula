extends Node

@export var animation_player: AnimationPlayer

@export var version_label: Label
@export var file_dialog: FileDialog

const ENGINE_SETTINGS: PackedScene = preload("res://scenes/editor/settings/engine_settings.scn")

func _ready() -> void:
	version_label.text = Singleton.VERSION
	file_dialog.file_selected.connect(_validate_file)
	EventBus.create_project_button_pressed.connect(_create_project)
	#animation_player.play("enter")

func _create_project() -> void:
	file_dialog.filters = ["*.wbfs; Wii Backup FileSsystem File"]
	file_dialog.visible = true

func _validate_file(path: String) -> void:
	var disc: WiiDisc = WiiDisc.open(path)
	
	if disc.game_info.get("id") in Singleton.game_list:
		print("W")
	else:
		Singleton.error.emit("[color=yellow]Unrecognized game file.\n[/color]This game is either invalid or is not\ncompatible with this version of Nebula.")
