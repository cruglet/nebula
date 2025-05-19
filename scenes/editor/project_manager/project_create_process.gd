extends Node

var game_iso: ISO

func create_project(project_name: String, project_path: String, disc: WiiDisc) -> void:
	pass
	#if disc.game_info.get("data_offset") and disc.disc_path:
		#var disc_file: FileAccess = FileAccess.open(disc.disc_path, FileAccess.READ)
		#
		#var wbfs_file: WBFS = WBFS.open(disc_file)
		#
		#var iso: ISO = ISO.new()
		#iso.parse_wbfs(wbfs_file)
