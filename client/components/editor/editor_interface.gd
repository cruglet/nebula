class_name NebulaEditorInterface
extends Control

const VALID_HOVER_COLOR: Color = Color(0.3, 0.5, 1.0, 0.4)
const VALID_COLOR: Color = Color(0.5, 0.5, 0.5, 0.2)
const INVALID_COLOR: Color = Color(1.0, 0.3, 0.3, 0.15)
const SOURCE_COLOR: Color = Color(0.466, 0.466, 0.466, 0.1)

@warning_ignore("unused_signal")
signal main_tab_close_request

@export_group("Internal")
@export var _menu_bar: MenuBar

@export_subgroup("Main Dock")
@export var _main_dock_node: Control
@export var _main_dock_tab_container: TabContainer

@export_subgroup("Top Right Dock")
@export var _top_right_dock_node: Control
@export var _top_right_dock_tab_container: TabContainer

@export_subgroup("Bottom Right Dock")
@export var _bottom_right_dock_node: Control
@export var _bottom_right_dock_tab_container: TabContainer


var _main_dock: NebulaEditorDock
var _top_right_dock: NebulaEditorDock
var _bottom_right_dock: NebulaEditorDock
var _menu_items: Dictionary#[StringName, Dictionary]

var _dragging_tab: Node = null
var _dragging_from_dock: NebulaEditorDock = null
var _drop_indicators: Dictionary = {}

var _potential_drag_node: Node = null
var _potential_drag_dock: NebulaEditorDock = null
var _drag_start_pos: Vector2 = Vector2.ZERO
var _hidden_docks_during_drag: Array[NebulaEditorDock] = []
var _drag_indicators_shown: bool = false


func _ready() -> void:
	get_main_dock()
	get_top_right_dock()
	get_bottom_right_dock()


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
		_main_dock = NebulaEditorDock.new(_main_dock_node, _main_dock_tab_container, self)
	return _main_dock


func get_top_right_dock() -> NebulaEditorDock:
	if !_top_right_dock:
		_top_right_dock = NebulaEditorDock.new(_top_right_dock_node, _top_right_dock_tab_container, self)
	return _top_right_dock


func get_bottom_right_dock() -> NebulaEditorDock:
	if !_bottom_right_dock:
		_bottom_right_dock = NebulaEditorDock.new(_bottom_right_dock_node, _bottom_right_dock_tab_container, self)
	return _bottom_right_dock


func done() -> void:
	get_main_dock()._refresh()
	get_top_right_dock()._refresh()
	get_bottom_right_dock()._refresh()


func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		var mb: InputEventMouseButton = event as InputEventMouseButton
		if mb.button_index == MOUSE_BUTTON_LEFT and not mb.pressed:
			if _dragging_tab and _dragging_from_dock:
				var mouse_pos: Vector2 = get_global_mouse_position()
				var target_dock: NebulaEditorDock = _get_dock_at_position(mouse_pos)
				
				if target_dock and target_dock != _dragging_from_dock and not target_dock.fixed and _drag_indicators_shown:
					_dragging_from_dock.switch_dock(target_dock, _dragging_tab)
				
				_end_drag()
			
			_potential_drag_node = null
			_potential_drag_dock = null
	
	elif event is InputEventMouseMotion:
		if _potential_drag_node and not _dragging_tab:
			var current_pos: Vector2 = get_global_mouse_position()
			var distance: float = _drag_start_pos.distance_to(current_pos)
			if distance > 10.0:
				_start_drag(_potential_drag_node, _potential_drag_dock)
		
		if _dragging_tab:
			var mouse_pos: Vector2 = get_global_mouse_position()
			_check_if_off_tabbar(mouse_pos)
			if _drag_indicators_shown:
				_update_drop_indicators(mouse_pos)


func _check_if_off_tabbar(mouse_pos: Vector2) -> void:
	if not _dragging_from_dock:
		return
	
	var tab_bar: TabBar = _dragging_from_dock._tc.get_tab_bar()
	var tab_bar_rect: Rect2 = Rect2(tab_bar.global_position, tab_bar.size)
	var is_over_tabbar: bool = tab_bar_rect.has_point(mouse_pos)
	
	if not is_over_tabbar and not _drag_indicators_shown:
		_show_all_dock_indicators()
		_drag_indicators_shown = true


