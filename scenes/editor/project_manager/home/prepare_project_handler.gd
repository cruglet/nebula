extends Node

@export var open_game_dialog: FileDialog
@export var prepare_project_window: Window


func _on_create_button_pressed() -> void:
	open_game_dialog.show()


func _on_open_game_dialog_file_selected(path: String) -> void:
	var opened_disc: WiiDisc = WiiDisc.open(path)
	
	if Nebula.find_in_game_list(opened_disc.game_id):
		Singleton.opened_disc = opened_disc
		Singleton.apply_scale(prepare_project_window)
		prepare_project_window.show()
	elif opened_disc.game_id:
		Singleton.toast_notification(
			"Unsupported game", 
			"This game is currently unsupported by Nebula, please check the latest version."
		)


func _on_prepare_project_window_close_requested() -> void:
	prepare_project_window.hide()
