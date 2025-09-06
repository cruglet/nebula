extends Panel

signal updates_available
signal updates_cleared

const INTERNAL_MODULES: Array[String] = ["https://github.com/cruglet/nebula.git"]
const MODULE_ITEM: PackedScene = preload("uid://bs1txkbdc5lg7")
const FETCH_TEXT_SPEED: float = 0.5

@export var fetching_label: Label
@export var installed_label: Label
@export var available_label: Label
@export var local_flow_container: FlowContainer
@export var online_flow_container: FlowContainer
@export var open_module_file_dialog: FileDialog

@export_group("Module Info")
@export var module_info_window: NebulaWindow
@export var module_info_banner: TextureRect
@export var module_info_name_label: Label
@export var module_info_description_label: Label
@export var module_info_authors_label: Label
@export var module_info_version_label: RichTextLabel
@export var module_info_id_label: Label
@export var module_info_size_label: RichTextLabel


var offline: bool = false
var loading_text_anim_timer: Timer
var _fetch_text_iter: int = 0
var updates_pending: int = 0:
	set(up):
		if up > 0 and not updates_available_notified:
			updates_available_notified = true
			updates_available.emit()
		else:
			updates_available_notified = false
			updates_cleared.emit()
		updates_pending = up

var local_loaded_modules: Dictionary[String, Variant] = {}
var online_loaded_modules: Dictionary[String, Variant] = {}
var online_loaded_images: Array[String] = []
var updates_available_notified: bool = false


func _ready() -> void:
	load_local_modules()
	
	loading_text_anim_timer = Timer.new()
	add_child(loading_text_anim_timer)
	loading_text_anim_timer.timeout.connect(_on_fetching_text_timeout)
	loading_text_anim_timer.start(FETCH_TEXT_SPEED)
	
	var module_request: ModuleRequest = ModuleRequest.new()
	module_request.metadata_fetched.connect(_on_module_metadata_fetched)
	module_request.preview_image_fetched.connect(_on_module_preview_image_fetched)
	module_request.could_not_connect.connect(_on_could_not_connect)
	module_request.fetch_parallel(INTERNAL_MODULES)


func update_module_count() -> void:
	installed_label.text = "Installed: %s" % QuickActions.get_visible_children(local_flow_container).size()
	available_label.text = "Available: %s" % QuickActions.get_visible_children(online_flow_container).size()


func load_local_modules() -> void:
	for module_path: String in CoreSettings.get(CoreSettings.SETTING_MODULE_LIST):
		_load_local_module(module_path)


func show_module_info(module_item: ModuleItem) -> void:
	module_info_name_label.text = module_item.module_name
	module_info_description_label.text = module_item.module_description
	module_info_authors_label.text = ", ".join(module_item.module_authors)
	module_info_version_label.text = "v" + module_item.module_version
	module_info_id_label.text = module_item.module_id
	module_info_size_label.text = String.humanize_size(module_item.module_size)
	module_info_banner.texture = module_item.module_preview_texture
	
	if module_item.update_available:
		module_info_version_label.text += "[color=#47ff60] -> v" + module_item.update_version
		
		var update_text_col: String
		
		if module_item.update_size < module_item.module_size:
			update_text_col = "#47ff60"
		else:
			update_text_col = "#ff4d4d"
		
		module_info_size_label.text += "[color=%s] -> %s" % [update_text_col, String.humanize_size(module_item.update_size)]
	
	module_info_window.show()


func hide_module_info() -> void:
	module_info_window.hide()


