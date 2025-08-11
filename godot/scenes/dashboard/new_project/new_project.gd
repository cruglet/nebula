extends PanelContainer

signal cancel_pressed


func _on_cancel_button_pressed() -> void:
	cancel_pressed.emit()
