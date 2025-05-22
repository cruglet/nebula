extends Node

signal toast_interaction

@export var toast_timer: Node


func _on_toast_notification_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.button_index == 1 and event.is_released() and not event.double_click:
		toast_timer.timer.paused = true
		toast_timer._on_finish()
		toast_interaction.emit() # Maybe do something with this in the future idk
