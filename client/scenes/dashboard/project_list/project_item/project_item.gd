class_name ProjectItem
extends Button

signal open_project_request(instance: ProjectItem)
signal remove_project_request(instance: ProjectItem)

@export var is_preview: bool = false:
	set(ip):
		if ip:
			gui_block.show()
			focus_mode = Control.FOCUS_NONE
			remove_button.focus_mode = Control.FOCUS_NONE
		else:
			gui_block.hide()
			focus_mode = Control.FOCUS_ALL
			remove_button.focus_mode = Control.FOCUS_ALL
		is_preview = ip
@export var project_name: String:
	get():
		if project_name.is_empty():
			return "My Project"
		else:
			return project_name
	set(pn):
		project_name_label.text = pn
		project_name = pn
@export var project_path: String:
	set(pp):
		project_path_label.text = pp
		project_path = pp
@export var project_banner_texture: Texture2D:
	set(ppt):
		if ppt:
			project_banner_rect.texture = ppt
			project_banner_texture = ppt
@export_group("Internal")
@export var project_name_label: Label
@export var project_path_label: Label
@export var project_banner_rect: TextureRect
@export var remove_button: Button
@export var gui_block: Control

var remove_button_hovering: bool = false:
	set(f):
		remove_button_hovering = f
		_play_hover_animation()

var item_hovering: bool = false:
	set(ih):
		item_hovering = ih
		_show_remove_button()


func _play_hover_animation() -> void:
	var tween: Tween = get_tree().create_tween()
	tween.set_ease(Tween.EASE_OUT)
	
	tween.tween_property(remove_button, ^"scale", Vector2(1.1, 1.1) if remove_button_hovering else Vector2.ONE, 0.1)


func _on_remove_button_mouse_entered() -> void:
	if is_preview:
		return
	remove_button_hovering = true


func _on_remove_button_mouse_exited() -> void:
	remove_button_hovering = false


func _on_remove_button_focus_entered() -> void:
	_show_remove_button()


func _on_remove_button_focus_exited() -> void:
	if not remove_button_hovering:
		_hide_remove_button()


func _show_remove_button() -> void:
	var tween: Tween = get_tree().create_tween()
	tween.set_ease(Tween.EASE_OUT)
	tween.set_parallel(true)
	
	tween.tween_property(remove_button, ^"scale", Vector2.ONE, 0.1)
	tween.tween_property(remove_button, ^"anchor_top", 0.0, 0.1)
	tween.tween_property(remove_button, ^"anchor_bottom", 0.0, 0.1)
	tween.tween_property(remove_button, ^"self_modulate", Color(1, 1, 1, 1), 0.1)
	

func _hide_remove_button() -> void:
	if remove_button_hovering or not is_inside_tree():
		return
	
	var tween: Tween = get_tree().create_tween()
	tween.set_ease(Tween.EASE_OUT)
	tween.set_parallel(true)
	
	tween.tween_property(remove_button, ^"scale", Vector2(0.8, 0.8), 0.1)
	tween.tween_property(remove_button, ^"anchor_top", -0.1, 0.1)
	tween.tween_property(remove_button, ^"anchor_bottom", -0.1, 0.1)
	tween.tween_property(remove_button, ^"self_modulate", Color.TRANSPARENT, 0.1)
	
	remove_button.release_focus()


func _on_mouse_entered() -> void:
	_show_remove_button()


func _on_mouse_exited() -> void:
	_hide_remove_button.call_deferred()


func _on_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.double_click:
			open_project_request.emit(self)


func _on_remove_button_pressed() -> void:
	remove_button_hovering = false
	_hide_remove_button()
	remove_project_request.emit(self)
