extends Control


func _on_import_button_pressed() -> void:
	Singleton.error.emit("Importing has not been implemented yet!", "Ok", false)

func _on_create_button_pressed() -> void:
	EventBus.create_project_button_pressed.emit()
