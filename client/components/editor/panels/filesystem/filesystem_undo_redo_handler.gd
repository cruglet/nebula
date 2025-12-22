class_name FilesystemUndoRedo
extends Node

signal action_performed()

var _undo_redo: UndoRedo = UndoRedo.new()
var _filesystem_dock: NebulaFilesystemPanel


func _init(dock: NebulaFilesystemPanel) -> void:
	_filesystem_dock = dock


func _input(event: InputEvent) -> void:
	if not _filesystem_dock or not _filesystem_dock.has_focus():
		return
	
	if event.is_action_pressed("undo"):
		undo()
		get_viewport().set_input_as_handled()
	elif event.is_action_pressed("redo"):
		redo()
		get_viewport().set_input_as_handled()


func undo() -> void:
	if _undo_redo.has_undo():
		_undo_redo.undo()
		_filesystem_dock.refresh()
		action_performed.emit()


func redo() -> void:
	if _undo_redo.has_redo():
		_undo_redo.redo()
		_filesystem_dock.refresh()
		action_performed.emit()


func clear() -> void:
	_undo_redo.clear_history()


func has_undo() -> bool:
	return _undo_redo.has_undo()


func has_redo() -> bool:
	return _undo_redo.has_redo()


func get_undo_name() -> String:
	if _undo_redo.has_undo():
		return _undo_redo.get_current_action_name()
	return ""


func get_redo_name() -> String:
	if _undo_redo.has_redo():
		return _undo_redo.get_current_action_name()
	return ""


func add_rename_action(old_path: String, new_path: String) -> void:
	_undo_redo.create_action("Rename %s" % old_path.get_file())
	_undo_redo.add_do_method(_rename_file.bind(old_path, new_path))
	_undo_redo.add_undo_method(_rename_file.bind(new_path, old_path))
	_undo_redo.add_do_method(_filesystem_dock._notify_file_renamed.bind(old_path, new_path))
	_undo_redo.add_undo_method(_filesystem_dock._notify_file_renamed.bind(new_path, old_path))
	_undo_redo.commit_action()
	action_performed.emit()


func add_delete_action(paths: Array[String]) -> void:
	var backup_data: Array[Dictionary] = []
	
	for path: String in paths:
		backup_data.append(_backup_path(path))
	
	var action_name: String = "Delete %d item%s" % [paths.size(), "s" if paths.size() > 1 else ""]
	_undo_redo.create_action(action_name)
	
	for path: String in paths:
		_undo_redo.add_do_method(_delete_path.bind(path))
	
	for data: Dictionary in backup_data:
		_undo_redo.add_undo_method(_restore_path.bind(data))
	
	_undo_redo.commit_action()
	action_performed.emit()


func add_create_folder_action(path: String) -> void:
	_undo_redo.create_action("Create Folder %s" % path.get_file())
	_undo_redo.add_do_method(_create_folder.bind(path))
	_undo_redo.add_undo_method(_delete_path.bind(path))
	_undo_redo.commit_action()
	action_performed.emit()


func add_paste_action(source_path: String, target_path: String, is_cut: bool) -> void:
	var action_name: String = "%s %s" % ["Move" if is_cut else "Copy", source_path.get_file()]
	
	_undo_redo.create_action(action_name)
	
	if is_cut:
		_undo_redo.add_do_method(_move_path.bind(source_path, target_path))
		_undo_redo.add_undo_method(_move_path.bind(target_path, source_path))
		_undo_redo.add_do_method(_filesystem_dock._notify_file_renamed.bind(source_path, target_path))
		_undo_redo.add_undo_method(_filesystem_dock._notify_file_renamed.bind(target_path, source_path))
	else:
		var backup_data: Dictionary = {}
		_undo_redo.add_do_method(_copy_path.bind(source_path, target_path))
		_undo_redo.add_do_method(_capture_backup.bind(target_path, backup_data))
		_undo_redo.add_undo_method(_delete_path.bind(target_path))
		_undo_redo.add_do_method(_restore_from_backup.bind(backup_data))
	
	_undo_redo.commit_action()
	action_performed.emit()


