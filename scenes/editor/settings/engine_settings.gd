extends PanelContainer

func _on_close_button_pressed() -> void:
	EngineConfig.save()
	Singleton.close_popup.emit()
