class_name IconSidebarButton
extends Button

@export var button_label_text: String = ""
@export_enum("Right", "Left") var button_label_position: int = 0
@export_group("Internal")
@export var _button_label: Label
@export var _panel_container: PanelContainer
@export var _button_label_size: Label
@export var _notification_badge: Panel

var hide_tween: Tween
var show_tween: Tween
var notification_tween: Tween


func _ready() -> void:
	_panel_container.modulate = Color.TRANSPARENT
	_panel_container.size.x = 20
	_button_label.text = button_label_text
	_button_label_size.text = button_label_text
	assign_label_position()


func assign_label_position() -> void:
	match button_label_position:
		0: # Right
			_panel_container.grow_horizontal = Control.GROW_DIRECTION_END
			_panel_container.position = Vector2(46, 5)
		1: # Left
			_panel_container.grow_horizontal = Control.GROW_DIRECTION_BEGIN
			_panel_container.position = Vector2(-24, 5)


func show_notification_badge() -> void:
	notification_tween = get_tree().create_tween()
	
	_notification_badge.scale = Vector2.ZERO
	_notification_badge.show()
	
	notification_tween.set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUINT)
	notification_tween.tween_property(_notification_badge, ^"scale", Vector2(1.1, 1.1), 0.3)


func hide_notification_badge() -> void:
	if not _notification_badge.visible:
		return
	
	notification_tween = get_tree().create_tween()
	
	notification_tween.set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUINT)
	notification_tween.tween_property(_notification_badge, ^"scale", Vector2.ZERO, 0.3)
	notification_tween.finished.connect(func() -> void:
		_notification_badge.hide(),
		CONNECT_ONE_SHOT
	)


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
	
	show_tween.tween_property(_panel_container, ^"custom_minimum_size:x", _button_label_size.size.x + 13, 0.25)
	show_tween.tween_property(_panel_container, ^"modulate", Color.WHITE, 0.1)


func _hide_button_label() -> void:
	hide_tween = get_tree().create_tween()
	
	hide_tween.set_parallel(true)
	hide_tween.set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUINT)
	
	hide_tween.tween_property(_panel_container, ^"custom_minimum_size:x", 20, 0.25)
	hide_tween.tween_property(_panel_container, ^"modulate", Color.TRANSPARENT, 0.25)





func _on_toggled(toggled_on: bool) -> void:
	if toggled_on and is_inside_tree():
		_hide_button_label()
