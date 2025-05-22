extends Node


func _on_cancel_button_pressed() -> void:
	owner.get_parent().hide()
