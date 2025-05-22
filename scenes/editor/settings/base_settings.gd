extends Node

@export var project_path_button: Button
@export var game_path_button: Button
@export var project_path_dialog: FileDialog


func _ready() -> void:
	project_path_button.text = NebulaConfig.default_project_path
	project_path_button.tooltip_text = project_path_button.text
	
	game_path_button.text = NebulaConfig.default_game_path
	game_path_button.tooltip_text = game_path_button.text


func _on_default_project_path_button_pressed() -> void:
	project_path_dialog.show()
	project_path_dialog.confirmed.connect(_on_default_path_dialog_dir_selected, CONNECT_ONE_SHOT)


func _on_default_path_dialog_dir_selected(dir: String) -> void:
	NebulaConfig.default_project_path = dir
	project_path_button.text = NebulaConfig.default_project_path
	project_path_button.tooltip_text = project_path_button.text


func _on_default_game_path_button_pressed() -> void:
	project_path_dialog.show()
	project_path_dialog.confirmed.connect(_on_default_game_path_dialog_dir_selected, CONNECT_ONE_SHOT)


func _on_default_game_path_dialog_dir_selected(dir: String) -> void:
	NebulaConfig.default_game_path = dir
	game_path_button.text = NebulaConfig.default_game_path
	game_path_button.tooltip_text = game_path_button.text
