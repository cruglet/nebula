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
	module_request.fetch_parallel(INTERNAL_MODULES)


func load_local_modules() -> void:
	for module_path: String in Singleton.loaded_modules:
		var module: Module = Singleton.loaded_modules.get(module_path)
		var module_item: ModuleItem = ModuleItem.from_module(module)
		var module_file: FileAccess = FileAccess.open(module.get_meta(&"path"), FileAccess.READ)
		module_item.module_size = module_file.get_length()
		module_item.module_preview_texture = load(module.module_image)
		module_item.is_local = true
		module_item.updated.connect(func(_p: String) -> void:
			updates_pending -= 1
		)
		module_file.close()
		local_loaded_modules.set(module.id, module_item)
		local_flow_container.add_child(module_item)


func _on_fetching_text_timeout() -> void:
	_fetch_text_iter = wrapi(_fetch_text_iter + 1, 0, 4)
	fetching_label.text = "Fetching available modules online" + ".".repeat(_fetch_text_iter)


func _on_module_metadata_fetched(data: Dictionary, source_url: String, module_size: int) -> void:
	if online_loaded_modules.has(data.id):
		return
	if local_loaded_modules.has(data.id):
		var local_module: ModuleItem = local_loaded_modules.get(data.id)
		var fetched_version: String = "%s.%s.%s" % [data.major_version, data.minor_version, data.patch_number]
		if Nebula.compare_versions(fetched_version, local_module.module_version.lstrip("v")) == 1:
			local_module.set_update_available(source_url)
			updates_pending += 1
		return
	
	var module_item: ModuleItem = ModuleItem.from_dict(data)
	module_item.module_source = source_url
	module_item.module_size = module_size
	module_item.installed_to_local.connect(_on_module_downloaded)
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


func update_module_count() -> void:
	installed_label.text = "Installed: %s" % local_flow_container.get_child_count()
	available_label.text = "Available: %s" % online_flow_container.get_child_count()


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
	Singleton.loaded_modules.set(id, Module.load(path))
