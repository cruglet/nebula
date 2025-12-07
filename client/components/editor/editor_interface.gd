class_name NebulaEditorInterface
extends Control

@warning_ignore("unused_signal")
signal main_tab_close_request

@export_group("Internal")
@export_subgroup("Main Dock")
@export var _main_dock_node: Control
@export var _main_dock_tab_container: TabContainer
@export var _secondary_dock_node: Control
@export var _secondary_dock_tab_container: TabContainer
@export var _menu_bar: MenuBar


var _main_dock: NebulaEditorDock
var _secondary_dock: NebulaEditorDock
var _menu_items: Dictionary#[StringName, Dictionary]


func _ready() -> void:
	get_main_dock()
	get_secondary_dock()


func create_menu_item(menu: StringName, item: String, callback: Callable) -> void:
	var items: Dictionary = _menu_items.get_or_add(menu, {})
	items.set(item, callback)
	_menu_items.set(menu, items)


@warning_ignore("unused_parameter")
func remove_menu_item(menu: StringName, item: String = "") -> void:
	pass


func reload_menu_items() -> void:
	for item: StringName in _menu_items:
		var menu: PopupMenu = _menu_bar.find_child(item)
		if not is_instance_valid(menu):
			menu = PopupMenu.new()
			menu.name = item
			_menu_bar.add_child(menu)
		
		menu.clear(true)
		
		var i: int = 0
		var options: Dictionary = _menu_items.get(item, {})
		for option: String in options:
			menu.add_item(option, i)
			menu.index_pressed.connect(options.get(option).unbind(1))
			i += 1


func get_main_dock() -> NebulaEditorDock:
	if !_main_dock:
		_main_dock = NebulaEditorDock.new(_main_dock_node, _main_dock_tab_container)
	return _main_dock


func get_secondary_dock() -> NebulaEditorDock:
	if !_secondary_dock:
		_secondary_dock = NebulaEditorDock.new(_secondary_dock_node, _secondary_dock_tab_container)
	return _secondary_dock


class NebulaEditorDock:
	signal tab_close_request
	
	var auto_close_tabs: bool = true
	var tab_close_display_policy: TabBar.CloseButtonDisplayPolicy = TabBar.CLOSE_BUTTON_SHOW_NEVER
	
	var _ref: Control
	var _tc: TabContainer
	var _empty_scene: Node
	
	var _tab_to_close: int = -1
	var _unsaved_tabs: Dictionary = {}
	
	
	func _init(ref: Control, tc: TabContainer) -> void:
		_ref = ref
		_tc = tc
		tc.get_tab_bar().tab_close_pressed.connect(_on_tab_close_request)
		_refresh()
	
	
	func add_scene(scene: PackedScene, name: String = "", refocus: bool = true) -> Node:
		var existing: int = -1
		if name != "":
			existing = _find_tab_by_title(name)
			if existing >= 0:
				_tc.current_tab = existing
				return null
		
		var node: Node = scene.instantiate()
		return add_node(node, name, refocus)
	
	
	
	func add_node(node: Node, name: String = "", refocus: bool = true) -> Node:
		if name != "":
			var existing: int = _find_tab_by_title(name)
			if existing >= 0:
				_tc.current_tab = existing
				return null
		
		# ensure a unique internal node name so '.' in titles doesn't break things
		node.name = "_tab_%s" % str(node.get_instance_id())
		_tc.add_child(node)
		
		if _tc.get_child(0) == _empty_scene:
			_tc.remove_child(_empty_scene)
		
		var idx: int = _tc.get_child_count() - 1
		
		if name == "":
			name = node.name
		_tc.set_tab_title(idx, name)
		
		if node.has_signal(&"unsaved"):
			node.unsaved.connect(_on_node_unsaved.bind(node))
		
		if node.has_signal(&"instance_renamed"):
			node.instance_renamed.connect(_on_node_renamed.bind(node))
		
		if refocus:
			_tc.current_tab = idx
		
		_refresh()
		return node
	
	
	
	func set_empty_scene(scene: PackedScene, name: String = "<empty>") -> void:
		_empty_scene = scene.instantiate()
		_empty_scene.name = "<empty>"
		_empty_scene.set_meta("__tab_title", name)
		_refresh()
	
	
	func accept_close_request() -> void:
		if _tab_to_close >= 0:
			var child: Node = _tc.get_child(_tab_to_close)
			if child:
				_unsaved_tabs.erase(child)
				child.queue_free()
		_tab_to_close = -1
		_refresh()
	
	
	func remove_active() -> Node:
		var node: Node = get_active_node()
		_tc.remove_child(node)
		_unsaved_tabs.erase(node)
		return node
	
	
	func replace_active(node: Node, name: String) -> Node:
		var last_active: Node = remove_active()
		add_node(node, name, true)
		return last_active
	
	
	func get_tab_count(include_empty: bool = true) -> int:
		var count: int = 0
		var total: int = _tc.get_child_count()
		for i: int in total:
			var child: Node = _tc.get_child(i)
			if not include_empty and child == _empty_scene:
				continue
			if child.is_queued_for_deletion():
				continue
			count += 1
		return count
	
	
	func get_active_node() -> Node:
		return _tc.get_child(_tc.current_tab)
	
	
	func _on_tab_close_request(tab: int) -> void:
		if _tc.get_child(tab) == _empty_scene:
			return
		
		_tab_to_close = tab
		
		if auto_close_tabs:
			accept_close_request()
		
		tab_close_request.emit()
	
	
	func _on_node_unsaved(status: bool, node: Node) -> void:
		_unsaved_tabs[node] = status
		_update_tab_title(node)
	
	
	func _on_node_renamed(new_name: String, node: Node) -> void:
		var idx: int = node.get_index()
		if idx < 0 or idx >= _tc.get_tab_count():
			return
		_tc.set_tab_title(idx, new_name)
	
	
	func _update_tab_title(node: Node) -> void:
		var idx: int = node.get_index()
		if idx < 0 or idx >= _tc.get_tab_count():
			return
		
		var title: String = _tc.get_tab_title(idx)
		var is_unsaved: bool = _unsaved_tabs.get(node, false)
		
		if is_unsaved and not title.begins_with("*"):
			_tc.set_tab_title(idx, "*" + title)
		elif not is_unsaved and title.begins_with("*"):
			_tc.set_tab_title(idx, title.substr(1))
	
	
	func _refresh() -> void:
		var children: Array[Node] = []
		var total: int = _tc.get_child_count()
		_tc.get_tab_bar().tab_close_display_policy = tab_close_display_policy
		
		for i: int in total:
			var n: Node = _tc.get_child(i)
			if not n.is_queued_for_deletion():
				children.append(n)
		
		if children.size() == 0 and _empty_scene:
			_tc.add_child(_empty_scene)
			_tc.get_tab_bar().tab_close_display_policy = TabBar.CLOSE_BUTTON_SHOW_NEVER
			_ref.show()
		
		elif children.size() == 0 and not _empty_scene:
			_ref.hide()
		
		elif children.size() > 1 and _empty_scene in _tc.get_children():
			_tc.remove_child(_empty_scene)
			_ref.show()
		
		if get_tab_count() > 0:
			_ref.show()
		else:
			_ref.hide()
	
	
	func _find_tab_by_title(title: String) -> int:
		var total: int = _tc.get_child_count()
		for i: int in total:
			var tab_title: String = _tc.get_tab_title(i)
			if tab_title == title:
				return i
		return -1


func _on_button_pressed() -> void:
	NebulaInfoWindow.get_instance().show()
