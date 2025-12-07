class_name NebulaInfoWindow
extends NebulaWindow

const INFO_WINDOW: PackedScene = preload("uid://doh2gawt7opik")

static var _ref: NebulaInfoWindow


@export var version_label: Label


static func get_instance() -> NebulaInfoWindow:
	if not is_instance_valid(_ref):
		_ref = INFO_WINDOW.instantiate()
		_ref.version_label.text = "v" + Nebula.get_version_string()
		Singleton.add_child(_ref)
	return _ref
