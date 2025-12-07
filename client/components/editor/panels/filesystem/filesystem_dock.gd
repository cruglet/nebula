class_name NebulaFilesystemDock
extends Panel

signal file_open_request(path: String)
signal file_renamed(original_path: String, new_path: String)

@export_group("Internal")
@export var filesystem_tree: Tree
@export var empty_filesystem_label: Label

@export_subgroup("Rename Dialog")
@export var rename_dialog: NebulaWindow
@export var rename_line_edit: LineEdit
@export var dot_label: Label
@export var extension_edit: LineEdit
@export var error_label: RichTextLabel
@export var rename_button: Button

var root: String = "":
	set(r):
		root = r
		refresh()

var _context_menu: PopupMenu
var _clipboard_path: String = ""
var _clipboard_is_cut: bool = false
var _context_menu_path: String = ""


func _ready() -> void:
	_setup_context_menu()
	_setup_tree()
	refresh()
	filesystem_tree.item_activated.connect(_on_item_activated)
	rename_dialog.confirmed.connect(_on_rename_dialog_confirmed)
	rename_line_edit.text_changed.connect(_on_rename_text_changed)
	extension_edit.text_changed.connect(_on_rename_text_changed)


func _notification(what: int) -> void:
	if what == NOTIFICATION_WM_WINDOW_FOCUS_IN:
		refresh()


func _setup_tree() -> void:
	filesystem_tree.set_drag_forwarding(
		_get_drag_data_fw,
		_can_drop_data_fw,
		_drop_data_fw
	)


func _setup_context_menu() -> void:
	_context_menu = PopupMenu.new()
	add_child(_context_menu)
	_context_menu.id_pressed.connect(_on_context_menu_id_pressed)
	
	filesystem_tree.gui_input.connect(_on_tree_gui_input)


func _on_tree_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		var mb: InputEventMouseButton = event as InputEventMouseButton
		if mb.button_index == MOUSE_BUTTON_RIGHT and mb.pressed:
			var item: TreeItem = filesystem_tree.get_item_at_position(mb.position)
			if item:
				filesystem_tree.set_selected(item, 0)
				_show_context_menu(item, mb.global_position)


func _show_context_menu(item: TreeItem, pos: Vector2) -> void:
	var path: String = item.get_metadata(0)
	var is_preserved: bool = (
		path == root or path.get_extension() in Nebula.get_reserved_extensions()
	)
	
	_context_menu_path = path
	
	_context_menu.clear()
	
	_context_menu.add_item("Add Folder", 5)
	
	if not is_preserved:
		_context_menu.add_separator()
		_context_menu.add_item("Rename", 0)
		_context_menu.add_item("Delete", 1)
		_context_menu.add_separator()
		_context_menu.add_item("Cut", 2)
		_context_menu.add_item("Copy", 3)
	
	if _clipboard_path != "":
		if not is_preserved:
			_context_menu.add_separator()
		_context_menu.add_item("Paste", 4)
	
	_context_menu.position = Vector2i(pos)
	_context_menu.popup()


func _on_context_menu_id_pressed(id: int) -> void:
	if _context_menu_path == "":
		return
	
	var path: String = _context_menu_path
	
	match id:
		0: _request_rename(path)
		1: _request_delete(path)
		2: _request_cut(path)
		3: _request_copy(path)
		4: _request_paste(path)
		5: _request_add_folder(path)


func _request_rename(path: String) -> void:
	rename_dialog.show()
	rename_line_edit.text = ""
	extension_edit.text = ""
	
	# If this is a dir
	if path.get_basename() == path:
		rename_dialog.set_header_text("Rename Folder")
		dot_label.hide()
		extension_edit.hide()
	else: # a file
		rename_dialog.set_header_text("Rename File")
		dot_label.show()
		extension_edit.show()
	
	rename_line_edit.placeholder_text = path.get_file().get_basename()
	extension_edit.placeholder_text = path.get_extension()
	error_label.hide()
	rename_button.disabled = false
	rename_line_edit.grab_focus()


func _request_delete(path: String) -> void:
	# TODO: Implement delete functionality
	print("Delete requested for: ", path)


func _request_cut(path: String) -> void:
	_clipboard_path = path
	_clipboard_is_cut = true
	print("Cut: ", path)


