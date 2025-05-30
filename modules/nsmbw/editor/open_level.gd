extends Node

@export var open_level_dialog: FileDialog
@export var viewport_tab_container: TabContainer

const LEVEL_CANVAS: PackedScene = preload("uid://dc2ti7s0l6qia")
const LEVEL_ICON: Texture2D = preload("uid://b7k2jj1jlcxuc")

func _ready() -> void:
	open_level_dialog.root_subfolder = NSMBW.stage_folder
	open_level_dialog.current_dir = NSMBW.stage_folder
	
	if not get_tree().root.has_user_signal(&"nsmbw_open_level"):
		get_tree().root.add_user_signal(&"nsmbw_open_level")
	
	get_tree().root.connect(&"nsmbw_open_level", show_open_level_dialog)


func show_open_level_dialog() -> void:
	open_level_dialog.show()


func parse_level(path: String) -> void:
	
	var level_opened: bool = false
	var file_name: String = path.get_file()
	var level_index: int = 0
	
	for i: int in range(viewport_tab_container.get_tab_count()):
		if viewport_tab_container.get_tab_title(i) == file_name:
			level_opened = true
			level_index = i
			break
	
	if not level_opened:
		var arc: ARC = ARC.open(path)
		var level: NSMBWLevel = NSMBWLevel.open(arc)
		var level_canvas: Node = LEVEL_CANVAS.duplicate(true).instantiate()
		
		level_canvas.level = level
		level_canvas.name = path.get_file().replace(".", ",")
		
		viewport_tab_container.add_child(level_canvas)
		
		if viewport_tab_container.get_child(0).name == "<empty>":
			viewport_tab_container.remove_child(viewport_tab_container.get_child(0))
		
		var tab_bar: TabBar = viewport_tab_container.get_tab_bar()
		tab_bar.set_tab_icon(tab_bar.tab_count - 1, LEVEL_ICON)
		tab_bar.set_tab_title(tab_bar.tab_count - 1, level_canvas.name.replace(",", "."))
		
		viewport_tab_container.current_tab = tab_bar.tab_count - 1
	else:
		viewport_tab_container.current_tab = level_index
