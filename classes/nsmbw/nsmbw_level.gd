extends Node
class_name NSMBWLevel

var sp: Subprocess = Subprocess.new()

var dump_success: bool = false

func dump_level(level_name: String, from: String, to: String) -> void:
	sp.run_threaded("neb-utils", ["nsmbw", "--dump", (from + level_name), (to + level_name)])
	sp.bind_filter(_dump_check)
	sp.binded_success.connect(_on_dump_success)
	sp.start()
	
func _dump_check(line: String) -> bool:
	if line.to_lower().contains("success"):
		dump_success = true
	dump_success = false
	return dump_success
	
func _on_dump_success() -> void:
	call_deferred("_post_dump")

func _post_dump() -> void:
	print(dump_success)
