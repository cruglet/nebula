class_name NebulaEditorInterface
extends Control

signal main_tab_close_request

@export_group("Internal")
@export_subgroup("Main Dock")
@export var _main_dock_node: Control
@export var _main_dock_tab_container: TabContainer


var _main_dock: NebulaEditorMainDock
var _menu_items: Dictionary[StringName, Dictionary]


func create_menu_item(menu: StringName, item: String, callback: Callable) -> void:
	pass


func remove_menu_item(menu: StringName, item: String = "") -> void:
	pass


func get_main_dock() -> NebulaEditorMainDock:
	if !_main_dock:
		_main_dock = NebulaEditorMainDock.new(_main_dock_node, _main_dock_tab_container)
	return _main_dock


func get_side_dock() -> void:
	pass


class NebulaEditorMainDock:
	signal tab_close_request
	
	var auto_close_tabs: bool = true
	
	var _ref: Control
	var _tc: TabContainer
	var _empty_scene: PackedScene
	
	var _tab_to_close: int = -1
	
	
	func _init(ref: Control, tc: TabContainer) -> void:
		_ref = ref
		_tc = tc
		tc.get_tab_bar().tab_close_pressed.connect(_on_tab_close_request)
	
	
	func add_scene(scene: PackedScene, refocus: bool = true) -> void:
		var node: Node = scene.duplicate().instantiate()
		add_node(node, refocus)
	
	
	func add_node(node: Node, refocus: bool = true) -> void:
		_tc.add_child(node)
		
		if _tc.get_child(0) == _empty_scene:
			_tc.get_child(0).free()
		
		if refocus:
			_tc.current_tab = _tc.get_child_count()
	
	
	func set_empty_scene(scene: PackedScene) -> void:
		_empty_scene = scene
	
	
	func accept_close_request() -> void:
		if _tab_to_close >= 0:
			_tc.get_child(_tab_to_close).queue_free()
		_tab_to_close = -1
	
	
	func _on_tab_close_request(tab: int) -> void:
		_tab_to_close = tab
		
		if auto_close_tabs:
			accept_close_request()
		
		tab_close_request.emit()

class NebulaEditorSideDock:
	pass
