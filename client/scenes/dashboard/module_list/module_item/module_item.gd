class_name ModuleItem
extends PanelContainer

const MODULE_ITEM: PackedScene = preload("uid://bs1txkbdc5lg7")

signal installed_to_local(id: String, path: String)
signal updated(id: String)

@export var module_name: String:
	set(mn):
		_preview_name_label.text = mn
		module_name = mn
@export var module_description: String:
	set(md):
		_preview_description_label.text = md
		module_description = md
@export var module_version: String:
	set(mv):
		_preview_version_label.text = "v" + mv
		module_version = mv
@export var module_preview_texture: Texture2D:
	set(mpt):
		_preview_image_rect.texture = mpt
		module_preview_texture = mpt
@export var module_size: int:
	set(ms):
		_preview_size_label.text = String.humanize_size(ms)
		module_size = ms
@export var module_file_path: String:
	get():
		if not module_file_path:
			return OS.get_user_data_dir().path_join("modules").path_join(module_id) + ".nmod"
		else:
			return module_file_path
@export var module_id: String
@export var module_source: String



@export_group("Internal")
@export var _preview_name_label: Label
@export var _preview_description_label: Label
@export var _preview_version_label: Label
@export var _preview_size_label: Label
@export var _preview_image_rect: TextureRect
@export var _download_button: Button
@export var _download_progress_bar: ProgressBar
@export var _http_request: HTTPRequest
@export var _update_available_text: RichTextLabel

var downloading: bool = false
var is_local: bool = false
var update_available: bool = false
var update_url: String
var update_version: String
var update_size: int


static func from_dict(dict: Dictionary) -> ModuleItem:
	var module_item: ModuleItem = MODULE_ITEM.duplicate().instantiate()
	module_item.module_name = dict.get("name")
	module_item.module_description = dict.get("description")
	module_item.module_version = "%s.%s.%s" % [dict.get("major_version"), dict.get("minor_version"), dict.get("patch_number")]
	module_item.module_id = dict.get("id")
	return module_item


static func from_module(module: Module) -> ModuleItem:
	var module_item: ModuleItem = MODULE_ITEM.duplicate().instantiate()
	module_item.module_name = module.name
	module_item.module_description = module.description
	module_item.module_version = "%s.%s.%s" % [module.major_version, module.minor_version, module.patch_number]
	module_item.module_id = module.id
	return module_item


func _ready() -> void:
	_check_downloadable()


func _process(_delta: float) -> void:
	if downloading:
		_while_downloading()


func matches(search_string: String) -> bool:
	var search_str: String = search_string.to_lower().to_snake_case()
	if module_name.to_snake_case().begins_with(search_str):
		return true
	elif module_id.begins_with(search_str):
		return true
	return false


func set_update_available(source_url: String, module_update_size: int, module_update_version: String) -> void:
	update_available = true
	_update_available_text.show()
	is_local = false
	update_url = source_url
	update_version = module_update_version
	update_size = module_update_size
	_check_downloadable()


func get_module_version_string() -> String:
	return module_version


func _check_downloadable() -> void:
	if is_local and not update_available:
		_download_button.hide()
	else:
		_download_button.show()


func _on_download_button_pressed() -> void:
	downloading = true
	_download_button.disabled = true
	_http_request.request_completed.connect(_on_download_completed, CONNECT_ONE_SHOT)
	if update_available:
		_http_request.request(update_url)
	else:
		_http_request.request(module_source)


func _while_downloading() -> void:
	_download_progress_bar.value = (_http_request.get_downloaded_bytes() / float(_http_request.get_body_size())) * 100


func _on_download_completed(result: int, response: int, _headers: PackedStringArray, body: PackedByteArray) -> void:
	if result != OK or response != 200:
		push_error("Could not download module! ", result, response)
		return
	
	if not DirAccess.dir_exists_absolute(module_file_path.get_base_dir()):
		DirAccess.make_dir_recursive_absolute(module_file_path.get_base_dir())
	
	var file: FileAccess = FileAccess.open(module_file_path, FileAccess.WRITE)
	file.store_buffer(body)
	file.close()
	
	if update_available:
		updated.emit(module_id)
		update_available = false
		module_version = update_version
		module_size = update_size
	
	downloading = false
	is_local = true
	_download_progress_bar.hide()
	_update_available_text.hide()
	installed_to_local.emit(module_id, module_file_path)
	_check_downloadable()
