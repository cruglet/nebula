class_name EngineConfig extends Object

static var config_path: String = OS.get_user_data_dir()

static var save_file_path: String:
	get(): return config_path.path_join("engine.conf")
static var log_dir_path: String:
	get(): return config_path.path_join("logs/")

static var config: Dictionary = {
	"engine": {
		"scale": 1.0,
		"projects": []
	},
	"debug": {
		"max_logs": 20
	}
}

static var scale: float:
	get(): return config.engine.scale
	set(val): 
		config.engine.scale = val
		Singleton.scale_changed.emit()
static var projects: Array:
	get(): return config.engine.projects
	set(val): config.engine.projects = val
static var max_logs: int:
	get(): return config.debug.max_logs
	set(val): config.debug.max_logs = val

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
