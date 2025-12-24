class_name NebulaFilesystemPanel
extends Panel

signal file_open_request(path: String, file: NebulaFile)
signal file_renamed(original_path: String, new_path: String)

@export_group("Internal")
@export var filesystem_tree: Tree
@export var empty_filesystem_label: Label

@export_subgroup("Handlers")
@export var tree_handler: FilesystemTreeHandler
@export var context_handler: FilesystemContextHandler
@export var rename_dialog_handler: FilesystemRenameHandler

@export_subgroup("Delete Dialog")
@export var delete_dialog: NebulaWindow
@export var delete_label: RichTextLabel

var root_dir: NebulaDir = null:
	set(r):
		root_dir = r
		if tree_handler:
			tree_handler.root_dir = r
			tree_handler.set_root_from_dir(r)
			if filesystem_tree:
				var root_item: TreeItem = filesystem_tree.get_root()
				if root_item:
					_collapse_all_children(root_item)
		if context_handler:
			context_handler.root_dir = r
		if rename_dialog_handler:
			rename_dialog_handler.root_dir = r


var read_only: bool = false:
	set(value):
		read_only = value
		_update_read_only_state()

var _pending_delete_path: String = ""
var _pending_delete_paths: Array[String] = []
var _undo_redo: FilesystemUndoRedo


func _ready() -> void:
	_undo_redo = FilesystemUndoRedo.new(self)
	add_child(_undo_redo)
	_connect_signals()
	
	focus_mode = Control.FOCUS_CLICK
	_update_read_only_state()


func _input(event: InputEvent) -> void:
	if not has_focus() or read_only:
		return
	
	if event.is_action_pressed("ui_copy") or event.is_action_pressed("copy"):
		_handle_copy_action()
		get_viewport().set_input_as_handled()
	elif event.is_action_pressed("ui_cut") or event.is_action_pressed("cut"):
		_handle_cut_action()
		get_viewport().set_input_as_handled()
	elif event.is_action_pressed("ui_paste") or event.is_action_pressed("paste"):
		_handle_paste_action()
		get_viewport().set_input_as_handled()


func _notification(what: int) -> void:
	if what == NOTIFICATION_WM_WINDOW_FOCUS_IN:
		refresh()


func _connect_signals() -> void:
	if tree_handler:
		tree_handler.item_activated.connect(_on_item_activated)
		if not read_only:
			tree_handler.drag_drop_completed.connect(_on_drag_drop_completed)
	
	if context_handler:
		context_handler.rename_requested.connect(_on_rename_requested)
		context_handler.delete_requested.connect(_on_delete_requested)
		context_handler.add_folder_requested.connect(_on_add_folder_requested)
		context_handler.paste_requested.connect(_on_paste_requested)
	
	if rename_dialog_handler:
		rename_dialog_handler.file_renamed.connect(_on_file_renamed)
		rename_dialog_handler.folder_created.connect(_on_folder_created)
	
	if delete_dialog:
		delete_dialog.confirmed.connect(_on_delete_confirmed)
	
	if filesystem_tree:
		filesystem_tree.focus_entered.connect(_on_tree_focus_entered)


func _update_read_only_state() -> void:
	if tree_handler:
		tree_handler.is_read_only = read_only
	if context_handler:
		context_handler.is_read_only = read_only


func _on_tree_focus_entered() -> void:
	grab_focus()


func refresh() -> void:
	if tree_handler:
		tree_handler.refresh()


func _notify_file_renamed(original_path: String, new_path: String) -> void:
	file_renamed.emit(original_path, new_path)


func _on_item_activated(path: String, is_directory: bool) -> void:
	if not is_directory and root_dir:
		var file: NebulaFile = root_dir.get_file(path)
		if file:
			file_open_request.emit(path, file)


func _on_drag_drop_completed(source_path: String, target_path: String) -> void:
	if read_only or not root_dir:
		return
	
	_undo_redo.add_drag_drop_action(source_path, target_path)
	file_renamed.emit(source_path, target_path)


