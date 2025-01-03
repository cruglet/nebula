extends Control

var sp: Subprocess = Subprocess.new()
var is_loading: bool = false

@onready var load_rom_dialog: FileDialog = $"../../LoadROMDialog"
@onready var select_dir_dialog: FileDialog = $"../../SelectDirDialog"
@onready var proj_dir_path: String = OS.get_config_dir() + "/nebula/projects/"
@onready var preview: Control = $Preview

@onready var proj_name_input: LineEdit = %ProjNameInput
@onready var proj_dir: LineEdit = %ProjDir
@onready var variants_dropdown: OptionButton = %VariantsDropdown

@onready var create_project_tip: Label = $CreateProjectTip
@onready var create_button: Button = $CreateButton



@onready var animation_player: AnimationPlayer = $"../../AnimationPlayer"

func start_project_creation() -> void:
	load_rom_dialog.visible = true



func _on_file_selected(path: String) -> void:
	ProjectManager.project.rom_path = path
	validate_file()

func validate_file() -> void:
	sp.run_threaded("dependencies/wit", ["D", ProjectManager.project.rom_path])
	sp.bind_filter(_check_for_game)
	sp.binded_success.connect(func(_i: String)->void: 
		sp.clean_thread()
		call_deferred(&"create_project_popup")
	)
	sp.start()

func create_project_popup() -> void:
	var preview_image: TextureRect = preview.preview_image
	var banner: String = Singleton.gamelist["engine"][ProjectManager.project.game_info.engine_id]["banner_path"]
	preview_image.texture = load(banner)
	_on_project_name_changed("")
	_populate_variants()
	
	animation_player.play("create_project_fade_in")

func _check_for_game(line: String) -> bool:
	if line.contains("disc="):
		var disc_id: String = line.split("disc=")[1].split(",")[0]
		
		if disc_id in Singleton.gamelist["games"]:
			ProjectManager.project["game_info"] = Singleton.gamelist["games"][disc_id]
			ProjectManager.project["game_info"]["disc_id"] = disc_id
			print("Loaded game info!\n" + str(ProjectManager.project["game_info"]))
			return true
		
	return false

func _on_project_name_changed(new_text: String) -> void:
	if !new_text:
		proj_name_input.placeholder_text = "My Project"
	else:
		proj_name_input.placeholder_text = new_text
	
	preview.preview_name.text = proj_name_input.placeholder_text
	proj_dir.text = proj_dir_path + proj_name_input.placeholder_text.to_snake_case().replace("_","-")
	proj_dir.tooltip_text = proj_dir.text
	preview.preview_dir.text = proj_dir.text
	
	if FileAccess.file_exists(proj_dir.text + "/project.nebula"):
		create_button.disabled = true
		create_project_tip.text = "Project already exists"
	else:
		create_button.disabled = false
		create_project_tip.text = ""

func _on_select_dir_button_pressed() -> void:
	select_dir_dialog.visible = true
		
func _on_dir_selected(dir: String) -> void:
	proj_dir_path = dir + "/"
	_on_project_name_changed(proj_name_input.placeholder_text)

func _populate_variants() -> void:
	if variants_dropdown.item_count == 0:
		var variants: Array = Singleton.gamelist["engine"][ProjectManager.project.game_info.engine_id]["variants"]
		for variant: String in variants:
			variants_dropdown.add_item(variant)

func _on_real_create_button_pressed() -> void:
	ProjectManager.project.project_path = preview.preview_dir.text
	animation_player.play(&"start_to_load_from_create")

func _start_loading() -> void:
	is_loading = true
	animation_player.play(&"loading_loop")
	var thread: Thread = Thread.new()
	thread.start(_extract_filesystem)

func _extract_filesystem() -> void:
	sp.run("dependencies/wit", ["X", ProjectManager.project.rom_path, ProjectManager.project.project_path])
	call_deferred(&"prep_editor")

func prep_editor() -> void:
	_clean_files()
	_setup_project_filesystem()
	_generate_project_json()
	_load_editor()

func _clean_files() -> void:
	var proj_path: String = ProjectManager.project.project_path
	var extracted_dir: PackedStringArray = Array(DirAccess.get_directories_at(proj_path))
	
	# Clean Remaining Dirs
	for dir: String in Array(extracted_dir):
		if dir != "files":
			for file: String in Array(DirAccess.get_files_at(proj_path + "/" + dir)):
				DirAccess.remove_absolute("%s/%s/%s" % [proj_path, dir, file])
			DirAccess.remove_absolute(proj_path + "/" + dir)
		else:
			DirAccess.rename_absolute(proj_path + "/" + dir, proj_path + "/" + "game")
	
	# Remove extra root files
	for file: String in Array(DirAccess.get_files_at(proj_path)):
		DirAccess.remove_absolute(proj_path + "/" + file)

func _setup_project_filesystem() -> void:
	var proj_path: String = ProjectManager.project.project_path + "/"
	DirAccess.make_dir_absolute(proj_path + "code")
	DirAccess.make_dir_absolute(proj_path + "other")

func _generate_project_json() -> void:
	# Project JSON file for engine recognition and validation
	var project_file_location: String = ProjectManager.project.project_path + "/project.nebula"
	var proj_path: String = ProjectManager.project.project_path + "/"
	ProjectManager.project["project_name"] = preview.preview_name.text
	var proj_file: FileAccess = FileAccess.open(project_file_location, FileAccess.WRITE)
	proj_file.store_var(ProjectManager.project)
	proj_file.close()
	
	# Add to list of projects
	var project_list: FileAccess = FileAccess.open("res://projectlist.json", FileAccess.READ)
	var projects: Array 
	
	if (project_list.get_length() > 4):
		projects = JSON.parse_string("" + project_list.get_as_text())

	if !project_file_location in projects:
		projects.append(project_file_location)
	project_list = FileAccess.open("res://projectlist.json", FileAccess.WRITE)
	project_list.store_string(str(projects))
	project_list.close()

func _load_editor() -> void:
	Singleton.change_scene("res://engines/nsmbw/editor/editor.tscn")
