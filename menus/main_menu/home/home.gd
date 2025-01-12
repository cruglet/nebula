extends Control

@onready var no_projects_container: Control = $NoProjects

@onready var projects_container: Control = $Projects
@onready var projects_scroll_container: ScrollContainer = $Projects/ProjectsScrollContainer
@onready var projects_grid_container: GridContainer = $Projects/ProjectsScrollContainer/ProjectsContainer/ProjectsGridContainer
@onready var project_num: Label = $Projects/ProjectNum

@onready var setup_project_panel: Panel = $SetupProject
@onready var animation_player: AnimationPlayer = $AnimationPlayer
@onready var load_project_dialog: FileDialog = $LoadProjectDialog


func _debug() -> void:
	no_projects_container.visible = false
	projects_container.visible = false
	
	var lvl: NSMBWLevel = NSMBWLevel.new()
	lvl.dump_level("01-02", "test/", "test/")

func _ready() -> void:
	# Determine if there are projects available or not.
	_debug()
	var projectlist_file: FileAccess = FileAccess.open("res://projectlist.json", FileAccess.READ)
	var projects_string: Variant = projectlist_file.get_as_text()
	var projects: Array
	if (projects_string != "null" and projects_string != ""): 
		projects = JSON.parse_string(projects_string)

	projects_container.visible = projects_scroll_container.load_projects(projects)
	no_projects_container.visible = !projects_container.visible
	
	Singleton.scale_factor = 1.25



func _input(_event: InputEvent) -> void:
	if Input.is_physical_key_pressed(KEY_ESCAPE) and setup_project_panel.visible:
		animation_player.play(&"fade_out")

func _on_create_project_button_pressed() -> void:
	$SetupProject/CreateProjectContainer.start_project_creation()

func _on_import_project_button_pressed() -> void:
	load_project_dialog.visible = true

func _on_import_project_file_selected(path: String) -> void:
	var file: FileAccess = FileAccess.open(path, FileAccess.READ)
	var projectlist_file: String = FileAccess.open("res://projectlist.json", FileAccess.READ).get_as_text()
	var project_list: Array
	
	if (projectlist_file == "null" or projectlist_file == ""):
		project_list = []
	else:
		project_list = JSON.parse_string(projectlist_file)
	
	var project: Dictionary = file.get_var(true)
	var banner_path: String
	if (!project.is_empty() and (project.project_path + "/project.nebula") not in project_list):
		banner_path = Singleton.gamelist.engine[project.game_info.engine_id].banner_path
		projects_scroll_container.add_project(project.project_name, project.project_path, banner_path)
		project_num.text = "Projects - " + str(projects_grid_container.get_child_count())
		project_list.append(project.project_path + "/project.nebula")
		var projectlist_write_file: FileAccess = FileAccess.open("res://projectlist.json", FileAccess.WRITE)
		projectlist_write_file.store_string(str(project_list))
		projectlist_write_file.close()
		
		if no_projects_container.visible:
			no_projects_container.visible = false
			projects_container.visible = true