func _on_rename_requested(path: String) -> void:
	if read_only or not root_dir:
		return
	
	if root_dir.dir_exists(path):
		rename_dialog_handler.show_rename_folder(path)
	else:
		rename_dialog_handler.show_rename_file(path)


func _on_delete_requested(paths: Array[String]) -> void:
	if read_only or not root_dir:
		return
	
	_pending_delete_paths = paths
	_pending_delete_path = ""
	
	var is_multiple: bool = paths.size() > 1
	
	if is_multiple:
		delete_label.text = "Are you sure you want to delete %d items?" % paths.size()
		delete_dialog.set_header_text("Delete Multiple Items")
	else:
		var path: String = paths[0]
		var is_dir: bool = root_dir.dir_exists(path)
		delete_label.text = "Are you sure you want to delete \"%s\"?" % path.get_file()
		
		if is_dir:
			var dir: NebulaDir = root_dir.get_dir(path)
			if dir and dir.get_files().size() > 0:
				delete_label.text += "\n\n[i](All content in this folder will also be deleted!)[/i]"
		
		delete_dialog.set_header_text("Delete %s" % ("Folder" if is_dir else "File"))
	
	delete_dialog.show()


func _on_add_folder_requested(parent_path: String) -> void:
	if read_only or not root_dir:
		return
	
	rename_dialog_handler.show_create_folder(parent_path)


func _on_paste_requested(target_path: String) -> void:
	if read_only or not root_dir:
		return
	
	_perform_paste(target_path)


func _handle_copy_action() -> void:
	var selected_items: Array = _get_selected_tree_items()
	if selected_items.is_empty():
		return
	
	var paths: Array[String] = []
	for item: TreeItem in selected_items:
		var path: String = item.get_metadata(0)
		if not context_handler.is_item_protected(path):
			paths.append(path)
	
	if not paths.is_empty():
		context_handler._copy(paths)


func _handle_cut_action() -> void:
	if read_only:
		return
	
	var selected_items: Array = _get_selected_tree_items()
	if selected_items.is_empty():
		return
	
	var paths: Array[String] = []
	for item: TreeItem in selected_items:
		var path: String = item.get_metadata(0)
		if not context_handler.is_item_protected(path):
			paths.append(path)
	
	if not paths.is_empty():
		context_handler._cut(paths)


func _handle_paste_action() -> void:
	if read_only or not root_dir:
		return
	
	var selected_items: Array = _get_selected_tree_items()
	var target_path: String = ""
	
	if not selected_items.is_empty():
		target_path = selected_items[0].get_metadata(0)
	
	_perform_paste(target_path)


func _get_selected_tree_items() -> Array:
	var selected: Array = []
	if tree_handler:
		var root_item: TreeItem = filesystem_tree.get_root()
		if root_item:
			_collect_selected_tree_items(root_item, selected)
	return selected


func _collect_selected_tree_items(item: TreeItem, selected: Array) -> void:
	if item.is_selected(0):
		selected.append(item)
	
	var child: TreeItem = item.get_first_child()
	while child:
		_collect_selected_tree_items(child, selected)
		child = child.get_next()


func _perform_paste(target_path: String) -> void:
	if read_only or not root_dir:
		return
	
	var clipboard: Dictionary = context_handler.get_clipboard_info()
	if not clipboard.has("paths"):
		return
	
	var source_paths: Array = clipboard.paths
	var is_cut: bool = clipboard.is_cut
	
	if source_paths.is_empty():
		return
	
	var target_dir: String = target_path
	if not root_dir.dir_exists(target_path):
		target_dir = target_path.get_base_dir()
	
	for source_path: String in source_paths:
		if target_dir == source_path or target_dir.begins_with(source_path + "/"):
			push_error("Cannot paste into itself or its subdirectory")
			continue
		
		if target_dir == source_path.get_base_dir() and is_cut:
			push_error("Cannot move item to its current location")
			continue
		
		var file_name: String = source_path.get_file()
		var new_path: String = target_dir.path_join(file_name)
		new_path = _get_unique_path(new_path)
		
		if is_cut:
			if root_dir.rename_path(source_path, new_path):
				_undo_redo.add_paste_action(source_path, new_path, true)
				file_renamed.emit(source_path, new_path)
			else:
				push_error("Failed to move: " + source_path)
		else:
			if root_dir.dir_exists(source_path):
				_copy_directory(source_path, new_path)
			else:
				_copy_file(source_path, new_path)
			
			_undo_redo.add_paste_action(source_path, new_path, false)
	
	if is_cut:
		context_handler.clear_clipboard()
	
	refresh()


