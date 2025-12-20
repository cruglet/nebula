## Handles rename and create folder dialogs
class_name FilesystemRenameHandler
extends Node

signal file_renamed(original_path: String, new_path: String)
signal folder_created(path: String)

@export var dialog: NebulaWindow
@export var line_edit: LineEdit
@export var dot_label: Label
@export var extension_edit: LineEdit
@export var error_label: RichTextLabel
@export var confirm_button: Button
@export var tree_handler: FilesystemTreeHandler

enum Mode { RENAME_FILE, RENAME_FOLDER, CREATE_FOLDER }

var _current_mode: Mode
var _target_path: String = ""


func _ready() -> void:
	if dialog:
		dialog.confirmed.connect(_on_confirmed)
	if line_edit:
		line_edit.text_changed.connect(_on_text_changed)
	if extension_edit:
		extension_edit.text_changed.connect(_on_text_changed)


func show_rename_file(path: String) -> void:
	_current_mode = Mode.RENAME_FILE
	_target_path = path
	
	dialog.set_header_text("Rename File")
	dot_label.show()
	extension_edit.show()
	confirm_button.text = "Rename"
	
	line_edit.text = ""
	extension_edit.text = ""
	line_edit.placeholder_text = path.get_file().get_basename()
	extension_edit.placeholder_text = path.get_extension()
	
	_reset_validation()
	dialog.show()
	line_edit.grab_focus()


func show_rename_folder(path: String) -> void:
	_current_mode = Mode.RENAME_FOLDER
	_target_path = path
	
	dialog.set_header_text("Rename Folder")
	dot_label.hide()
	extension_edit.hide()
	confirm_button.text = "Rename"
	
	line_edit.text = ""
	line_edit.placeholder_text = path.get_file()
	
	_reset_validation()
	dialog.show()
	line_edit.grab_focus()


func show_create_folder(parent_path: String) -> void:
	_current_mode = Mode.CREATE_FOLDER
	_target_path = parent_path
	
	dialog.set_header_text("Add Folder")
	dot_label.hide()
	extension_edit.hide()
	confirm_button.text = "Create"
	
	line_edit.text = ""
	line_edit.placeholder_text = "New Folder"
	
	_reset_validation()
	dialog.show()
	line_edit.grab_focus()


func _reset_validation() -> void:
	error_label.hide()
	confirm_button.disabled = false


func _on_confirmed() -> void:
	match _current_mode:
		Mode.RENAME_FILE:
			_handle_rename_file()
		Mode.RENAME_FOLDER:
			_handle_rename_folder()
		Mode.CREATE_FOLDER:
			_handle_create_folder()


func _handle_rename_file() -> void:
	var filename: String = line_edit.text if line_edit.text else line_edit.placeholder_text
	var extension: String = extension_edit.text if extension_edit.text else extension_edit.placeholder_text
	
	var validation: Dictionary = _validate_filename(filename, extension)
	if not validation.valid:
		_show_error(validation.error)
		return
	
	var new_filename: String = "%s.%s" % [filename, extension]
	var new_path: String = _target_path.get_base_dir().path_join(new_filename)
	
	if new_path != _target_path and (FileAccess.file_exists(new_path) or DirAccess.dir_exists_absolute(new_path)):
		_show_error("A file or folder with that name already exists")
		return
	
	if not FileAccess.file_exists(_target_path):
		_show_error("Source file no longer exists")
		return
	
	var error: int = DirAccess.rename_absolute(_target_path, new_path)
	if error != OK:
		_show_error("Failed to rename: Error code %d" % error)
		return
	
	file_renamed.emit(_target_path, new_path)
	dialog.hide()


func _handle_rename_folder() -> void:
	var foldername: String = line_edit.text if line_edit.text else line_edit.placeholder_text
	
	var validation: Dictionary = _validate_filename(foldername, "")
	if not validation.valid:
		_show_error(validation.error)
		return
	
	var new_path: String = _target_path.get_base_dir().path_join(foldername)
	
	if new_path != _target_path and DirAccess.dir_exists_absolute(new_path):
		_show_error("A folder with that name already exists")
		return
	
	if not DirAccess.dir_exists_absolute(_target_path):
		_show_error("Source folder no longer exists")
		return
	
	var error: int = DirAccess.rename_absolute(_target_path, new_path)
	if error != OK:
		_show_error("Failed to rename: Error code %d" % error)
		return
	
	if tree_handler:
		tree_handler.update_collapsed_paths_after_rename(_target_path, new_path)
	
	file_renamed.emit(_target_path, new_path)
	dialog.hide()


func _handle_create_folder() -> void:
	var foldername: String = line_edit.text if line_edit.text else line_edit.placeholder_text
	
	var validation: Dictionary = _validate_filename(foldername, "")
	if not validation.valid:
		_show_error(validation.error)
		return
	
	var new_path: String = _target_path.path_join(foldername)
	
	if DirAccess.dir_exists_absolute(new_path):
		_show_error("A folder with that name already exists")
		return
	
	var error: int = DirAccess.make_dir_absolute(new_path)
	if error != OK:
		_show_error("Failed to create folder: Error code %d" % error)
		return
	
	folder_created.emit(new_path)
	dialog.hide()


func _validate_filename(filename: String, extension: String) -> Dictionary:
	if filename.strip_edges().is_empty():
		return {"valid": false, "error": "Filename cannot be empty"}
	
	var invalid_chars: String = "\\/:*?\"<>|"
	for c: String in invalid_chars:
		if filename.contains(c) or extension.contains(c):
			return {"valid": false, "error": "Filename contains invalid characters: %s" % invalid_chars}
	
	if extension in Nebula.get_reserved_extensions():
		return {"valid": false, "error": "This extension cannot be used"}
	
	return {"valid": true}


func _on_text_changed(_new_text: String) -> void:
	var filename: String = line_edit.text if line_edit.text else line_edit.placeholder_text
	var extension: String = ""
	
	if _current_mode == Mode.RENAME_FILE:
		extension = extension_edit.text if extension_edit.text else extension_edit.placeholder_text
	
	var validation: Dictionary = _validate_filename(filename, extension)
	
	if not validation.valid:
		_show_error(validation.error)
		return
	
	var new_name: String = filename if _current_mode == Mode.RENAME_FOLDER else "%s.%s" % [filename, extension]
	var base_dir: String = _target_path.get_base_dir() if _current_mode != Mode.CREATE_FOLDER else _target_path
	var new_path: String = base_dir.path_join(new_name)
	
	if _current_mode != Mode.CREATE_FOLDER and new_path != _target_path and (FileAccess.file_exists(new_path) or DirAccess.dir_exists_absolute(new_path)):
		_show_error("A file or folder with that name already exists")
		return
	
	if _current_mode == Mode.CREATE_FOLDER and DirAccess.dir_exists_absolute(new_path):
		_show_error("A folder with that name already exists")
		return
	
	error_label.hide()
	confirm_button.disabled = false


func _show_error(message: String) -> void:
	error_label.text = message
	error_label.show()
	confirm_button.disabled = true
