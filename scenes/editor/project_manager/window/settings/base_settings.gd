extends PanelContainer

@export var default_project_path_button: Button
@export var default_project_path_dialog: FileDialog

func _ready() -> void:
	default_project_path_button.text = EngineConfig.default_project_path
	default_project_path_button.tooltip_text = default_project_path_button.text

func _on_default_path_button_pressed() -> void:
	default_project_path_dialog.show()

func _on_default_path_dialog_dir_selected(dir: String) -> void:
	EngineConfig.default_project_path = dir
	default_project_path_button.text = EngineConfig.default_project_path
	default_project_path_button.tooltip_text = default_project_path_button.text
