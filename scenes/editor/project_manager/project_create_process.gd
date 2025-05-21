extends Node

var game_iso: ISO

func create_project(project_name: String, project_path: String, disc: WiiDisc) -> void:
	EventBus.project_ready.connect.call_deferred(switch_to_editor.bind(disc.game_id))
	# Create project dir
	if !DirAccess.dir_exists_absolute(project_path):
		var err: Error = DirAccess.make_dir_recursive_absolute(project_path)
		if err != OK:
			Singleton.error.emit("Error creating project directory!\nError code: %s" % err)
			return
	# Extract game data
	var extracted_game_root: String = NebulaConfig.default_game_path.path_join(Nebula.find_in_game_list(disc.game_id))
	if !DirAccess.dir_exists_absolute(extracted_game_root):
		var err: Error = DirAccess.make_dir_recursive_absolute(project_path)
		if err != OK:
			Singleton.error.emit("Error creating project directory!\nError code: %s" % err)
			return
		disc.extract(extracted_game_root)
	else:
		print("Extracted game already exists, skipping...")
		EventBus.project_ready.emit.call_deferred()

func switch_to_editor(engine_id: String) -> void:
	get_tree().change_scene_to_file(Nebula.GAME_LIST.get(Nebula.find_in_game_list(engine_id)).editor)
