class_name NSMBWPath extends Object

var points: Array[NSMBWPathPoint]
var id: int
var loops: bool

const OFFSET: int = 8
const SUB_OFFSET: int = 16

static func from_blocks(path_block: PackedByteArray, path_node_block: PackedByteArray) -> Array[NSMBWPath]:
	
	var paths: Array[NSMBWPath] = []
	
	var path_idx: int = 0
	while path_idx + OFFSET <= path_block.size():
		var chunk: PackedByteArray = path_block.slice(path_idx, path_idx + OFFSET)
		var path: NSMBWPath = NSMBWPath.new()
		
		path.id = chunk[0]
		var count: int = Packer.decode_u16_be(chunk, 4)
		
		path.loops = bool(chunk[6])
		
		var path_points: Array[NSMBWPathPoint] = []
		for i: int in range(count):
			var current_offset: int = i * SUB_OFFSET
			if current_offset + SUB_OFFSET > path_node_block.size():
				break
				
			var node_chunk: PackedByteArray = path_node_block.slice(current_offset, current_offset + SUB_OFFSET)
			var point: NSMBWPathPoint = NSMBWPathPoint.new()
			point.position.x = Packer.decode_u16_be(node_chunk, 0)
			point.position.y = Packer.decode_u16_be(node_chunk, 2)
			point.speed = Packer.decode_float_be(node_chunk, 4)
			point.acceleration = Packer.decode_float_be(node_chunk, 8)
			point.delay = Packer.decode_s16_be(node_chunk, 12)
			
			path_points.append(point)
		
		path.points = path_points
		paths.append(path)
		
		path_idx += OFFSET
	
	return paths
