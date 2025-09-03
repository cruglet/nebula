extends Panel

const PROJECT_ITEM: PackedScene = preload("uid://dw0t4n57ys8d")

signal switch_screen_request(screen: int)

@export var no_projects: Control
@export var projects: Control
@export var project_count_label: Label
@export var project_list_vbox: VBoxContainer
@export var blur_overlay: ColorRect
@export var new_project_handler: PanelContainer
@export var new_project_window: NebulaWindow
@export var loading_window: NebulaWindow
@export var remove_project_window: NebulaWindow
@export var delete_contents_checkbox: CheckBox

var current_project_item: ProjectItem
var project_item_map: Dictionary[String, ProjectItem]
var creating_project: bool = false

func _ready() -> void:
	check_projects_exist()
	
	var project_list: Array = CoreSettings.get(CoreSettings.SETTING_PROJECT_LIST)
	
	if project_list.is_empty():
		no_projects.show()
		projects.hide()
	else:
		no_projects.hide()
		projects.show()
		refresh_project_list()
		update_project_count()


func _notification(what: int) -> void:
	if what == NOTIFICATION_WM_WINDOW_FOCUS_IN:
		check_projects_exist()
		refresh_project_list()
		check_for_no_projects()


func show_blur() -> void:
	blur_overlay.material.set_shader_parameter(&"blur_amount", 0.0)
	blur_overlay.show()
	
	var tween: Tween = get_tree().create_tween()
	tween.tween_property(blur_overlay.material, ^"shader_parameter/blur_amount", 2.5, 0.25)


func hide_blur() -> void:
	blur_overlay.material.set_shader_parameter(&"blur_amount", 2.5)
	
	var tween: Tween = get_tree().create_tween()
	tween.tween_property(blur_overlay.material, ^"shader_parameter/blur_amount", 0.0, 0.25)
	
	await tween.finished
	blur_overlay.hide()


func check_for_no_projects() -> void:
	var project_list: Array = CoreSettings.get(CoreSettings.SETTING_PROJECT_LIST)
	
	if project_list.is_empty():
		no_projects.show()
		projects.hide()
	else:
		no_projects.hide()
		projects.show()


func check_projects_exist() -> void:
	var project_list: Array = CoreSettings.get(CoreSettings.SETTING_PROJECT_LIST)
	var filtered_list: Array = project_list.filter(func(project_path: String) -> bool:
		return FileAccess.file_exists(project_path)
	)
	
	CoreSettings.set(CoreSettings.SETTING_PROJECT_LIST, filtered_list)


func refresh_project_list() -> void:
	project_item_map.clear()
	for child: Node in project_list_vbox.get_children(): child.queue_free()
	var project_list: Array = CoreSettings.get(CoreSettings.SETTING_PROJECT_LIST)
	for project_path: String in project_list:
		var project_file: FileAccess = FileAccess.open(project_path, FileAccess.READ)
		var project_data: Dictionary = project_file.get_var(true)
		var project_item: ProjectItem = PROJECT_ITEM.duplicate().instantiate()
		project_item.project_name = project_data.get("name", "<Unnamed>")
		project_item.project_path = project_path
		
		var module: Module = Singleton.get_module(project_data.get("module"))
		project_item.project_banner_texture = QuickActions.load_image_with_fallback(module.project_image, "uid://4xxbc7xne4f3")
		
		project_item.open_project_request.connect(_on_open_project_request)
		project_item.remove_project_request.connect(_on_remove_project_request)
		
		project_list_vbox.add_child(project_item)
		project_item_map.set(project_path, project_item)


func update_project_count() -> void:
	var project_list: Array = CoreSettings.get(CoreSettings.SETTING_PROJECT_LIST)
	project_count_label.text = "Projects - %s" % project_list.size()
	
	if project_list.size() == 0:
		pass


func _on_open_project_request(item: ProjectItem) -> void:
	ProjectData.set_path(item.project_path)
	var project_file: FileAccess = FileAccess.open(item.project_path, FileAccess.READ)
	var project_data: Dictionary = project_file.get_var(true)
	var module: Module = Singleton.get_module(project_data.get("module"))
	
	if not module.id:
		push_error("Could not find module!")
		# TODO: Toast invalid module!
		return
	
	get_tree().change_scene_to_file(module.entry_scene)


func _on_remove_project_request(item: ProjectItem) -> void:
	current_project_item = item
	remove_project_window.show()
	show_blur()

func _on_create_button_pressed() -> void:
	release_focus()
	show_blur()
	new_project_window.show()


func _on_nebula_window_hide_request() -> void:
	if not creating_project:
		hide_blur()


func _on_new_project_cancel_pressed() -> void:
	new_project_window.hide()


func _on_new_project_switch_to_module_request() -> void:
	switch_screen_request.emit(1)


func _on_new_project_create_request(path: String, module: Module) -> void:
	creating_project = true
	new_project_window.hide()
	loading_window.show()
	
	if not DirAccess.dir_exists_absolute(path):
		DirAccess.make_dir_recursive_absolute(path)
	
	var project_file_path: String = path.path_join("%s.nproj" % path.get_base_dir().get_file().to_kebab_case())
	
	var new_project_file: FileAccess = FileAccess.open(project_file_path, FileAccess.WRITE)
	new_project_file.store_var({
		"name": new_project_handler.get_project_name(),
		"module": module.id,
	})
	new_project_file.close()
	
	CoreSettings.append(CoreSettings.SETTING_PROJECT_LIST, project_file_path)
	
	ProjectData.set_path(path)
	
	get_tree().change_scene_to_file(module.entry_scene)


func _on_remove_project_window_hide_request() -> void:
	hide_blur()


func _on_remove_project_cancel_button_pressed() -> void:
	remove_project_window.hide()


func _on_remove_project_remove_button_pressed() -> void:
	var project_list: Array = CoreSettings.get(CoreSettings.SETTING_PROJECT_LIST)
	var project_path: String = current_project_item.project_path
	
	var i: int = project_list.find(project_path)
	project_list.pop_at(i)
	
	CoreSettings.set(CoreSettings.SETTING_PROJECT_LIST, project_list)
	current_project_item.queue_free()
	update_project_count()
	check_for_no_projects()
	
	if delete_contents_checkbox.button_pressed:
		QuickActions.delete_folder_recursively(project_path)
	
	remove_project_window.hide()
