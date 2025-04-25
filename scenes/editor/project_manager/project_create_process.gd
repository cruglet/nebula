extends Node

var game_iso: ISO

func create_project(project_name: String, project_path: String, disc: WiiDisc) -> void:
	if disc.game_info.get("data_offset") and disc.disc_path:
		var disc_file: FileAccess = FileAccess.open(disc.disc_path, FileAccess.READ)
		
		WBFS.extract(disc_file, disc.game_info.data_offset)
