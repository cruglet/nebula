extends Node

@export var animation_player: AnimationPlayer

var new_project_thread: Thread = Thread.new()


func _enter_tree() -> void:
	get_tree().root.add_user_signal(&"create_project")
	get_tree().root.connect(&"create_project", create_project, ConnectFlags.CONNECT_ONE_SHOT)


func create_project(project_name: String, project_path: String) -> void:
	animation_player.play(&"start_to_load")
	
	new_project_thread.start(_create_project_threaded.bind(project_name, project_path))
	
	animation_player.animation_finished.connect(func(_a: Variant) -> void:
		animation_player.play(&"loading_loop"), CONNECT_ONE_SHOT
	)


func _create_project_threaded(project_name: String, project_path: String) -> void:
	var disc: WiiDisc = Singleton.opened_disc
	
	# Create project dir
	if !DirAccess.dir_exists_absolute(project_path):
		var err: Error = DirAccess.make_dir_recursive_absolute(project_path)
		if err != OK:
			Singleton.toast_notification("Error code %s" % err, "Could not create project directory!")
			return
	var project_file: FileAccess = FileAccess.open(project_path.path_join("project.nebula"), FileAccess.WRITE)
	
	# Extract game data
	var extracted_game_root: String = NebulaConfig.default_game_path.path_join(Nebula.find_in_game_list(disc.game_id))
	if !DirAccess.dir_exists_absolute(extracted_game_root):
		var err: Error = DirAccess.make_dir_recursive_absolute(project_path)
		if err != OK:
			Singleton.toast_notification("Error code %s" % err, "Could not create game directory!")
			return
		disc.extract(extracted_game_root)
	else:
		print("Extracted game already exists, skipping...")
	
	_switch_to_editor.call_deferred(Nebula.find_in_game_list(disc.game_id))


func _switch_to_editor(editor_id: String) -> void:
	new_project_thread.wait_to_finish()
	get_tree().root.remove_user_signal(&"create_project")
	get_tree().change_scene_to_file(Nebula.GAME_LIST.get(editor_id).editor)
