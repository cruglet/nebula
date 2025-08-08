@tool
class_name TabButtonContainer
extends BoxContainer

signal selection_changed(index: int, from: int)

@export var selected_index: int = -1:
	set(value):
		var old_index: int = selected_index
		
		if buttons.size() > 0:
			selected_index = clamp(value, -1, buttons.size() - 1)
		else:
			selected_index = value
		
		if old_index != selected_index:
			_update_button_states(old_index, selected_index)
			selection_changed.emit(selected_index, old_index)

var buttons: Array[Button] = []


func _ready() -> void:
	if not child_order_changed.is_connected(_update_button_references):
		child_order_changed.connect(_update_button_references)
	_update_button_references()
	
	selected_index = clamp(selected_index, -1, buttons.size() - 1)
	_update_button_states(-1, selected_index)


func select(index: int) -> void:
	if index == selected_index:
		buttons[selected_index].button_pressed = true
		return
	
	selected_index = index


func _update_button_references() -> void:
	_disconnect_all_buttons()
	buttons.clear()
	
	for child: Node in get_children():
		if child is Button:
			var button: Button = child as Button
			button.toggle_mode = true
			buttons.append(button)
	
	_connect_all_buttons()
	
	if selected_index >= buttons.size():
		selected_index = buttons.size() - 1


func _update_button_states(old_index: int, new_index: int) -> void:
	if old_index >= 0 and old_index < buttons.size():
		buttons[old_index].button_pressed = false
	
	if new_index >= 0 and new_index < buttons.size():
		buttons[new_index].button_pressed = true


func _disconnect_all_buttons() -> void:
	for button: Button in buttons:
		if button.pressed.is_connected(select):
			button.pressed.disconnect(select)


func _connect_all_buttons() -> void:
	for i: int in range(buttons.size()):
		if not buttons[i].pressed.is_connected(select):
			buttons[i].pressed.connect(select.bind(i))
