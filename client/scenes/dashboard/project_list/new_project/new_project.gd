extends PanelContainer

const VALIDATION_ERR_NO_MODULES: String = "No modules found! Please download or import a module."
const VALIDATION_ERR_OCCUPIED: String = "A directory with files inside already exists with this name."

signal create_request(path: String, module: Module)
signal cancel_pressed
signal switch_to_module_request

@export var project_preview: ProjectItem
@export var project_name_line_edit: LineEdit
@export var project_path_line_edit: LineEdit
@export var module_options_container: VBoxContainer
@export var module_option_button: OptionButton
@export var error_label: RichTextLabel
@export var create_button: Button

var selected_module: Module
var module_project_images: Array = []
var dir_regex: RegEx

func _ready() -> void:
	dir_regex = RegEx.create_from_string("^(?!^(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(\\.|$))[^<>:\"/\\\\|?*\\x00-\\x1F]{1,255}$")
	set_project_name("My Project")
	repopulate_module_options()


func get_project_name() -> String:
	return project_preview.project_name


func get_project_path() -> String:
	return project_preview.project_path


func set_project_name(p_name: String) -> void:
	var m: RegExMatch = dir_regex.search(p_name)
	if not m:
		project_name_line_edit.delete_char_at_caret()
		return
	var parsed_string: String = m.get_string()
	
	project_preview.project_name = parsed_string
	project_preview.project_path = OS.get_user_data_dir().path_join("projects").path_join(parsed_string.to_kebab_case()) + '/'
	project_path_line_edit.text = project_preview.project_path
	validate_project_creation()


func repopulate_module_options() -> void:
	module_option_button.clear()
	module_project_images.clear()
	for module: Module in Singleton.get_modules():
		module_option_button.add_item(module.name)
		module_project_images.append(module.project_image)
	if Singleton.get_modules().size() > 0:
		_switch_module(0)


func validate_project_creation() -> void:
	if Singleton.get_modules().size() == 0:
		validation_error("[url]%s[/url]" % VALIDATION_ERR_NO_MODULES)
		return
	if DirAccess.dir_exists_absolute(project_preview.project_path):
		if DirAccess.get_files_at(project_preview.project_path):
			validation_error(VALIDATION_ERR_OCCUPIED)
			return
	
	error_label.hide()
	create_button.disabled = false


func validation_error(err: String) -> void:
	error_label.text = err
	error_label.show()
	create_button.disabled = true


func _switch_module(index: int) -> void:
	if not module_project_images.is_empty():
		project_preview.project_banner_texture = QuickActions.load_image(module_project_images[index])
		selected_module = Singleton.get_module(index)


func _on_cancel_button_pressed() -> void:
	cancel_pressed.emit()


func _on_project_name_line_edit_text_changed(new_text: String) -> void:
	if new_text.is_empty():
		set_project_name("My Project")
	else:
		set_project_name(new_text)


func _on_project_dir_button_pressed() -> void:
	pass


func _on_visibility_changed() -> void:
	if Singleton.get_modules().size() > 0:
		module_options_container.show()
	else:
		module_options_container.hide()
	
	repopulate_module_options()
	validate_project_creation()


func _on_module_option_button_item_selected(index: int) -> void:
	_switch_module(index)


func _on_create_button_pressed() -> void:
	create_request.emit(project_preview.project_path, selected_module)


func _on_error_label_meta_hover_started(_meta: Variant) -> void:
	error_label.meta_underlined = true


func _on_error_label_meta_hover_ended(_meta: Variant) -> void:
	error_label.meta_underlined = false


func _on_error_label_meta_clicked(meta: Variant) -> void:
	if meta == VALIDATION_ERR_NO_MODULES:
		cancel_pressed.emit()
		switch_to_module_request.emit()
