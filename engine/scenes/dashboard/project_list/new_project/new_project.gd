extends PanelContainer

signal cancel_pressed
signal switch_to_module_request

@export var project_preview: ProjectItem
@export var project_name_line_edit: LineEdit
@export var project_path_line_edit: LineEdit
@export var no_module_label: RichTextLabel
@export var module_options_container: VBoxContainer
@export var module_option_button: OptionButton

var module_project_images: Array = []
var dir_regex: RegEx

func _ready() -> void:
	dir_regex = RegEx.create_from_string("^(?!^(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(\\.|$))[^<>:\"/\\\\|?*\\x00-\\x1F]{1,255}$")
	set_project_name("My Project")
	repopulate_module_options()


func set_project_name(p_name: String) -> void:
	var m: RegExMatch = dir_regex.search(p_name)
	if not m:
		project_name_line_edit.delete_char_at_caret()
		return
	var parsed_string: String = m.get_string()
	
	project_preview.project_name = parsed_string
	project_preview.project_path = OS.get_user_data_dir().path_join("projects").path_join(parsed_string).to_kebab_case() + '/'
	project_path_line_edit.text = project_preview.project_path


func repopulate_module_options() -> void:
	module_option_button.clear()
	module_project_images.clear()
	for module_path: String in Singleton.loaded_modules:
		var module: Module = Singleton.loaded_modules.get(module_path)
		module_option_button.add_item(module.name)
		module_project_images.append(module.project_image)
	if Singleton.loaded_modules.size() > 0:
		_switch_banner(0)


func validate_project_creation() -> void:
	pass


func _switch_banner(index: int) -> void:
	if not module_project_images.is_empty():
		project_preview.project_banner_texture = load(module_project_images[index])


func _on_cancel_button_pressed() -> void:
	cancel_pressed.emit()


func _on_project_name_line_edit_text_changed(new_text: String) -> void:
	if new_text.is_empty():
		set_project_name("My Project")
	else:
		set_project_name(new_text)


func _on_project_dir_button_pressed() -> void:
	pass


func _on_no_module_label_meta_hover_started(_meta: Variant) -> void:
	no_module_label.meta_underlined = true


func _on_no_module_label_meta_hover_ended(_meta: Variant) -> void:
	no_module_label.meta_underlined = false


func _on_no_module_label_meta_clicked(_meta: Variant) -> void:
	cancel_pressed.emit()
	switch_to_module_request.emit()


func _on_visibility_changed() -> void:
	if Singleton.loaded_modules.size() > 0:
		no_module_label.hide()
		module_options_container.show()
		repopulate_module_options()
	else:
		no_module_label.show()
		module_options_container.hide()


func _on_module_option_button_item_selected(index: int) -> void:
	_switch_banner(index)
