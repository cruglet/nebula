class_name FilesystemTreeHandler
extends Node

signal item_activated(path: String, is_directory: bool)
signal item_collapsed_changed(path: String, collapsed: bool)
signal drag_drop_completed(source_path: String, target_path: String)

@export var tree: Tree
@export var empty_label: Label

var root_dir: NebulaDir = null
var _collapsed_paths: Dictionary = {}

var is_read_only: bool = false


func _ready() -> void:
	if tree:
		tree.item_activated.connect(_on_item_activated)
		tree.item_collapsed.connect(_on_item_collapsed)
		tree.set_drag_forwarding(
			_get_drag_data_fw,
			_can_drop_data_fw,
			_drop_data_fw
		)
		tree.set_select_mode(Tree.SELECT_MULTI)


func set_root_from_dir(dir: NebulaDir) -> void:
	root_dir = dir
	refresh()


func refresh() -> void:
	tree.clear()
	
	if not root_dir:
		empty_label.show()
		return
	
	if not _has_items(root_dir):
		empty_label.show()
		return
	
	empty_label.hide()
	
	var tree_root: TreeItem = tree.create_item()
	tree_root.set_text(0, get_root_display_name())
	tree_root.set_icon(0, get_root_icon())
	tree_root.set_metadata(0, root_dir.get_path())
	
	if _collapsed_paths.has(root_dir.get_path()):
		tree_root.collapsed = _collapsed_paths[root_dir.get_path()]
	
	_populate_directory(tree_root, root_dir, root_dir.get_path())


func _has_items(dir: NebulaDir) -> bool:
	var entries: PackedStringArray = dir.get_entries()
	return entries.size() > 0


func _populate_directory(parent: TreeItem, dir: NebulaDir, current_abs_path: String) -> void:
	if not dir:
		return
	
	var entries: PackedStringArray = dir.get_entries()
	var directories: Array[String] = []
	var files: Array[String] = []
	
	for entry: String in entries:
		var is_dir: bool = entry.ends_with("/")
		var entry_name: String = entry.trim_suffix("/")
		
		if should_show_entry(entry_name, is_dir):
			if is_dir:
				directories.append(entry_name)
			else:
				files.append(entry_name)
	
	directories.sort()
	files.sort()
	
	for directory: String in directories:
		var item: TreeItem = tree.create_item(parent)
		item.set_text(0, directory)
		item.set_icon(0, get_folder_icon())
		var dir_abs_path: String = current_abs_path.path_join(directory)
		item.set_metadata(0, dir_abs_path)
		
		if _collapsed_paths.has(dir_abs_path):
			item.collapsed = _collapsed_paths[dir_abs_path]
		
		var sub_dir: NebulaDir = dir.get_dir(directory)
		if sub_dir:
			_populate_directory(item, sub_dir, dir_abs_path)
	
	for file: String in files:
		var item: TreeItem = tree.create_item(parent)
		item.set_text(0, file)
		item.set_icon(0, get_file_icon(file))
		item.set_metadata(0, current_abs_path.path_join(file))


func get_root_display_name() -> String:
	return "root"


func get_root_icon() -> Texture2D:
	return tree.get_theme_icon(&"fs_folder", &"nIcons")


func get_folder_icon() -> Texture2D:
	return tree.get_theme_icon(&"fs_folder", &"nIcons")


func get_file_icon(file_name: String) -> Texture2D:
	var extension: String = file_name.get_extension().to_lower()
	
	match extension:
		"png", "jpg", "jpeg", "svg", "webp":
			return tree.get_theme_icon("Image", "EditorIcons")
		"mp3", "ogg", "wav":
			return tree.get_theme_icon("AudioStreamPlayer", "EditorIcons")
		"glb", "gltf", "obj":
			return tree.get_theme_icon("MeshInstance3D", "EditorIcons")
		"txt", "json", "md":
			return tree.get_theme_icon(&"fs_file_text", &"nIcons")
		"nproj":
			return tree.get_theme_icon(&"base_icon", &"nIcons")
		_:
			return tree.get_theme_icon("File", "EditorIcons")


func should_show_entry(_entry_name: String, _is_dir: bool) -> bool:
	return true


func _get_relative_path(absolute_path: String) -> String:
	if not root_dir:
		return absolute_path
	var root_path: String = root_dir.get_path()
	if absolute_path.begins_with(root_path + "/"):
		return absolute_path.substr(root_path.length() + 1)
	elif absolute_path == root_path:
		return ""
	return absolute_path


