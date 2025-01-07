extends ScrollContainer

const PROJECT_PREVIEW: PackedScene = preload("res://resources/components/ui/project_preview/project_preview.tscn")

@onready var grid_container: GridContainer = $ProjectsContainer/ProjectsGridContainer
@onready var project_num: Label = $"../ProjectNum"
@onready var fetching_text: Label = $"../FetchingText"

@onready var animation_player: AnimationPlayer = $"../../AnimationPlayer"

@onready var delete_project_container: VBoxContainer = $"../../SetupProject/DeleteProjectContainer"

func _ready() -> void:
	get_tree().root.size_changed.connect(_update_grid)

func load_projects(projects: Array) -> bool:
	
	for child: Control in grid_container.get_children():
		child.free()
	
	var project_list_file: FileAccess = FileAccess.open("res://projectlist.json", FileAccess.WRITE)
	var i: int = 0
	for project: String in projects:
		if !_add_project_to_grid(project):
			projects.remove_at(i)
			i -= 1
			continue
		i += 1
	
	fetching_text.visible = false
	if projects.size() > 0:
		project_num.text = "Projects - " + str(projects.size())
		print_debug("Changing Projectlist JSON file...")
		project_list_file.store_string(str(projects))
		return true
	else:
		return false

func _read_project_data(path: String) -> Dictionary:
	var file: FileAccess = FileAccess.open(path, FileAccess.READ)
	if !file:
		return {}
	return file.get_var(true)

func _update_grid() -> void:
	var window_size: Vector2i = DisplayServer.window_get_size()
	if (window_size.x < 1350 * Singleton.scale_factor):
		grid_container.columns = 1
	else: 
		grid_container.columns = (window_size.x / (650 * Singleton.scale_factor))

func add_project(proj_name: String, proj_path: String, proj_banner: String) -> void:
	var project_preview: PackedScene = PROJECT_PREVIEW.duplicate(true)
	var project_scene: Control = project_preview.instantiate(PackedScene.GEN_EDIT_STATE_INSTANCE)
	
	project_scene.custom_minimum_size = Vector2(600, 65)
	project_scene.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	
	project_scene._name = proj_name
	project_scene._dir = proj_path
	project_scene._image = proj_banner
	project_scene.real = true
	
	project_scene.delete_project_pressed.connect(delete_project_container.show_delete_panel)
	
	grid_container.add_child(project_scene)

func _add_project_to_grid(project: String) -> bool:
	var project_data: Dictionary = _read_project_data(project)
	if !project_data:
		return false

	add_project(
		project_data["project_name"],
		project_data["project_path"],
		Singleton.gamelist.engine[project_data.game_info.engine_id].banner_path
	)
	return true



func delete_project(project_scene: Control) -> void:
	print(project_scene.preview_dir.text)
