class_name IconSidebarButton
extends Button

@export var button_label_text: String = ""
@export_group("Internal")
@export var button_label: Label
@export var panel_container: PanelContainer
@export var button_label_size: Label

var hide_tween: Tween
var show_tween: Tween


func _ready() -> void:
	panel_container.modulate = Color.TRANSPARENT
	panel_container.size.x = 20
	button_label.text = button_label_text
	button_label_size.text = button_label_text


func _on_mouse_entered() -> void:
	_show_button_label()


func _on_mouse_exited() -> void:
	_hide_button_label()


func _show_button_label() -> void:
	if button_pressed or button_label_text.is_empty():
		return
	
	if hide_tween and hide_tween.is_valid():
		hide_tween.kill()
	
	show_tween = get_tree().create_tween()
	
	show_tween.set_parallel(true)
	show_tween.set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUINT)
	
	show_tween.tween_property(panel_container, ^"size:x", button_label_size.size.x + 13, 0.25)
	show_tween.tween_property(panel_container, ^"modulate", Color.WHITE, 0.1)


func _hide_button_label() -> void:
	hide_tween = get_tree().create_tween()
	
	hide_tween.set_parallel(true)
	hide_tween.set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUINT)
	
	hide_tween.tween_property(panel_container, ^"size:x", 20, 0.25)
	hide_tween.tween_property(panel_container, ^"modulate", Color.TRANSPARENT, 0.25)


func _on_toggled(toggled_on: bool) -> void:
	if toggled_on and is_inside_tree():
		_hide_button_label()