func _get_dock_at_position(pos: Vector2) -> NebulaEditorDock:
	var docks: Array[NebulaEditorDock] = [_main_dock, _top_right_dock, _bottom_right_dock]
	
	for dock: NebulaEditorDock in docks:
		if dock:
			var rect: Rect2 = Rect2(dock._ref.global_position, dock._ref.size)
			if rect.has_point(pos):
				return dock
	
	return null


func _update_drop_indicators(mouse_pos: Vector2) -> void:
	var target_dock: NebulaEditorDock = _get_dock_at_position(mouse_pos)
	
	for dock: NebulaEditorDock in [_main_dock, _top_right_dock, _bottom_right_dock]:
		if dock in _drop_indicators:
			var indicator: ColorRect = _drop_indicators[dock]
			if is_instance_valid(indicator):
				indicator.position = dock._ref.global_position - global_position
				indicator.size = dock._ref.size
				
				if dock == target_dock and not dock.fixed and dock != _dragging_from_dock:
					indicator.color = VALID_HOVER_COLOR
				elif dock.fixed:
					indicator.color = INVALID_COLOR
				elif dock == _dragging_from_dock:
					indicator.color = SOURCE_COLOR
				else:
					indicator.color = VALID_COLOR


func _update_cursor(mouse_pos: Vector2) -> void:
	var target_dock: NebulaEditorDock = _get_dock_at_position(mouse_pos)
	
	if target_dock and target_dock != _dragging_from_dock and not target_dock.fixed:
		Input.set_default_cursor_shape(Input.CURSOR_DRAG)
	else:
		Input.set_default_cursor_shape(Input.CURSOR_FORBIDDEN)


func _clear_drop_indicators() -> void:
	for dock: NebulaEditorDock in _drop_indicators:
		var indicator: ColorRect = _drop_indicators[dock]
		if is_instance_valid(indicator):
			indicator.queue_free()
	_drop_indicators.clear()


func _show_all_dock_indicators() -> void:
	_clear_drop_indicators()
	
	_hidden_docks_during_drag.clear()
	for dock: NebulaEditorDock in [_main_dock, _top_right_dock, _bottom_right_dock]:
		if dock and not dock._ref.visible:
			dock._ref.show()
			_hidden_docks_during_drag.append(dock)
	
	for dock: NebulaEditorDock in [_main_dock, _top_right_dock, _bottom_right_dock]:
		if dock:
			var indicator: ColorRect = ColorRect.new()
			if dock.fixed:
				indicator.color = Color(1.0, 0.3, 0.3, 0.15)
			else:
				indicator.color = Color(0.5, 0.5, 0.5, 0.2)
			
			indicator.position = dock._ref.global_position - global_position
			indicator.size = dock._ref.size
			indicator.mouse_filter = Control.MOUSE_FILTER_IGNORE
			indicator.z_index = 100
			add_child(indicator)
			_drop_indicators[dock] = indicator


func _start_potential_drag(node: Node, dock: NebulaEditorDock, start_pos: Vector2) -> void:
	if dock.fixed:
		return
	
	_potential_drag_node = node
	_potential_drag_dock = dock
	_drag_start_pos = start_pos


func _start_drag(node: Node, dock: NebulaEditorDock) -> void:
	if dock.fixed:
		return
	
	_dragging_tab = node
	_dragging_from_dock = dock
	_potential_drag_node = null
	_potential_drag_dock = null
	_drag_indicators_shown = false


func _end_drag() -> void:
	_dragging_tab = null
	_dragging_from_dock = null
	_drag_indicators_shown = false
	_clear_drop_indicators()
	mouse_default_cursor_shape = Control.CURSOR_ARROW
	
	for dock: NebulaEditorDock in _hidden_docks_during_drag:
		if dock:
			dock._refresh()
	_hidden_docks_during_drag.clear()


