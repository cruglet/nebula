class_name NSMBWSprite extends Object

var position: Vector2i
var type: int
var data: int

const OFFSET: int = 16

static func from_blocks(block_data: PackedByteArray) -> Array[NSMBWSprite]:
	var sprites: Array[NSMBWSprite]
	for i: int in range(block_data.size() / OFFSET):
		var pos: int = i * OFFSET
		var chunk: PackedByteArray = block_data.slice(pos, pos + OFFSET)
		var sprite: NSMBWSprite = NSMBWSprite.new()
		
		sprite.type = Packer.decode_u16_be(chunk, 0)
		sprite.position.x = Packer.decode_u16_be(chunk, 2)
		sprite.position.y = Packer.decode_u16_be(chunk, 4)
		sprite.data = Packer.decode_u64_be(chunk, 6)
		
		sprites.append(sprite)
	return sprites

func _to_string() -> String:
	return "type: %s\nposition: %s\ndata: %s" % [type, position, data]
