@tool
extends Panel

@export var content_container: Control
@export var screen_label: Label
@export var tab_button_container: TabButtonContainer
@export var animation_player: AnimationPlayer


func _ready() -> void:
	animation_player.play(&"reveal_dashboard")


func _on_tab_button_container_selection_changed(index: int, _from: int) -> void:
	for child: Node in content_container.get_children():
		if child is Control:
			child.hide()
	
	content_container.get_child(index).show()
	screen_label.text = content_container.get_child(index).name


func _on_project_list_switch_screen_request(screen: int) -> void:
	tab_button_container.select(screen)
