extends PanelContainer

@export var project_preview: ProjectItem
@export var new_project_path_dialog: FileDialog
@export var project_path_button: Button

@export var info_label: RichTextLabel
@export var create_button: Button


func _ready() -> void:
	project_preview.project_name = "My Project"
	project_path_button.text = project_preview.project_path
	project_path_button.tooltip_text = project_path_button.text
	check_conditions()


func _on_cancel_button_pressed() -> void:
	Singleton.close_popup.emit()


func _on_project_name_changed(new_text: String) -> void:
	project_preview.project_name = new_text
	project_path_button.text = project_preview.project_path
	project_path_button.tooltip_text = project_path_button.text
	check_conditions()


func _on_project_path_button_pressed() -> void:
	new_project_path_dialog.show()


func _on_new_project_path_dir_selected(dir: String) -> void:
	project_preview.custom_project_path = dir
	_on_project_name_changed(project_preview.project_name)


func _on_create_button_pressed() -> void:
	EventBus.create_project_request.emit(project_preview.project_name, project_preview.project_path)
	Singleton.close_popup.emit()


func check_conditions() -> void:
	if DirAccess.dir_exists_absolute(project_preview.project_path):
		if DirAccess.get_files_at(project_preview.project_path).size() > 0:
			info_label.text = "[color=red]A directory with files inside already exists with this name."
			create_button.disabled = true
			return
	info_label.text = "[color=green]Everything's good to go!"
	create_button.disabled = false
