class_name NebulaConfig
extends Object

static var config_path: String = OS.get_user_data_dir()

static var save_file_path: String:
	get(): return config_path.path_join("editor.conf")
static var log_dir_path: String:
	get(): return config_path.path_join("logs/")
static var config: Dictionary = {
	"editor": {
		"scale": 1.0,
		"projects": [],
		"default_project_path": "",
		"default_game_path": "",
	},
	"debug": {
		"max_logs": 5
	}
}
static var scale: float:
	get(): return config.editor.scale
	set(val): 
		config.editor.scale = val
		Singleton.scale_changed.emit()
static var projects: Dictionary = {}
static var max_logs: int:
	get(): return config.debug.max_logs
	set(val): config.debug.max_logs = val
static var default_project_path: String: 
	get():
		if !config.editor.get("default_project_path"):
			return config_path.path_join("projects/")
		else:
			return config.editor.default_project_path
	set(new_path):
		config.editor.default_project_path = new_path
static var default_game_path: String:
	get():
		if !config.editor.get("default_game_path"):
			return config_path.path_join("extracted_games/")
		else:
			return config.editor.default_game_path
	set(new_path):
		config.editor.default_game_path = new_path


static func exists() -> bool:
	return FileAccess.file_exists(save_file_path)

static func load() -> void:
	var file: FileAccess = FileAccess.open(save_file_path, FileAccess.READ)
	var stored_conf: Dictionary = file.get_var(true)
	config.merge(stored_conf, true)

static func save() -> void:
	var file: FileAccess = FileAccess.open(save_file_path, FileAccess.WRITE)
	file.store_var(config)
	file.close()
