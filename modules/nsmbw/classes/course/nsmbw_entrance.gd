class_name NSMBWEntrance extends Object

var position: Vector2i
var id: int
var destination_area: int
var destination_entrance: int
var type: int # TODO: Make enum
#var zone: int
var active_layer: int
var path: int
var exit_to_map: bool

const OFFSET: int = 20

static func from_blocks(block_data: PackedByteArray) -> Array[NSMBWEntrance]:
	var entrances: Array[NSMBWEntrance]
	for i: int in range(block_data.size() / OFFSET):
		var pos: int = i * OFFSET
		var chunk: PackedByteArray = block_data.slice(pos, pos + OFFSET)
		var entrance: NSMBWEntrance = NSMBWEntrance.new()
		
		entrance.position.x = Packer.decode_u16_be(chunk, 0)
		entrance.position.y = Packer.decode_u16_be(chunk, 2)
		# 5-8
		entrance.id = chunk[8]
		entrance.destination_area = chunk[9]
		entrance.destination_entrance = chunk[10]
		entrance.type = chunk[11]
		#entrance.zone = chunk[13]
		entrance.active_layer = chunk[14]
		entrance.path = chunk[15]
		entrance.exit_to_map = bool(chunk[18])
	
		entrances.append(entrance)
	return entrances
