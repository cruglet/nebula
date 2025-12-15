class_name NebulaFilesystemDock
extends Panel

signal file_open_request(path: String)
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

var root: String = "":
	set(r):
		root = r
		if tree_handler:
			tree_handler.root_path = r
			tree_handler.set_root(r)
		if context_handler:
			context_handler.root_path = r

var _pending_delete_path: String = ""
var _pending_delete_paths: Array[String] = []


func _ready() -> void:
	_connect_signals()


func _notification(what: int) -> void:
	if what == NOTIFICATION_WM_WINDOW_FOCUS_IN:
		refresh()


func _connect_signals() -> void:
	if tree_handler:
		tree_handler.item_activated.connect(_on_item_activated)
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


func refresh() -> void:
	if tree_handler:
		tree_handler.refresh()


func _on_item_activated(path: String, is_directory: bool) -> void:
	if not is_directory:
		file_open_request.emit(path)


func _on_drag_drop_completed(source_path: String, target_path: String) -> void:
	file_renamed.emit(source_path, target_path)


func _on_rename_requested(path: String) -> void:
	if DirAccess.dir_exists_absolute(path):
		rename_dialog_handler.show_rename_folder(path)
	else:
		rename_dialog_handler.show_rename_file(path)


func _on_delete_requested(paths: Array[String]) -> void:
	_pending_delete_paths = paths
	_pending_delete_path = ""
	
	var is_multiple: bool = paths.size() > 1
	
	if is_multiple:
		delete_label.text = "Are you sure you want to delete %d items?" % paths.size()
		delete_dialog.set_header_text("Delete Multiple Items")
	else:
		var path: String = paths[0]
		var is_dir: bool = DirAccess.dir_exists_absolute(path)
		delete_label.text = "Are you sure you want to delete \"%s\"?" % path.get_file()
		
		if is_dir and DirAccess.get_files_at(path).size() > 0:
			delete_label.text += "\n\n[i](All content in this folder will also be deleted!)[/i]"
		
		delete_dialog.set_header_text("Delete %s" % ("Folder" if is_dir else "File"))
	
	delete_dialog.show()


func _on_add_folder_requested(parent_path: String) -> void:
	rename_dialog_handler.show_create_folder(parent_path)


func _on_paste_requested(target_path: String) -> void:
	var clipboard: Dictionary = context_handler.get_clipboard_info()
	print("Paste requested. Source: ", clipboard.path, " Target: ", target_path, " Is Cut: ", clipboard.is_cut)
	# TODO: Implement paste functionality


func _on_file_renamed(original_path: String, new_path: String) -> void:
	file_renamed.emit(original_path, new_path)
	refresh()


func _on_folder_created(_path: String) -> void:
	refresh()


func _on_delete_confirmed() -> void:
	if not _pending_delete_paths.is_empty():
		for path: String in _pending_delete_paths:
			_delete_recursive(path)
		_pending_delete_paths.clear()
	elif _pending_delete_path:
		_delete_recursive(_pending_delete_path)
		_pending_delete_path = ""
	
	refresh()


func _delete_recursive(path: String) -> void:
	if DirAccess.dir_exists_absolute(path):
		var dir: DirAccess = DirAccess.open(path)
		if dir == null:
			push_error("Failed to open directory: " + path)
			return
		
		dir.list_dir_begin()
		var file_name: String = dir.get_next()
		
		while file_name != "":
			if file_name != "." and file_name != "..":
				var full_path: String = path.path_join(file_name)
				_delete_recursive(full_path)
			file_name = dir.get_next()
		
		dir.list_dir_end()
		
		DirAccess.remove_absolute(path)
	else:
		DirAccess.remove_absolute(path)
