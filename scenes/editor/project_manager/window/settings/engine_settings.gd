extends PanelContainer

func _on_close_button_pressed() -> void:
	NebulaConfig.save()
	Singleton.close_popup.emit()


func _on_restore_defaults_button_pressed() -> void:
	pass # Replace with function body.