func _on_file_renamed(original_path: String, new_path: String) -> void:
	if read_only or not root_dir:
		return
	
	_undo_redo.add_rename_action(original_path, new_path)
	file_renamed.emit(original_path, new_path)
	refresh()


func _on_folder_created(path: String) -> void:
	if read_only or not root_dir:
		return
	
	_undo_redo.add_create_folder_action(path)
	refresh()


func _on_delete_confirmed() -> void:
	if read_only or not root_dir:
		return
	
	if not _pending_delete_paths.is_empty():
		_undo_redo.add_delete_action(_pending_delete_paths)
		for path: String in _pending_delete_paths:
			_delete_recursive(path)
		_pending_delete_paths.clear()
	elif _pending_delete_path:
		_undo_redo.add_delete_action([_pending_delete_path])
		_delete_recursive(_pending_delete_path)
		_pending_delete_path = ""
	
	refresh()


func _delete_recursive(path: String) -> void:
	if not root_dir:
		return
	
	if root_dir.dir_exists(path):
		var dir: NebulaDir = root_dir.get_dir(path)
		if not dir:
			push_error("Failed to open directory: " + path)
			return
		
		var entries: PackedStringArray = dir.get_entries()
		for entry: String in entries:
			var is_dir: bool = entry.ends_with("/")
			var entry_name: String = entry.trim_suffix("/")
			var full_path: String = path.path_join(entry_name)
			_delete_recursive(full_path)
		
		root_dir.remove_dir(path)
	else:
		root_dir.remove_file(path)


func _copy_file(source: String, target: String) -> void:
	if not root_dir:
		return
	
	var source_file: NebulaFile = root_dir.get_file(source)
	if not source_file:
		push_error("Source file not found: " + source)
		return
	
	var buffer: NebulaBuffer = source_file.get_buffer()
	if not buffer:
		push_error("Failed to get buffer from source file: " + source)
		return
	
	var target_file: NebulaFile = root_dir.create_file(target)
	if not target_file:
		push_error("Failed to create target file: " + target)
		return
	
	target_file.set_buffer(buffer)


func _copy_directory(source: String, target: String) -> void:
	if not root_dir:
		return
	
	if not root_dir.create_dir(target):
		push_error("Failed to create target directory: " + target)
		return
	
	var source_dir: NebulaDir = root_dir.get_dir(source)
	if not source_dir:
		push_error("Source directory not found: " + source)
		return
	
	var entries: PackedStringArray = source_dir.get_entries()
	
	for entry: String in entries:
		var is_dir: bool = entry.ends_with("/")
		var entry_name: String = entry.trim_suffix("/")
		var source_path: String = source.path_join(entry_name)
		var target_path: String = target.path_join(entry_name)
		
		if is_dir:
			_copy_directory(source_path, target_path)
		else:
			_copy_file(source_path, target_path)


func _get_unique_path(path: String) -> String:
	if not root_dir:
		return path
	
	if not root_dir.file_exists(path) and not root_dir.dir_exists(path):
		return path
	
	var base_dir: String = path.get_base_dir()
	var file_name: String = path.get_file()
	var extension: String = path.get_extension()
	var base_name: String = file_name.get_basename()
	
	var counter: int = 1
	var new_path: String = path
	
	while root_dir.file_exists(new_path) or root_dir.dir_exists(new_path):
		if extension:
			new_path = base_dir.path_join("%s_%d.%s" % [base_name, counter, extension])
		else:
			new_path = base_dir.path_join("%s_%d" % [base_name, counter])
		counter += 1
	
	return new_path


func _collapse_all_children(item: TreeItem) -> void:
	var child: TreeItem = item.get_first_child()
	while child:
		child.collapsed = true
		_collapse_all_children(child)
		child = child.get_next()
