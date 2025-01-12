extends Node
class_name NSMBWLevel

var _sp: Subprocess = Subprocess.new()

var _dump_success: bool = false

var level: Array = []
var dumped_level_path: String
signal dump_finished

func dump_level(level_name: String, from: String, to: String) -> void:
	dumped_level_path = "%s%s.lvl" % [to, level_name]
	_sp.run_threaded("dependencies/neb-utils", [
		"nsmbw",
		"--dump", 
		from + level_name + ".arc", 
		dumped_level_path,
		])
	_sp.bind_filter(_dump_check)
	_sp.start()
	
func _dump_check(line: String) -> Variant:
	if line.to_lower().contains("success"):
		_dump_success = true
		call_deferred("_on_dump_success")
		return true
	return false
	
func _on_dump_success() -> void:
	dump_finished.emit()
	level = FileAccess.open(dumped_level_path, FileAccess.READ).get_var(true)
	# TODO: the course is stored in "level" now. Do something with that later