func _load_local_module(module_path: String) -> void:
	var module: Module = Module.load(module_path)
	
	if module.id in local_loaded_modules:
		return
	
	var module_item: ModuleItem = ModuleItem.from_module(module)
	var module_file: FileAccess = FileAccess.open(module_path, FileAccess.READ)
	module_item.module_size = module_file.get_length()
	module_item.module_preview_texture = QuickActions.load_image_with_fallback(module.project_image, ProjectItem.FALLBACK_IMAGE)
	module_item.is_local = true
	module_item.module_file_path = module_path
	module_item.updated.connect(func(_id: String) -> void:
		updates_pending -= 1
	)
	module_item.more_info_request.connect(_on_module_item_more_info_request)
	module_file.close()
	local_loaded_modules.set(module.id, module_item)
	local_flow_container.add_child(module_item)
	
	if module.id in online_loaded_modules:
		var online_module_item: ModuleItem = online_loaded_modules.get(module.id)
		online_loaded_modules.erase(module.id)
		
		if Nebula.compare_versions(online_module_item.get_module_version_string(), module.get_version_string()) == 1:
			module_item.update_version = online_module_item.get_module_version_string()
			module_item.set_update_available(
				online_module_item.module_source, 
				online_module_item.module_size, 
				online_module_item.module_version
			)
			online_module_item.free()
	
	CoreSettings.append(CoreSettings.SETTING_MODULE_LIST, module_path)
	update_module_count()


func _on_fetching_text_timeout() -> void:
	if offline:
		return
	_fetch_text_iter = wrapi(_fetch_text_iter + 1, 0, 4)
	fetching_label.text = "Fetching available modules online" + ".".repeat(_fetch_text_iter)


func _on_module_metadata_fetched(data: Dictionary, source_url: String, module_size: int) -> void:
	if online_loaded_modules.has(data.id):
		return
	if local_loaded_modules.has(data.id):
		var local_module: ModuleItem = local_loaded_modules.get(data.id)
		var fetched_version: String = "%s.%s.%s" % [data.major_version, data.minor_version, data.patch_number]
		if Nebula.compare_versions(fetched_version, local_module.get_module_version_string()) == 1:
			local_module.set_update_available(source_url, module_size, fetched_version)
			updates_pending += 1
		return
	
	var module_item: ModuleItem = ModuleItem.from_dict(data)
	module_item.module_source = source_url
	module_item.module_size = module_size
	module_item.installed_to_local.connect(_on_module_downloaded)
	module_item.more_info_request.connect(_on_module_item_more_info_request)
	online_loaded_modules.set(data.id, module_item)
	online_flow_container.add_child(module_item)
	update_module_count()


func _on_module_preview_image_fetched(img_data: PackedByteArray, module_id: String, img_type: String) -> void:
	if module_id in online_loaded_images:
		return
	
	var img: Image = Image.new()
	match img_type:
		"svg": img.load_svg_from_buffer(img_data)
	
	if online_loaded_modules.has(module_id):
		online_loaded_modules.get(module_id).module_preview_texture = ImageTexture.create_from_image(img)
		online_loaded_images.append(module_id)


func _on_module_item_more_info_request(instance: ModuleItem) -> void:
	show_module_info(instance)


func _on_module_downloaded(id: String, path: String) -> void:
	var module_item: ModuleItem = online_loaded_modules.get(id)
	module_item.reparent(local_flow_container)
	local_loaded_modules.set(id, module_item)
	online_loaded_modules.erase(id)
	update_module_count()
	
	fetching_label.hide()
	
	var module_list: Array = CoreSettings.get(CoreSettings.SETTING_MODULE_LIST)
	module_list.append(path)
	CoreSettings.set(CoreSettings.SETTING_MODULE_LIST, module_list)
	Singleton.register_module(Module.load(path))


func _on_could_not_connect() -> void:
	offline = true
	fetching_label.text = "Offline Mode"


func _on_import_local_button_pressed() -> void:
	open_module_file_dialog.show()


func _on_open_module_file_dialog_file_selected(path: String) -> void:
	if path in CoreSettings.get(CoreSettings.SETTING_MODULE_LIST):
		return
	
	var module: Module = Module.load(path)
	
	if module and not (module.id in Singleton.get_module_ids()):
		Singleton.register_module(module)
		_load_local_module(path)


func _on_search_line_edit_text_changed(new_text: String) -> void:
	for module_item_container: Container in [local_flow_container, online_flow_container]:
		for child: ModuleItem in module_item_container.get_children():
			if new_text.is_empty() or child.matches(new_text):
				child.show()
			else:
				child.hide()
	
	update_module_count()


func _on_more_info_close_button_pressed() -> void:
	module_info_window.hide()
