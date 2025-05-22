extends Node

signal check_conditions

@export var project_preview: ProjectItem
@export var project_name_edit: LineEdit
@export var project_path_button: Button


func _ready() -> void:
	_on_project_info_updated(0)
	owner.visibility_changed.connect(_update_project_banner)


func _on_project_info_updated(_a: Variant) -> void:
	var project_name: String
	
	if project_name_edit.text.strip_edges().length() > 0:
		project_name = project_name_edit.text
	else:
		project_name = "My Project"
	
	project_preview.project_name = project_name
	project_preview.project_path = project_name.to_kebab_case()
	project_path_button.text = project_preview.project_path
	project_path_button.tooltip_text = project_path_button.text
	
	check_conditions.emit()


func _update_project_banner() -> void:
	var editor_id: String = Nebula.find_in_game_list(Singleton.opened_disc.game_id)
	
	if editor_id:
		project_preview.project_banner.texture = Nebula.GAME_LIST.get(editor_id).banner
	else:
		project_preview.project_banner.texture = load("uid://qjixe1gcnmph")
