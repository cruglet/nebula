class_name NebulaTextEditor
extends Control

signal unsaved(status: bool)

@export var text_editor: CodeEdit
var font_size: int = 12
var changes_made: bool = false
var file_path: String:
	set(fp):
		text_editor.text = FileAccess.get_file_as_string(fp)
		file_path = fp


func _ready() -> void:
	font_size = text_editor.get_theme_default_font_size()


func _input(event: InputEvent) -> void:
	if event.is_action_pressed("save"):
		changes_made = false
		
		var file: FileAccess = FileAccess.open(file_path, FileAccess.WRITE)
		if file.store_string(text_editor.text):
			unsaved.emit(false)
			Singleton.send_notification("Saved file", "Changes have been saved!")
		else:
			Singleton.send_notification("Error saving file", "The file could not be written to.")


func _on_text_editor_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		match event.button_index:
			MOUSE_BUTTON_WHEEL_UP when event.ctrl_pressed:
				font_size += 1
				accept_event()
				_update_text()
			MOUSE_BUTTON_WHEEL_DOWN when event.ctrl_pressed:
				font_size -= 1
				accept_event()
				_update_text()
	
	if event is InputEventKey and event.pressed:
		match event.keycode:
			KEY_EQUAL when event.ctrl_pressed:
				font_size += 4
				accept_event()
				_update_text()
			KEY_MINUS when event.ctrl_pressed:
				font_size -= 4
				accept_event()
				_update_text()


func _update_text() -> void:
	font_size = clampi(font_size, 6, 48)
	text_editor.add_theme_font_size_override(&"font_size", font_size)


func _on_text_editor_text_changed() -> void:
	changes_made = true
	unsaved.emit(true)
