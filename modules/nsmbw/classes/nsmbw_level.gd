class_name NSMBWLevel extends Node

@export var areas: Array[NSMBWArea]
var archive_path: String

static func open(level_archive: ARC) -> NSMBWLevel:
	var base_regex: RegEx = RegEx.new()
	var layer_regex: RegEx = RegEx.new()
	var layer_trim: RegEx = RegEx.new()
	base_regex.compile("^course\\d+\\.bin$")
	layer_regex.compile("^course\\d+\\_bgdatL\\d+\\.bin$")
	layer_trim.compile("_bgdatL\\d+\\.bin$")
	
	var level_data: Dictionary
	
	if level_archive.is_valid() and level_archive.filesystem.has("course"):
		var course_dir: Dictionary = level_archive.filesystem["course"]
		for filename: String in course_dir.keys():
			if base_regex.search(filename):
				var base_name: String = filename.trim_suffix(".bin")
				if not level_data.has(base_name):
					level_data[base_name] = {}
				var file_data: PackedByteArray = course_dir[filename]
				var base_entry: Dictionary = level_data[base_name]
				base_entry["data"] = file_data
				level_data[base_name] = base_entry
			elif layer_regex.search(filename):
				var base_name: String = layer_trim.sub(filename, "", true)
				var layer_part: String = filename.get_slice("_bgdatL", 1)
				var layer_num: String = layer_part.trim_suffix(".bin")
				var layer_key: String = "layer" + layer_num
				if not level_data.has(base_name):
					level_data[base_name] = {}
				var file_data: PackedByteArray = course_dir[filename]
				var base_entry: Dictionary = level_data[base_name]
				base_entry[layer_key] = file_data
				level_data[base_name] = base_entry
	
	var level: NSMBWLevel = NSMBWLevel.new()
	level.archive_path = level_archive.path
	
	for area: String in level_data:
		level.areas.append(NSMBWArea.parse(level_data[area]))
	
	return level
