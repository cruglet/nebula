extends PanelContainer

signal close_requested

func _on_cancel_button_pressed() -> void:
	close_requested.emit()