func _on_item_activated() -> void:
	var item: TreeItem = tree.get_selected()
	if not item or not root_dir:
		return
	
	var path: String = item.get_metadata(0)
	if path == "":
		return
	
	var rel_path: String = _get_relative_path(path)
	var is_dir: bool = root_dir.dir_exists(rel_path)
	if is_dir:
		item.collapsed = not item.collapsed
	
	item_activated.emit(path, is_dir)


func _on_item_collapsed(item: TreeItem) -> void:
	var path: String = item.get_metadata(0)
	_collapsed_paths[path] = item.collapsed
	item_collapsed_changed.emit(path, item.collapsed)


func _get_drag_data_fw(_at_position: Vector2) -> Variant:
	if is_read_only:
		return null
	
	var selected_items: Array = _get_selected_items()
	if selected_items.is_empty():
		return null
	
	var paths: Array[String] = []
	for item: TreeItem in selected_items:
		var path: String = item.get_metadata(0)
		if can_drag_item(path):
			paths.append(path)
	
	if paths.is_empty():
		return null
	
	var preview: Label = Label.new()
	if paths.size() == 1:
		preview.text = paths[0].get_file()
	else:
		preview.text = "%d items" % paths.size()
	tree.set_drag_preview(preview)
	
	return {"type": "files", "paths": paths}


func _can_drop_data_fw(at_position: Vector2, data: Variant) -> bool:
	if is_read_only or not root_dir:
		return false
	
	if typeof(data) != TYPE_DICTIONARY:
		return false
	
	if not data.has("type") or data["type"] != "files":
		return false
	
	var item: TreeItem = tree.get_item_at_position(at_position)
	if not item:
		return false
	
	var target_path: String = item.get_metadata(0)
	var source_paths: Array = data["paths"]
	
	var rel_target: String = _get_relative_path(target_path)
	if not root_dir.dir_exists(rel_target):
		target_path = target_path.get_base_dir()
		rel_target = _get_relative_path(target_path)
	
	if not can_drop_item("", target_path):
		return false
	
	for source_path: String in source_paths:
		if target_path == source_path:
			return false
		if target_path.begins_with(source_path + "/"):
			return false
		if target_path == source_path.get_base_dir():
			return false
		if not can_drop_item(source_path, target_path):
			return false
	
	return true


func _drop_data_fw(at_position: Vector2, data: Variant) -> void:
	if is_read_only or not root_dir:
		return
	
	var item: TreeItem = tree.get_item_at_position(at_position)
	if not item:
		return
	
	var target_path: String = item.get_metadata(0)
	var source_paths: Array = data["paths"]
	
	var rel_target: String = _get_relative_path(target_path)
	if not root_dir.dir_exists(rel_target):
		target_path = target_path.get_base_dir()
		rel_target = _get_relative_path(target_path)
	
	for source_path: String in source_paths:
		if not can_drag_item(source_path) or not can_drop_item(source_path, target_path):
			continue
		
		var file_name: String = source_path.get_file()
		var new_path: String = target_path.path_join(file_name)
		
		var rel_source: String = _get_relative_path(source_path)
		var rel_new: String = _get_relative_path(new_path)
		
		if root_dir.rename_path(rel_source, rel_new):
			if root_dir.dir_exists(rel_new):
				update_collapsed_paths_after_rename(source_path, new_path)
			drag_drop_completed.emit(source_path, new_path)
	
	refresh()


func can_drag_item(path: String) -> bool:
	if not root_dir:
		return false
	if path == root_dir.get_path():
		return false
	if path.get_extension() in Nebula.get_reserved_extensions():
		return false
	return true


func can_drop_item(_source_path: String, target_path: String) -> bool:
	if not root_dir:
		return false
	if target_path == root_dir.get_path():
		return true
	if target_path.get_extension() in Nebula.get_reserved_extensions():
		return false
	return true


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


func update_collapsed_paths_after_rename(old_path: String, new_path: String) -> void:
	var paths_to_update: Array = []
	
	for path: String in _collapsed_paths.keys():
		if path == old_path or path.begins_with(old_path + "/"):
			paths_to_update.append(path)
	
	for path: String in paths_to_update:
		var collapsed_state: bool = _collapsed_paths[path]
		_collapsed_paths.erase(path)
		
		var updated_path: String = new_path + path.substr(old_path.length())
		_collapsed_paths[updated_path] = collapsed_state
