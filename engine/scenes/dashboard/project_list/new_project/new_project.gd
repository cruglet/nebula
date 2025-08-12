extends PanelContainer

signal cancel_pressed
signal switch_to_module_request

@export var project_preview: ProjectItem
@export var project_name_line_edit: LineEdit
@export var project_path_line_edit: LineEdit
@export var no_module_label: RichTextLabel

var dir_regex: RegEx

func _ready() -> void:
	dir_regex = RegEx.create_from_string("^(?!^(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(\\.|$))[^<>:\"/\\\\|?*\\x00-\\x1F]{1,255}$")
	set_project_name("My Project")


func set_project_name(p_name: String) -> void:
	var m: RegExMatch = dir_regex.search(p_name)
	if not m:
		project_name_line_edit.delete_char_at_caret()
		return
	var parsed_string: String = m.get_string()
	
	project_preview.project_name = parsed_string
	project_preview.project_path = OS.get_user_data_dir().path_join("projects").path_join(parsed_string).to_kebab_case() + '/'
	project_path_line_edit.text = project_preview.project_path


func validate_project_creation() -> void:
	pass


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
