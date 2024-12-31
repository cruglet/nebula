extends Control

@onready var load_rom_dialog: FileDialog = $LoadROMDialog
@onready var select_dir_dialog: FileDialog = $SelectDirDialog

@onready var proj_dir_path: String = OS.get_config_dir() + "/nebula/projects/"

@onready var animation_player: AnimationPlayer = $AnimationPlayer
@onready var setup_project_panel: Panel = $SetupProject
@onready var preview: Control = $SetupProject/CreateProjectContainer/Preview

@onready var proj_name_input: LineEdit = %ProjNameInput
@onready var proj_dir: LineEdit = %ProjDir
@onready var variants_dropdown: OptionButton = %VariantsDropdown

var is_loading: bool = false

func _input(_event: InputEvent) -> void:
	if Input.is_physical_key_pressed(KEY_ESCAPE) and setup_project_panel.visible and !is_loading:
		animation_player.play("setup_project_fade_out")



func _on_create_project_button_pressed() -> void:
	load_rom_dialog.visible = true

	# An error occured with the popup, resorting to default
	if !load_rom_dialog.visible:
		load_rom_dialog.use_native_dialog = false
		load_rom_dialog.visible = true

func _on_file_selected(path: String) -> void:
	ProjectManager.project.rom_path = path
	validate_file()

func validate_file() -> void:
	var sp: Subprocess = Subprocess.new()
	sp.run_threaded("dependencies/wit", ["D", ProjectManager.project.rom_path])
	sp.bind_filter(_check_for_game)
	sp.binded_success.connect(func(_i: String)->void: 
		sp.clean_thread()
		call_deferred(&"create_project_popup")
	)
	sp.start()

func create_project_popup() -> void:
	# This is a fix, dunno why but it breaks position in the MainMenu scene for whatever reason
	setup_project_panel.set_anchors_preset(Control.PRESET_CENTER)
	
	var preview_image: TextureRect = preview._image
	var banner: String = Singleton.gamelist["engine"][ProjectManager.project.game_info.engine_id]["banner_path"]
	preview_image.texture = load(banner)
	
	proj_dir.text = proj_dir_path
	preview._dir.text = proj_dir.text
	
	_populate_variants()
	
	animation_player.play("setup_project_fade_in")

func _check_for_game(line: String) -> bool:
	if line.contains("disc="):
		var disc_id: String = line.split("disc=")[1].split(",")[0]
		
		if disc_id in Singleton.gamelist["games"]:
			ProjectManager.project["game_info"] = Singleton.gamelist["games"][disc_id]
			ProjectManager.project["game_info"]["disc_id"] = disc_id
			print("Loaded project info!\n" + str(ProjectManager.project["game_info"]))
			return true
		
	return false


func _on_project_name_changed(new_text: String) -> void:
	if !new_text:
		proj_name_input.placeholder_text = "My Project"
	else:
		proj_name_input.placeholder_text = new_text
	
	preview._name.text = proj_name_input.placeholder_text
	proj_dir.text = proj_dir_path + proj_name_input.placeholder_text.to_snake_case().replace("_","-")
	proj_dir.tooltip_text = proj_dir.text
	preview._dir.text = proj_dir.text

func _on_select_dir_button_pressed() -> void:
	$SelectDirDialog.visible = true
	
	if !$SelectDirDialog.visible:
		$SelectDirDialog.use_native_dialog = false
		$SelectDirDialog.visible = true
		
func _on_dir_selected(dir: String) -> void:
	proj_dir_path = dir + "/"
	_on_project_name_changed(proj_name_input.placeholder_text)

func _populate_variants() -> void:
	if variants_dropdown.item_count == 0:
		var variants: Array = Singleton.gamelist["engine"][ProjectManager.project.game_info.engine_id]["variants"]
		for variant: String in variants:
			variants_dropdown.add_item(variant)

func _on_real_create_button_pressed() -> void:
	animation_player.play("start_to_load")

func _start_loading() -> void:
	is_loading = true
	# TODO: Implement rest of (actually, ALL of) loading here
