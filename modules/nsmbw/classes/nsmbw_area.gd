class_name NSMBWArea extends Resource

var time_limit: int
var start_entrance: int
var can_wrap: bool
var is_credits: bool
var is_ambush: bool
var a_events: int
var b_events: int

var tilesets: Array[NSMBWTileset]
var sprites: Array[NSMBWSprite]
var entrances: Array[NSMBWEntrance]
var zones: Array[NSMBWZone]
var paths: Array[NSMBWPath]
var regions: Array[NSMBWRegion]
var tile_layers: Array[NSMBWTileLayer]

const BLOCK_SIZE: int = 14
const BLOCK_METADATA_SIZE: int = 8;

static func parse(area: Dictionary, level: NSMBWLevel) -> NSMBWArea:
	
	var blocks: Array[PackedByteArray] = _parse_blocks(area.data)
	var new_area: NSMBWArea = NSMBWArea.new()
	
	new_area._read_area_settings(blocks[1])
	new_area.tilesets = NSMBWTileset.from_blocks(blocks[0], level)
	new_area.sprites = NSMBWSprite.from_blocks(blocks[7])
	new_area.entrances = NSMBWEntrance.from_blocks(blocks[6])
	new_area.zones = NSMBWZone.from_blocks(blocks[9], blocks[2], blocks[4], blocks[5])
	new_area.paths = NSMBWPath.from_blocks(blocks[12], blocks[13])
	new_area.regions = NSMBWRegion.from_blocks(blocks[10])
	new_area.tile_layers = NSMBWTileLayer.from_blocks(
		area.get(&"layer0", []), 
		area.get(&"layer1", []), 
		area.get(&"layer2", [])
	)

	return new_area


static func _parse_blocks(area_data: PackedByteArray) -> Array[PackedByteArray]:
	if area_data.size() < BLOCK_SIZE * BLOCK_METADATA_SIZE:
		printerr("File to small to contain metadata for all blocks!")
		return []
	
	var blocks: Array[PackedByteArray]
	
	for i: int in range(0, BLOCK_SIZE):
		var meta_offset: int = i * BLOCK_METADATA_SIZE
		var block_offset: int = Packer.decode_u32_be(area_data, meta_offset)
		var block_size: int = Packer.decode_u32_be(area_data, meta_offset + 4)
		
		if block_size == 0:
			blocks.append(PackedByteArray())
		elif block_offset + block_size <= area_data.size():
			blocks.append(area_data.slice(block_offset, block_offset + block_size))
		else:
			printerr("Invalid metadata for block %s. Offset=%s, Size=%s, FileSize=%s" % [i, block_offset, block_size, area_data.size()])
			blocks.append(PackedByteArray())
	return blocks

func _read_area_settings(area_chunk: PackedByteArray) -> void:
	a_events = Packer.decode_u32_be(area_chunk, 0)
	b_events = Packer.decode_u32_be(area_chunk, 4)
	can_wrap = bool(area_chunk[9])
	is_credits = bool(area_chunk[13])
	start_entrance = area_chunk[16]
	is_ambush = area_chunk[17]
	
	var timer_chunk: PackedByteArray = area_chunk.slice(10, 12)
	var timer: int = 200 - (256 - timer_chunk[1])
	if timer_chunk[0] != 255:
		timer = (256 * timer_chunk[0]) + timer_chunk[1] + 200
	
	time_limit = timer