func add_drag_drop_action(source_path: String, target_path: String) -> void:
	_undo_redo.create_action("Move %s" % source_path.get_file())
	_undo_redo.add_do_method(_move_path.bind(source_path, target_path))
	_undo_redo.add_undo_method(_move_path.bind(target_path, source_path))
	_undo_redo.add_do_method(_filesystem_dock._notify_file_renamed.bind(source_path, target_path))
	_undo_redo.add_undo_method(_filesystem_dock._notify_file_renamed.bind(target_path, source_path))
	_undo_redo.commit_action()
	action_performed.emit()



func _rename_file(old_path: String, new_path: String) -> void:
	var dir: DirAccess = DirAccess.open(old_path.get_base_dir())
	if dir:
		dir.rename(old_path, new_path)
		if DirAccess.dir_exists_absolute(new_path) and _filesystem_dock.tree_handler:
			_filesystem_dock.tree_handler.update_collapsed_paths_after_rename(old_path, new_path)


func _delete_path(path: String) -> void:
	_delete_recursive(path)


func _create_folder(path: String) -> void:
	DirAccess.make_dir_absolute(path)


func _move_path(source: String, target: String) -> void:
	var dir: DirAccess = DirAccess.open(source.get_base_dir())
	if dir:
		dir.rename(source, target)
		if DirAccess.dir_exists_absolute(target) and _filesystem_dock.tree_handler:
			_filesystem_dock.tree_handler.update_collapsed_paths_after_rename(source, target)


func _copy_path(source: String, target: String) -> void:
	if DirAccess.dir_exists_absolute(source):
		_copy_directory(source, target)
	else:
		_copy_file(source, target)


func _capture_backup(path: String, backup_dict: Dictionary) -> void:
	backup_dict["data"] = _backup_path(path)


func _restore_from_backup(backup_dict: Dictionary) -> void:
	if backup_dict.has("data"):
		_restore_path(backup_dict["data"])


func _restore_path(backup_data: Dictionary) -> void:
	var path: String = backup_data.path
	var is_dir: bool = backup_data.is_dir
	
	if is_dir:
		DirAccess.make_dir_absolute(path)
		for child_data: Dictionary in backup_data.children:
			_restore_path(child_data)
	else:
		var file: FileAccess = FileAccess.open(path, FileAccess.WRITE)
		if file:
			file.store_buffer(backup_data.content)
			file.close()


func _backup_path(path: String) -> Dictionary:
	var data: Dictionary = {
		"path": path,
		"is_dir": DirAccess.dir_exists_absolute(path)
	}
	
	if data.is_dir:
		data.children = []
		var dir: DirAccess = DirAccess.open(path)
		if dir:
			dir.list_dir_begin()
			var entry: String = dir.get_next()
			
			while entry != "":
				if entry != "." and entry != "..":
					var child_path: String = path.path_join(entry)
					data.children.append(_backup_path(child_path))
				entry = dir.get_next()
			
			dir.list_dir_end()
	else:
		var file: FileAccess = FileAccess.open(path, FileAccess.READ)
		if file:
			data.content = file.get_buffer(file.get_length())
			file.close()
	
	return data


func _delete_recursive(path: String) -> void:
	if DirAccess.dir_exists_absolute(path):
		var dir: DirAccess = DirAccess.open(path)
		if dir:
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


func _copy_file(source: String, target: String) -> void:
	var dir: DirAccess = DirAccess.open(source.get_base_dir())
	if dir:
		dir.copy(source, target)


func _copy_directory(source: String, target: String) -> void:
	DirAccess.make_dir_absolute(target)
	
	var dir: DirAccess = DirAccess.open(source)
	if dir:
		dir.list_dir_begin()
		var entry: String = dir.get_next()
		
		while entry != "":
			if entry != "." and entry != "..":
				var source_path: String = source.path_join(entry)
				var target_path: String = target.path_join(entry)
				
				if DirAccess.dir_exists_absolute(source_path):
					_copy_directory(source_path, target_path)
				else:
					_copy_file(source_path, target_path)
			
			entry = dir.get_next()
		
		dir.list_dir_end()
