extends Node

@export var project_preview: ProjectItem
@export var info_label: RichTextLabel
@export var create_button: Button


func _ready() -> void:
	_check_conditions()


func _check_conditions() -> void:
	# Check for invalid path name
	if project_preview.project_name != "":
		var regex: RegEx = RegEx.new()
		var pattern: String = "^[^<>:\"/\\\\|?*\\x00-\\x1F]+$"
		var compile_result: int = regex.compile(pattern)
		
		if compile_result != OK:
			push_error("Failed to compile regex.")
		else:
			var dir_name: String = project_preview.project_name.to_kebab_case()
			var regex_match: RegExMatch = regex.search(dir_name)
			if regex_match == null:
				info_label.text = "[color=red]Invalid characters in project path."
				create_button.disabled = true
				return
	
	# Check if directory already exists
	if DirAccess.dir_exists_absolute(project_preview.project_path):
		if DirAccess.get_files_at(project_preview.project_path).size() > 0:
			info_label.text = "[color=red]A directory with files inside already exists with this name."
			create_button.disabled = true
			return
	
	info_label.text = "[color=green]Everything's good to go!"
	create_button.disabled = false
