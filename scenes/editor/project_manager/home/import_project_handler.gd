extends Node

const PROJECT_ITEM: PackedScene = preload("uid://dejm314chwoes")

@export var open_project_dialog: FileDialog
@export var check_project_list: Node
@export var project_list_vbox: VBoxContainer


func _on_import_button_pressed() -> void:
	open_project_dialog.show()
	#Singleton.toast_notification("Under construction!", "Importing projects has not been implemented yet.")


func _on_project_file_selected(path: String) -> void:
	var data: Variant = Nebula.Project.load(path)
	var project: Nebula.Project
	var project_item: ProjectItem = PROJECT_ITEM.duplicate(true).instantiate()
	
	if data and data is Nebula.Project:
		project = data
		
		if project.path.path_join("project.nebula") in Nebula.Config.Editor.projects:
			Singleton.toast_notification("Import skipped", "This project has already been imported!")
			return
		
		Nebula.Config.Editor.projects.insert(0, project.path.path_join("project.nebula"))
		project_item.project_name = project.name
		project_item.project_banner.texture = Nebula.GAME_LIST.get(Nebula.find_in_game_list(project.type)).banner
		project_list_vbox.add_child(project_item)
		project_list_vbox.move_child(project_item, 0)
		check_project_list.check_project_list()
		Nebula.Config.save()
