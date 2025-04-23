class_name NSMBWRegion extends Object

var position: Vector2i
var size: Vector2i
var id: int

static func from_blocks(block_data: PackedByteArray) -> Array[NSMBWRegion]:
	const OFFSET: int = 12
	
	var regions: Array[NSMBWRegion] = []
	
	var region_idx: int = 0
	while region_idx + OFFSET <= block_data.size():
		var chunk: PackedByteArray = block_data.slice(region_idx, region_idx + OFFSET)
		var region: NSMBWRegion = NSMBWRegion.new()
		
		region.position.x = Packer.decode_u16_be(chunk, 0)
		region.position.y = Packer.decode_u16_be(chunk, 2)
		region.size.x = Packer.decode_u16_be(chunk, 4)
		region.size.y = Packer.decode_u16_be(chunk, 6)
		region.id = chunk[8]
		
		regions.append(region)
		region_idx += OFFSET
	
	return regions
