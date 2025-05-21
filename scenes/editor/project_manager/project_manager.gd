extends Node

const CREATE_PROJECT: PackedScene = preload("res://scenes/editor/project_manager/window/create_project/create_project.scn")

@export var animation_player: AnimationPlayer
@export var version_label: Label
@export var file_dialog: FileDialog
@export var content: Control
@export var project_create_process: Node

var disc: WiiDisc
var thread: Thread = Thread.new()


func _ready() -> void:
	version_label.text = Nebula.VERSION
	file_dialog.file_selected.connect(validate_file)
	EventBus.create_project_button_pressed.connect(create_project_popup)
	EventBus.create_project_request.connect(initialize_project)


func create_project_popup() -> void:
	file_dialog.filters = ["*.wbfs; Wii Backup FileSsystem File"]
	file_dialog.visible = true


func validate_file(path: String) -> void:
	disc = WiiDisc.open(path)
	
	if Nebula.find_in_game_list(disc.game_id):
		var create_project_popup: Node = CREATE_PROJECT.instantiate()
		var disc_banner: Texture2D = disc.get_banner()
		create_project_popup.project_preview.project_banner.texture = disc_banner
		Singleton.popup.emit(create_project_popup)
	else:
		Singleton.error.emit("[color=yellow]Unrecognized game file.\n[/color]This game is either invalid or is not\ncompatible with this version of Nebula.")


func initialize_project(project_name: String, project_path: String) -> void:
	animation_player.play(&"start_to_load")
	animation_player.animation_finished.connect(func(_x: StringName) -> void:
			animation_player.play(&"loading_loop")
	)

	thread.start(project_create_process.create_project.bind(project_name, project_path, disc))
