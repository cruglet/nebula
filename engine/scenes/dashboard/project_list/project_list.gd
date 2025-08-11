extends Panel

@export var no_projects: Control
@export var projects: Control
@export var project_list_vbox: VBoxContainer
@export var blur_overlay: ColorRect

func _ready() -> void:
	var project_list: Array[String] = CoreSettings.get(CoreSettings.SETTING_PROJECT_LIST)
	
	if project_list.is_empty():
		no_projects.show()
	else:
		projects.show()


func show_blur() -> void:
	blur_overlay.material.set_shader_parameter(&"blur_amount", 0.0)
	blur_overlay.show()
	
	var tween: Tween = get_tree().create_tween()
	tween.tween_property(blur_overlay.material, ^"shader_parameter/blur_amount", 2.5, 0.25)


func hide_blur() -> void:
	blur_overlay.material.set_shader_parameter(&"blur_amount", 2.5)
	
	var tween: Tween = get_tree().create_tween()
	tween.tween_property(blur_overlay.material, ^"shader_parameter/blur_amount", 0.0, 0.25)
	
	await tween.finished
	blur_overlay.hide()


func _on_create_button_pressed() -> void:
	release_focus()
	show_blur()
	$NebulaWindow.show()


func _on_nebula_window_hide_request() -> void:
	hide_blur()


func _on_new_project_cancel_pressed() -> void:
	$NebulaWindow.hide()