func _on_button_pressed() -> void:
	NebulaInfoWindow.in_dashboard = false
	NebulaInfoWindow.get_instance().show()


class NebulaEditorDock:
	signal tab_close_request
	signal tab_changed
	
	var auto_close_tabs: bool = true
	var hide_tabs_on_hidden: bool = true
	var hide_on_empty: bool = true
	var tab_close_display_policy: TabBar.CloseButtonDisplayPolicy = TabBar.CLOSE_BUTTON_SHOW_NEVER
	var fixed: bool = false
	
	var _ref: Control
	var _tc: TabContainer
	var _empty_scene: Node
	var _queued_active: Node
	var _interface: NebulaEditorInterface
	
	var _tab_to_close: int = -1
	var _unsaved_tabs: Dictionary = {}
	var _hidden_tabs: Dictionary = {}
	var _bound_nodes: Dictionary[Node, StringName]
	
	
	func _init(ref: Control, tc: TabContainer, interface: NebulaEditorInterface) -> void:
		_ref = ref
		_tc = tc
		_interface = interface
		tc.get_tab_bar().tab_close_pressed.connect(_on_tab_close_request)
		tc.get_tab_bar().tab_changed.connect(_on_tab_changed)
		tc.get_tab_bar().gui_input.connect(_on_tab_bar_input)
		_refresh()
	
	
	func add_scene(scene: PackedScene, name: String = "", refocus: bool = true) -> Node:
		var existing: int = -1
		if name != "":
			existing = _find_tab_by_title(name)
			if existing >= 0:
				_tc.current_tab = existing
				return null
		
		var node: Node = scene.duplicate().instantiate()
		return add_node(node, name, refocus)
	
	
	func add_node(node: Node, name: String = "", refocus: bool = true) -> Node:
		if name != "":
			var existing: int = _find_tab_by_title(name)
			if existing >= 0:
				_tc.current_tab = existing
				return null
		
		node.name = "_tab_%s" % str(node.get_instance_id())
		node.set_meta(&"dock", self)
		_tc.add_child(node)
		
		if _tc.get_child(0) == _empty_scene:
			_tc.remove_child(_empty_scene)
		
		var idx: int = _tc.get_child_count() - 1
		
		if name == "":
			name = node.name
		_tc.set_tab_title(idx, name)
		
		if node is Control:
			node.clip_contents = true
		
		if node.has_signal(&"unsaved"):
			node.unsaved.connect(_on_node_unsaved.bind(node))
		
		if node.has_signal(&"instance_renamed"):
			node.instance_renamed.connect(_on_node_renamed.bind(node))
		
		if refocus:
			_queued_active = node
			_tc.current_tab = idx
			_tc.get_tree().process_frame.connect(_clear_queued_active, CONNECT_ONE_SHOT)
		
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
				_hidden_tabs.erase(child)
				child.queue_free()
		_tab_to_close = -1
		_refresh()
	
	
	func remove_active() -> Node:
		var node: Node = get_active_node()
		_tc.remove_child(node)
		_unsaved_tabs.erase(node)
		_hidden_tabs.erase(node)
		return node
	
	
	func remove_node(node: Node) -> Node:
		_tc.remove_child(node)
		_unsaved_tabs.erase(node)
		_hidden_tabs.erase(node)
		return node
	
	
	func replace_active(node: Node, name: String) -> Node:
		var last_active: Node = remove_active()
		add_node(node, name, true)
		return last_active
	
	
	func hide_node(node: Node) -> void:
		if node in _tc.get_children():
			_hidden_tabs[node] = true
			_tc.set_tab_hidden(node.get_index(), true)
			_refresh()
	
	
	func show_node(node: Node) -> void:
		if node in _tc.get_children():
			_hidden_tabs.erase(node)
			_tc.set_tab_hidden(node.get_index(), false)
			_refresh()
	
	
	func bind_node(node: Node, type: StringName) -> void:
		_bound_nodes.set(node, type)
		_refresh()
	
	
	func unbind_node(node: Node) -> void:
		_bound_nodes.erase(node)
	
	
	func switch_dock(target_dock: NebulaEditorDock, node: Node = null) -> void:
		if target_dock.fixed or fixed:
			return
		
		var transfer_node: Node = node if node else get_active_node()
		if not transfer_node or transfer_node == _empty_scene:
			return
		
		var tab_idx: int = transfer_node.get_index()
		var tab_title: String = _tc.get_tab_title(tab_idx)
		var is_unsaved: bool = _unsaved_tabs.get(transfer_node, false)
		var is_hidden: bool = _hidden_tabs.get(transfer_node, false)
		var bound_type: StringName = _bound_nodes.get(transfer_node, &"")
		
		_tc.remove_child(transfer_node)
		_unsaved_tabs.erase(transfer_node)
		_hidden_tabs.erase(transfer_node)
		_bound_nodes.erase(transfer_node)
		
		transfer_node.set_meta(&"dock", target_dock)
		target_dock.add_node(transfer_node, tab_title, true)
		
		if is_unsaved:
			target_dock._unsaved_tabs[transfer_node] = true
			target_dock._update_tab_title(transfer_node)
		
		if is_hidden:
			target_dock.hide_node(transfer_node)
		
		if bound_type != &"":
			target_dock.bind_node(transfer_node, bound_type)
		
		_refresh()
		target_dock._refresh()
	
	
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
	
	
	func get_visible_tab_count(include_empty: bool = true) -> int:
		var count: int = 0
		var total: int = _tc.get_child_count()
		for i: int in total:
			var child: Node = _tc.get_child(i)
			if not include_empty and child == _empty_scene:
				continue
			if child.is_queued_for_deletion():
				continue
			if _hidden_tabs.get(child, false):
				continue
			count += 1
		return count
	
	
	func get_active_node() -> Node:
		if _queued_active:
			return _queued_active
		if _tc.get_child_count() == 0:
			return null
		var current_node: Node = _tc.get_child(_tc.current_tab)
		return null if current_node.is_queued_for_deletion() else current_node
	
	
	func _clear_queued_active() -> void:
		_queued_active = null
	
	
	func _on_tab_close_request(tab: int) -> void:
		if _tc.get_child(tab) == _empty_scene:
			return
		
		_tab_to_close = tab
		
		if auto_close_tabs:
			accept_close_request()
		
		tab_close_request.emit()
	
	
	func _on_tab_changed(tab: int) -> void:
		_queued_active = null
		tab_changed.emit(tab)
		_refresh()
	
	
	func _on_node_unsaved(status: bool, node: Node) -> void:
		_unsaved_tabs[node] = status
		_update_tab_title(node)
	
	
	func _on_node_renamed(new_name: String, node: Node) -> void:
		var idx: int = node.get_index()
		if idx < 0 or idx >= _tc.get_tab_count():
			return
		_tc.set_tab_title(idx, new_name)
	
	
	func _on_tab_bar_input(event: InputEvent) -> void:
		if fixed:
			return
		
		if event is InputEventMouseButton:
			var mb: InputEventMouseButton = event as InputEventMouseButton
			if mb.button_index == MOUSE_BUTTON_LEFT and mb.pressed:
				var tab_idx: int = _tc.get_tab_bar().get_tab_idx_at_point(mb.position)
				
				if tab_idx >= 0 and tab_idx < _tc.get_child_count():
					var node: Node = _tc.get_child(tab_idx)
					if node != _empty_scene:
						var global_pos: Vector2 = _tc.get_tab_bar().global_position + mb.position
						_interface._start_potential_drag(node, self, global_pos)
	
	
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
		
		var active_node: Node = get_active_node()
		
		for node: Node in _bound_nodes:
			var bound_class: StringName = _bound_nodes.get(node)
			
			var should_show: bool = active_node and active_node.is_class(bound_class)
			
			if node.has_meta(&"dock"):
				var dock: Variant = node.get_meta(&"dock")
				if should_show:
					dock.show_node(node)
				else:
					dock.hide_node(node)
			else:
				node.visible = should_show
		
		if hide_on_empty:
			var visible_count: int = get_visible_tab_count(false)
			if visible_count == 0:
				_ref.hide()
			else:
				_ref.show()
		else:
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
