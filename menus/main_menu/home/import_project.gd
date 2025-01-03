extends VBoxContainer

@onready var animation_player: AnimationPlayer = $"../../AnimationPlayer"
@onready var projects_scroll_container: ScrollContainer = $"../../Projects/ProjectsScrollContainer"
@onready var projects_grid_container: GridContainer = $"../../Projects/ProjectsScrollContainer/ProjectsContainer/ProjectsGridContainer"

@onready var no_projects_container: Control = $"../../NoProjects"
@onready var projects_container: Control = $"../../Projects"

var project_path: String
# Called when the node enters the scene tree for the first time.
func show_delete_panel(project_scene: Control) -> void:
	var preview: Control = load("res://resources/components/ui/project_preview/project_preview.tscn").instantiate()
	preview._name = project_scene._name
	preview._dir = project_scene._dir
	preview._image = project_scene._image
	preview.real = false
	animation_player.play(&"delete_project_fade_in")
	var tmp_preview: Control = find_child("Preview")
	if tmp_preview.get_child(0):
		tmp_preview.get_child(0).free()
	tmp_preview.replace_by(preview)
	tmp_preview.free()


func _remove_project_from_list(project_path: String) -> void:
	var projectlist: FileAccess = FileAccess.open("res://projectlist.json", FileAccess.READ)
	var project_list: Array = JSON.parse_string(projectlist.get_as_text())
	project_list.remove_at(project_list.find(project_path))
	projects_scroll_container.load_projects(project_list)
	
	if projects_grid_container.get_child_count() == 0:
		no_projects_container.visible = true
		projects_container.visible = false

func _on_hide_proj_button_pressed() -> void:
	var preview: Control = find_child("Preview")
	project_path = preview._dir + "/project.nebula"
	_remove_project_from_list(project_path)
	animation_player.play(&"fade_out")

func _on_delete_proj_button_pressed() -> void:
	var preview: Control = find_child("Preview")
	project_path = preview._dir + "/project.nebula"
	
	# First remove from list
	_remove_project_from_list(project_path)
	animation_player.play(&"start_to_load_from_delete")


func start_deletion() -> void:
	animation_player.play(&"loading_loop")
	var thread: Thread = Thread.new()
	thread.start(delete_directory_recursive.bind(project_path))

func delete_directory_recursive(dir_path: String) -> void:
	var path: String = dir_path.replace("/project.nebula", "")
	var dir: DirAccess = DirAccess.open(path)
	if dir:
		# First, open the directory and look at all files/folders within
		dir.list_dir_begin()
		var file_name: String = dir.get_next()
		
		while file_name != "":
			if file_name == "." or file_name == "..":
				file_name = dir.get_next()
				continue
				
			var full_path: String = path.path_join(file_name)
			
			if dir.current_is_dir():
				# Recursively delete subdirectories
				delete_directory_recursive(full_path)
			else:
				# Delete files
				dir.remove(file_name)
			file_name = dir.get_next()
			
		dir.list_dir_end()
		
		# Now delete this directory itself
		var parent_dir: DirAccess = DirAccess.open(path.get_base_dir())
		if parent_dir:
			# Get the name of the current directory
			var dir_name: String = path.get_file()
			if dir_name != "":
				parent_dir.remove(dir_name)
	call_deferred("finished_deleting")

func finished_deleting() -> void:
	animation_player.play(&"fade_out")
