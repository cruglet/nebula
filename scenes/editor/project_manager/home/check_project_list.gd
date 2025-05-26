extends Node

const PROJECT_ITEM: PackedScene = preload("uid://dejm314chwoes")

@export var empty_project_list_container: Control
@export var project_list_container: Control
@export var project_list_vbox: VBoxContainer
@export var project_num_label: Label


func _ready() -> void:
	populate_list()


func check_project_list() -> void:
	if Nebula.Config.Editor.projects.size() > 0:
		project_num_label.text = "Projects - %s" % Nebula.Config.Editor.projects.size()
		project_list_container.show()
		empty_project_list_container.hide()
	else:
		project_list_container.hide()
		empty_project_list_container.show()


func populate_list() -> void:
	var project_list: Array = Nebula.Config.Editor.projects
	var project_count: int = 0
	
	var i: int = 0
	
	for child: Node in project_list_vbox.get_children():
		project_list_vbox.remove_child(child)
		child.queue_free()
	
	while i < project_list.size():
		var project_item: ProjectItem = PROJECT_ITEM.duplicate(true).instantiate()
		
		if FileAccess.file_exists(project_list.get(i)):
			var project: Nebula.Project = Nebula.Project.load(project_list.get(i))
			
			project_item.project_name = project.name
			project_item.project_banner.texture = Nebula.GAME_LIST.get(Nebula.find_in_game_list(project.type)).banner
			
			# TODO there seems to be a weird bug with Godot where scroll
			# events aren't propegated correctly.
			project_list_vbox.add_child(project_item)
			project_count += 1
		else:
			project_list.remove_at(i)
			Singleton.toast_notification("Project(s) not found!", "Nebula could not locate one or more of your project files.\nIf this is a mistake, try re-importing.", 5)
			i -= 1
		i += 1
	
	Nebula.Config.Editor.projects = project_list
	Nebula.Config.save()
	check_project_list()


func silent_check() -> void:
	var project_list: Array = Nebula.Config.Editor.projects
	var project_count: int = project_list.size()
	
	var i: int = 0
	while i < project_list.size():
		if !FileAccess.file_exists(project_list.get(i)):
			Singleton.toast_notification("Project(s) not found!", "Nebula could not locate one or more of your project files.\nIf this is a mistake, try re-importing.", 5)
			project_list.remove_at(i)
			i -= 1
		i += 1
	
	Nebula.Config.Editor.projects = project_list
	Nebula.Config.save()
	
	if project_count != project_list.size():
		populate_list()
	
	check_project_list()
