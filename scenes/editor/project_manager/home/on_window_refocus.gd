extends Node

@export var check_project_list: Node


func _notification(what: int) -> void:
	if what == NOTIFICATION_WM_WINDOW_FOCUS_IN:
		check_project_list.silent_check()
