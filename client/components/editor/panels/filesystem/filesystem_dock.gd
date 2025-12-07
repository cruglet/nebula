class_name NebulaFilesystemDock
extends Panel

signal file_open_request(path: String)

@export var filesystem_tree: Tree
@export var empty_filesystem_label: Label

var root: String = "":
	set(r):
		root = r
		refresh()


func _ready() -> void:
	refresh()
	filesystem_tree.item_activated.connect(_on_item_activated)


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