func _request_copy(path: String) -> void:
	_clipboard_path = path
	_clipboard_is_cut = false
	print("Copy: ", path)


func _request_paste(target_path: String) -> void:
	# TODO: Implement paste functionality
	print("Paste requested. Source: ", _clipboard_path, " Target: ", target_path, " Is Cut: ", _clipboard_is_cut)


func _request_add_folder(path: String) -> void:
	# TODO: Implement add folder functionality
	var parent_dir: String = path if DirAccess.dir_exists_absolute(path) else path.get_base_dir()
	print("Add folder requested in: ", parent_dir)


func _get_drag_data_fw(_at_position: Vector2) -> Variant:
	var item: TreeItem = filesystem_tree.get_selected()
	if not item:
		return null
	
	var path: String = item.get_metadata(0)
	if path == root or path.get_extension() in Nebula.get_reserved_extensions():
		return null
	
	var preview: Label = Label.new()
	preview.text = item.get_text(0)
	filesystem_tree.set_drag_preview(preview)
	
	return {"type": "file", "path": path}


func _can_drop_data_fw(at_position: Vector2, data: Variant) -> bool:
	if typeof(data) != TYPE_DICTIONARY:
		return false
	
	if not data.has("type") or data["type"] != "file":
		return false
	
	var item: TreeItem = filesystem_tree.get_item_at_position(at_position)
	if not item:
		return false
	
	var target_path: String = item.get_metadata(0)
	var source_path: String = data["path"]
	
	if target_path == source_path:
		return false
	
	if not DirAccess.dir_exists_absolute(target_path):
		target_path = target_path.get_base_dir()
	
	return target_path != source_path.get_base_dir()


func _drop_data_fw(at_position: Vector2, data: Variant) -> void:
	var item: TreeItem = filesystem_tree.get_item_at_position(at_position)
	if not item:
		return
	
	var target_path: String = item.get_metadata(0)
	var source_path: String = data["path"]
	
	if not DirAccess.dir_exists_absolute(target_path):
		target_path = target_path.get_base_dir()
	
	var file_name: String = source_path.get_file()
	var new_path: String = target_path.path_join(file_name)
	
	var dir: DirAccess = DirAccess.open(source_path.get_base_dir())
	if dir:
		dir.rename(source_path, new_path)
	
	refresh()


func refresh() -> void:
	filesystem_tree.clear()
	var dir: DirAccess = DirAccess.open(root)
	if dir == null or not root:
		empty_filesystem_label.show()
		return
	
	dir.list_dir_begin()
	var has_items: bool = false
	var entry: String = dir.get_next()
	
	while entry != "":
		if entry != "." and entry != "..":
			has_items = true
			break
		entry = dir.get_next()
	
	dir.list_dir_end()
	
	if not has_items:
		empty_filesystem_label.show()
		return
	
	empty_filesystem_label.hide()
	
	var tree_root: TreeItem = filesystem_tree.create_item()
	tree_root.set_text(0, "root")
	tree_root.set_icon(0, get_theme_icon(&"fs_folder", &"nIcons"))
	tree_root.set_metadata(0, root)
	_populate_directory(tree_root, root)


func _populate_directory(parent: TreeItem, path: String) -> void:
	var dir: DirAccess = DirAccess.open(path)
	if dir == null:
		return
	
	dir.list_dir_begin()
	var file_name: String = dir.get_next()
	var directories: Array[String] = []
	var files: Array[String] = []
	
	while file_name != "":
		if file_name != "." and file_name != "..":
			if dir.current_is_dir():
				directories.append(file_name)
			else:
				files.append(file_name)
		file_name = dir.get_next()
	
	dir.list_dir_end()
	
	directories.sort()
	files.sort()
	
	for directory: String in directories:
		var item: TreeItem = filesystem_tree.create_item(parent)
		item.set_text(0, directory)
		item.set_icon(0, get_theme_icon(&"fs_folder", &"nIcons"))
		var dir_path: String = path.path_join(directory)
		item.set_metadata(0, dir_path)
		_populate_directory(item, dir_path)
	
	for file: String in files:
		var item: TreeItem = filesystem_tree.create_item(parent)
		item.set_text(0, file)
		item.set_icon(0, _get_file_icon(file))
		item.set_metadata(0, path.path_join(file))


