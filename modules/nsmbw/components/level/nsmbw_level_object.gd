class_name NSMBWLevelObject
extends GraphElement

enum ScaleMode {
	REPEAT
}

@export var tileset: NSMBWTileset
@export var object_position: Vector2i = Vector2i(0, 0)
@export var object_size: Vector2i = Vector2i(1, 1)
@export var object_data: Array
@export var texture: Texture2D

var scaling_mode: ScaleMode


func _ready() -> void:
	position_offset = object_position * 16
	
	if object_data and object_data.get(0) and object_data.get(0).size() > 0:
		size = Vector2i(object_data[0].size(), object_data.size()) * 16
	
	
	
	process_mode = Node.PROCESS_MODE_DISABLED
