class_name FilesystemContextHandler
extends Node

enum ContextOption {
	RENAME,
	DELETE,
	CUT,
	COPY,
	PASTE,
	ADD_FOLDER,
}

signal rename_requested(path: String)
signal delete_requested(paths: Array[String])
signal add_folder_requested(parent_path: String)
signal paste_requested(target_path: String)

@export var tree: Tree

var _popup: PopupMenu
var _context_path: String = ""
var _context_selected_paths: Array[String] = []
var _clipboard_paths: Array[String] = []
var _clipboard_is_cut: bool = false

var is_read_only: bool = false
var root_dir: NebulaDir = null


func _ready() -> void:
	_setup_popup()
	if tree:
		tree.gui_input.connect(_on_tree_gui_input)
	else:
		push_error("FilesystemContextHandler: Tree not assigned!")


func _setup_popup() -> void:
	_popup = PopupMenu.new()
	add_child(_popup)
	_popup.id_pressed.connect(_on_popup_id_pressed)


func _on_tree_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		var mb: InputEventMouseButton = event as InputEventMouseButton
		if mb.button_index == MOUSE_BUTTON_RIGHT and mb.pressed:
			var item: TreeItem = tree.get_item_at_position(mb.position)
			if item:
				if not item.is_selected(0):
					tree.set_selected(item, 0)
				_show_menu(item, tree.get_screen_position() + mb.position)
			else:
				_show_menu(null, tree.get_screen_position() + mb.position)

			tree.accept_event()


func _show_menu(item: TreeItem, pos: Vector2) -> void:
	if not root_dir:
		return
	
	_context_selected_paths.clear()
	
	if item == null:
		_context_path = root_dir.get_path()
		if not is_read_only:
			_popup.clear()
			_popup.add_item("Add Folder", ContextOption.ADD_FOLDER)
			
			if not _clipboard_paths.is_empty():
				_popup.add_separator()
				_popup.add_item("Paste", ContextOption.PASTE)
			
			_popup.position = Vector2i(pos)
			_popup.popup()
		return
	
	var path: String = item.get_metadata(0)
	_context_path = path
	
	var selected_items: Array = _get_selected_items()
	_context_selected_paths.clear()
	for selected_item: TreeItem in selected_items:
		var context_path: String = selected_item.get_metadata(0)
		if context_path.get_extension() in Nebula.get_reserved_extensions():
			continue
		_context_selected_paths.append(context_path)
	
	var has_multiple: bool = _context_selected_paths.size() > 1
	var is_protected: bool = is_item_protected(path)
	var is_root: bool = path == root_dir.get_path()
	
	_popup.clear()
	
	if not is_read_only and not has_multiple:
		_popup.add_item("Add Folder", ContextOption.ADD_FOLDER)
	
	if not is_read_only:
		var has_unprotected: bool = false
		for selected_path: String in _context_selected_paths:
			if not is_item_protected(selected_path):
				has_unprotected = true
				break
		
		if has_unprotected:
			if not has_multiple and not is_protected:
				_popup.add_separator()
				_popup.add_item("Rename", ContextOption.RENAME)
			
			if has_multiple or (_context_selected_paths.size() == 1 and not is_protected):
				if not has_multiple:
					_popup.add_separator()
				var delete_text: String = "Delete %d items" % _context_selected_paths.size() if has_multiple else "Delete"
				_popup.add_item(delete_text, ContextOption.DELETE)
			
			if _context_selected_paths.size() > 0:
				_popup.add_separator()
				var cut_text: String = "Cut %d items" % _context_selected_paths.size() if has_multiple else "Cut"
				var copy_text: String = "Copy %d items" % _context_selected_paths.size() if has_multiple else "Copy"
				_popup.add_item(cut_text, ContextOption.CUT)
				_popup.add_item(copy_text, ContextOption.COPY)
	
	if not _clipboard_paths.is_empty() and not is_read_only and not has_multiple:
		if is_root or root_dir.dir_exists(path):
			_popup.add_separator()
			_popup.add_item("Paste", ContextOption.PASTE)
	
	if _popup.item_count == 0:
		return
	
	_popup.position = Vector2i(pos)
	_popup.popup()


func _on_popup_id_pressed(id: int) -> void:
	if _context_path == "":
		return
	
	match id:
		ContextOption.RENAME: 
			if not is_item_protected(_context_path):
				rename_requested.emit(_context_path)
		ContextOption.DELETE: 
			var paths: Array[String] = []
			for path: String in _context_selected_paths:
				if not is_item_protected(path):
					paths.append(path)
			if not paths.is_empty():
				delete_requested.emit(paths)
		ContextOption.CUT: 
			var paths: Array[String] = []
			for path: String in _context_selected_paths:
				if not is_item_protected(path):
					paths.append(path)
			if not paths.is_empty():
				_cut(paths)
		ContextOption.COPY: 
			var paths: Array[String] = []
			for path: String in _context_selected_paths:
				if not is_item_protected(path):
					paths.append(path)
			if not paths.is_empty():
				_copy(paths)
		ContextOption.PASTE: 
			paste_requested.emit(_context_path)
		ContextOption.ADD_FOLDER: 
			_add_folder(_context_path)


func _cut(paths: Array[String]) -> void:
	_clipboard_paths = paths.duplicate()
	_clipboard_is_cut = true


func _copy(paths: Array[String]) -> void:
	_clipboard_paths = paths.duplicate()
	_clipboard_is_cut = false


func _add_folder(path: String) -> void:
	if not root_dir:
		return
	
	var parent_dir: String = path if root_dir.dir_exists(path) else path.get_base_dir()
	add_folder_requested.emit(parent_dir)


func get_clipboard_info() -> Dictionary:
	return {
		"paths": _clipboard_paths,
		"is_cut": _clipboard_is_cut
	}


func clear_clipboard() -> void:
	_clipboard_paths.clear()
	_clipboard_is_cut = false


func is_item_protected(path: String) -> bool:
	if not root_dir:
		return true
	return path == root_dir.get_path() or path.get_extension() in Nebula.get_reserved_extensions()


func _get_selected_items() -> Array:
	var selected: Array = []
	var root_item: TreeItem = tree.get_root()
	if root_item:
		_collect_selected_items(root_item, selected)
	return selected


func _collect_selected_items(item: TreeItem, selected: Array) -> void:
	if item.is_selected(0):
		selected.append(item)
	
	var child: TreeItem = item.get_first_child()
	while child:
		_collect_selected_items(child, selected)
		child = child.get_next()
