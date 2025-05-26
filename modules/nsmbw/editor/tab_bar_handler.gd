extends Node

const EMPTY_VIEWPORT: PackedScene = preload("uid://oiawe7xv0wv4")

@export var viewport_container: TabContainer


func _ready() -> void:
	var tab_bar: TabBar = viewport_container.get_tab_bar()
	
	tab_bar.tab_close_display_policy = TabBar.CLOSE_BUTTON_SHOW_ACTIVE_ONLY
	tab_bar.tab_close_pressed.connect(tab_close_requested)
	

func tab_close_requested(i: int) -> void:
	viewport_container.get_child(i).free()
	
	if viewport_container.get_tab_count() == 0:
		var empty_viewport: Node = EMPTY_VIEWPORT.duplicate(true).instantiate()
		empty_viewport.name = "<empty>"
		viewport_container.add_child(empty_viewport)