## TODO: Refactor once proper theme system is made
func _get_file_icon(file_name: String) -> Texture2D:
	var extension: String = file_name.get_extension().to_lower()
	
	match extension:
		"png", "jpg", "jpeg", "svg", "webp":
			return get_theme_icon("Image", "EditorIcons")
		"mp3", "ogg", "wav":
			return get_theme_icon("AudioStreamPlayer", "EditorIcons")
		"glb", "gltf", "obj":
			return get_theme_icon("MeshInstance3D", "EditorIcons")
		"txt", "json", "md":
			return get_theme_icon(&"fs_file_text", &"nIcons")
		"nproj":
			return get_theme_icon(&"base_icon", &"nIcons")
		_:
			return get_theme_icon("File", "EditorIcons")


func _on_item_activated() -> void:
	var item: TreeItem = filesystem_tree.get_selected()
	var path: String = item.get_metadata(0)
	if path == "":
		return
	
	if DirAccess.dir_exists_absolute(path):
		item.collapsed = not item.collapsed
	else:
		file_open_request.emit(path)

#region Rename
func _on_rename_dialog_confirmed() -> void:
	var filename: String = rename_line_edit.text if rename_line_edit.text else rename_line_edit.placeholder_text
	var extension: String = extension_edit.text if extension_edit.text else extension_edit.placeholder_text
	
	if filename.strip_edges().is_empty():
		_show_rename_error("Filename cannot be empty")
		return
	
	var invalid_chars: String = "\\/:*?\"<>|"
	for c: String in invalid_chars:
		if filename.contains(c) or extension.contains(c):
			_show_rename_error("Filename contains invalid characters: %s" % invalid_chars)
			return
	
	var new_filename: String = "%s.%s" % [filename, extension]
	
	# In case this is a directory
	if not extension_edit.placeholder_text:
		new_filename = filename
	
	var new_path: String = _context_menu_path.get_base_dir().path_join(new_filename)
	
	if new_path != _context_menu_path and (FileAccess.file_exists(new_path) or DirAccess.dir_exists_absolute(new_path)):
		_show_rename_error("A file or folder with that name already exists")
		return
	
	if not FileAccess.file_exists(_context_menu_path) and not DirAccess.dir_exists_absolute(_context_menu_path):
		_show_rename_error("Source file no longer exists")
		return
	
	var error: int = DirAccess.rename_absolute(_context_menu_path, new_path)
	if error != OK:
		_show_rename_error("Failed to rename: Error code %d" % error)
		return
	
	var original_path: String = _context_menu_path
	file_renamed.emit(original_path, new_path)
	rename_dialog.hide()
	refresh()


func _show_rename_error(message: String) -> void:
	error_label.text = message
	error_label.show()
	rename_button.disabled = true


func _on_filename_edit_text_submitted(_new_text: String) -> void:
	if not rename_button.disabled:
		_on_rename_dialog_confirmed()


func _on_rename_text_changed(_new_text: String) -> void:
	var filename: String = rename_line_edit.text if rename_line_edit.text else rename_line_edit.placeholder_text
	var extension: String = extension_edit.text if extension_edit.text else extension_edit.placeholder_text
	
	if extension in Nebula.get_reserved_extensions():
		_show_rename_error("This extension cannot be used")
		return
	
	if filename.strip_edges().is_empty():
		_show_rename_error("Filename cannot be empty")
		return
	
	var invalid_chars: String = "\\/:*?\"<>|"
	for c: String in invalid_chars:
		if filename.contains(c) or extension.contains(c):
			_show_rename_error("Filename contains invalid characters: %s" % invalid_chars)
			return
	
	
	var new_filename: String = "%s.%s" % [filename, extension]
	
	var new_path: String = _context_menu_path.get_base_dir().path_join(new_filename)
	
	if new_path != _context_menu_path and (FileAccess.file_exists(new_path) or DirAccess.dir_exists_absolute(new_path)):
		_show_rename_error("A file or folder with that name already exists")
		return
	
	error_label.hide()
	rename_button.disabled = false
#endregion
