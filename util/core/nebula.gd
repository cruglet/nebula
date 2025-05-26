class_name Nebula
extends Node

const VERSION: String = "0.0.0"
const BRANCH: String = "dev"
const GAME_LIST: Dictionary = {
	"SMN#01": {
		"banner": preload("uid://v23ehifiek1i"),
		"editor": "uid://ljkmme41v22c"
	}
}

## Finds the game and returns the regionless variation
## (e.g. SMNE01 -> SMN#01)
static func find_in_game_list(game_id: String) -> String:
	for pattern: String in GAME_LIST:
		var regex_pattern: String = "^" + pattern.replace("#", ".") + "$"
		var regex: RegEx = RegEx.new()
		regex.compile(regex_pattern)
		if regex.search(game_id):
			return pattern
	return ""


## Editor configuration
class Config:
	## This is technically what actually *holds* the data,
	## the class variables just provide an abstract way to
	## access and modify them.
	static var _conf_dict: Dictionary = {}
	static var config_path: String = OS.get_user_data_dir()
	static var save_file_path: String:
		get(): return config_path.path_join("editor.conf")
	static var log_dir_path: String:
		get(): return config_path.path_join("logs/")
	
	
	static func save() -> void:
		var file: FileAccess = FileAccess.open(save_file_path, FileAccess.WRITE)
		file.store_var(_conf_dict, true)
		file.close()
	
	
	static func load() -> void:
		var file: FileAccess = FileAccess.open(save_file_path, FileAccess.READ)
		var stored_conf: Dictionary = file.get_var(true)
		_conf_dict.merge(stored_conf, true)
		print(_conf_dict)
	
	
	static func exists() -> bool:
		return FileAccess.file_exists(save_file_path)
	
	
	class Editor:
		static var scale: int:
			get(): return Config._conf_dict.get(&"editor_scale", 1)
			set(s): Config._conf_dict.set(&"editor_scale", s)
		static var projects: PackedStringArray:
			get(): return Config._conf_dict.get(&"editor_projects", [])
			set(p): Config._conf_dict.set(&"editor_projects", p)
		static var default_project_path: String:
			get(): return Config._conf_dict.get(&"editor_default_project_path", Config.config_path.path_join("projects/"))
			set(dpp): Config._conf_dict.set(&"editor_default_project_path", dpp)
		static var default_game_path: String:
			get(): return Config._conf_dict.get(&"editor_default_game_path", Config.config_path.path_join("extracted_games/"))
			set(dgp): Config._conf_dict.set(&"editor_default_game_path", dgp)
		
		
		static func add_project(path: String) -> void:
			var project_list: PackedStringArray = projects
			project_list.append(path)
			projects = project_list
		
		
		static func remove_project(index: int) -> void:
			var project_list: PackedStringArray = projects
			project_list.remove_at(index)
			projects = project_list
	
	
	class Debug:
		static var max_logs: int:
			get(): return Config._conf_dict.get(&"debug_max_logs", 5)
			set(ml): Config._conf_dict.set(&"debug_max_logs", ml)


## Project data
class Project extends Resource:
	static var _proj_dict: Dictionary = {}
	# Primary stuff
	var name: String:
		get(): return _proj_dict.get(&"project_name", "")
		set(s): _proj_dict.set(&"project_name", s)
	var path: String:
		get(): return _proj_dict.get(&"project_path", "")
		set(p): _proj_dict.set(&"project_path", p)
	var type: String:
		get(): return _proj_dict.get(&"project_type", "")
		set(t): _proj_dict.set(&"project_type", t)
	# Editor specific data
	var editor_data: EditorData:
		get(): return _proj_dict.get(&"editor_data", EditorData.new())
		set(ed): _proj_dict.set(&"editor_data", ed)
	# Metadata
	var last_version: String:
		get(): return _proj_dict.get(&"metadata_last_version", "")
		set(lv): _proj_dict.set(&"metadata_last_version", lv)
	var last_opened: String:
		get(): return _proj_dict.get(&"metadata_last_opened", Singleton.time)
		set(lo): _proj_dict.set(&"metadata_last_opened", lo)
	
	
	static func save(project: Project) -> void:
		var file: FileAccess = FileAccess.open(project.path.path_join("project.nebula"), FileAccess.WRITE)
		file.store_var(_proj_dict, true)
	
	
	static func load(path: String) -> Project:
		var project: Project = Project.new()
		var file: FileAccess = FileAccess.open(path, FileAccess.READ)
		var file_data: Variant = file.get_var(true)
		if file_data is Dictionary:
			project._proj_dict.merge(file_data, true)
		return project
