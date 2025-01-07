extends Node
class_name NSMBWLevel

var _sp: Subprocess = Subprocess.new()

var _dump_success: bool = false

var level: Dictionary = {}

#region Level extracting
# haha funny word dump
func dump_level(level_name: String, from: String, to: String) -> void:
	_sp.run_threaded("dependencies/neb-utils", ["nsmbw", "--dump", (from + level_name), to])
	_sp.bind_filter(_dump_check)
	_sp.binded_success.connect(_on_dump_success)
	_sp.start()
	
func _dump_check(line: String) -> Variant:
	if line.to_lower().contains("success"):
		_dump_success = true
		return true
	return false
	
func _on_dump_success() -> void:
	call_deferred("_post_dump")

func _post_dump() -> void:
	print(_dump_success)
#endregion

#region Level reading
func read_level(dump_path: String) -> void:
	var course: Dictionary = {}
	var course_area: String = "course1"
	
	# List of layers
	var layers: Array = ["bgdatL0", "bgdatL1", "bgdatL2"]

	var course_data: Array[PackedByteArray] = _read_level_blocks("test/course/%s.bin" % course_area)
	
	level["metadata"] = _read_metadata(course_data[0])
	level["options"] = _read_options(course_data[1])
	level["entrances"] = _read_entrances(course_data[6])
	level["sprites"] = _read_sprites(course_data[7])
	level["zones"] = _read_zones(course_data[9], course_data[2], course_data[4], course_data[5])
	
	# Iterate through the layers
	for layer: String in layers:
		var layer_path: String = "%s%s_%s.bin" % [dump_path, course_area, layer]
		var layer_data: PackedByteArray = _read_level_layerdata(layer_path)
		
		if layer_data != null:
			#print("Loaded %s (%s bytes)" % [layer, str(layer_data.size())])
			course[layer] = layer_data
		else:
			#print("%s not found." % layer)
			course[layer] = null # Assign null for missing layers

	#print("Layer data preparation completed.")

func _read_level_blocks(file_path: String) -> Array[PackedByteArray]:
	# Read the file data as a PackedByteArray
	var file_data: PackedByteArray = FileAccess.get_file_as_bytes(file_path)
	
	# Array to store blocks
	var blocks: Array[PackedByteArray] = []
	
	# Total number of blocks
	const BLOCKS: int = 14
	
	# Ensure there are enough bytes for metadata (14 blocks × 8 bytes each = 112 bytes)
	if file_data.size() < BLOCKS * 8:
		push_error("File too small to contain metadata for all blocks!")
		return blocks
	
	# Loop through the metadata section (112 bytes: 14 blocks × 8 bytes each)
	for i: int in range(BLOCKS):
		# Calculate the offset of the current block's metadata in file_data
		var meta_offset: int = i * 8
		
		# Extract the block offset (4 bytes) and size (4 bytes)
		var block_offset: int = (
			file_data[meta_offset] << 24 |
			file_data[meta_offset + 1] << 16 |
			file_data[meta_offset + 2] << 8 |
			file_data[meta_offset + 3]
		)
		var block_size: int = (
			file_data[meta_offset + 4] << 24 |
			file_data[meta_offset + 5] << 16 |
			file_data[meta_offset + 6] << 8 |
			file_data[meta_offset + 7]
		)
		
		# Debug: Print metadata
		#print("Block %d: Offset=%d, Size=%d" % [i, block_offset, block_size])
		
		# Validate the block bounds
		if block_size == 0:
			# If size is 0, append an empty block
			blocks.append(PackedByteArray())
		elif block_offset + block_size <= file_data.size():
			# Otherwise, extract the block data if within bounds
			var block_data: PackedByteArray = file_data.slice(block_offset, block_offset + block_size)
			#print("Block Data: \n%s\n" % block_data)
			
			blocks.append(block_data)
		else:
			# Invalid block metadata, log an error
			push_error("Invalid block metadata for block %d: Offset=%d, Size=%d" % [i, block_offset, block_size])
			blocks.append(PackedByteArray())  # Append an empty block as a fallback
	
	return blocks

func _read_metadata(block: PackedByteArray) -> Dictionary:
	# 32s32s32s32s
	# Ensure the block is large enough (4 strings × 32 bytes each = 128 bytes)
	if block.size() < 128:
		push_error("Block is too small to contain tileset names!")
		return {}

	# Initialize a dictionary to store the tileset names
	var tilesets: Dictionary = {}
	
	# Extract 4 strings (32 bytes each)
	for i: int in range(4):
		var start: int = i * 32
		var raw_data: PackedByteArray = block.slice(start, start + 32)
		
		# Convert the bytes to a string manually, stripping null bytes
		var tileset_name: String = ""
		for byte: Variant in raw_data:
			if byte != 0:  # Ignore null bytes
				tileset_name += char(byte)
		
		tilesets["tileset%d" % i] = tileset_name
	
	return tilesets

func _read_options(block: PackedByteArray) -> Dictionary:
	# IIHhLBBBx
	if block.size() < 20:
		push_error("Block is too small to contain options data!")
		return {}

	var options: Dictionary = {}
	
	var defEventsA: int = _int_from_bytes(block.slice(0, 4))
	var defEventsB: int = _int_from_bytes(block.slice(4, 8))
	options["wrapFlag"] = _int_from_bytes(block.slice(8, 10)) as int
	
	options["timeLimit"] = _parse_timer(block.slice(10, 12))
	
	options["unk1"] = _int_from_bytes(block.slice(12, 14)) as int
	options["startEntrance"] = (block.slice(14, 18))
	options["unk2"] = block[18]
	options["unk3"] = block[19]

	options["defEvents"] = defEventsA | (defEventsB << 32)
	
	return options

func _read_entrances(block: PackedByteArray) -> Array[Dictionary]:
	# HHxxxxBBBBxBBBHBB
	const OFFSET: int = 20
	var entrances: Array[Dictionary] = []
	var i: int = 0
	while i < len(block):
		if block:
			var chunk: PackedByteArray = block.slice(i, i + OFFSET + 1)
			var entrance: Dictionary = {}
			
			# CONSTANT VALUES
			entrance.x = (chunk[0] << 8) | chunk[1]
			entrance.y = (chunk[2] << 8) | chunk[3]
			entrance.id = chunk[8]
			entrance.destination_area = chunk[9]
			entrance.destination_entrance = chunk[10]
			entrance.type = chunk[11]
			entrance.zone = chunk[13]
			entrance.layer = chunk[14]
			entrance.path = chunk[15]
			entrance.exit_to_map = chunk[18]
			entrance.connected_pipe_direction = chunk[19]
			
			# OTHER SETTINGS
			var settings: Array[bool] = _get_bits(chunk.slice(16,18))
			if settings[8] == false:
				entrance.enterable = true
			entrances.append(entrance)
		i += OFFSET
	return entrances

func _read_sprites(block: PackedByteArray) -> Array[Dictionary]:
	# HHH8sxx
	const OFFSET: int = 16
	var sprites: Array[Dictionary]
	var i: int = 0
	while (i < len(block)):
		var chunk: PackedByteArray = block.slice(i, i + OFFSET + 1)
		var sprite: Dictionary = {}
		
		sprite.type = (chunk[0] << 8) | chunk[1]
		sprite.x = (chunk[2] << 8) | chunk[3]
		sprite.y = (chunk[4] << 8) | chunk[5]
		sprite.data = chunk.slice(6,14)
		
		if chunk[16] == 255:
			break
		
		i += OFFSET
	
	return sprites

func _read_zones(zone_config_block: PackedByteArray, zone_bounds_block: PackedByteArray, zone_bg_front: PackedByteArray, zone_bg_back: PackedByteArray) -> Array[Dictionary]:
	
	const OFFSET: int = 24
	var zones: Array[Dictionary]
	
	var i: int = 0
	
	# LOOP THROUGH ZONE CONFIGS
	while (i < len(zone_config_block)):
		var zone_config: PackedByteArray = zone_config_block.slice(i, i + OFFSET + 1)
		var zone_bounds: PackedByteArray = zone_bounds_block.slice(i, i + OFFSET + 1)
		var zone_bgf: PackedByteArray = zone_bg_front.slice(i, i + OFFSET + 1)
		var zone_bgb: PackedByteArray = zone_bg_back.slice(i, i + OFFSET + 1)
		
		zones.append(_parse_zone(zone_config, zone_bounds, zone_bgf, zone_bgb))
		i += OFFSET

	return zones

func _read_level_layerdata(file_path: String) -> PackedByteArray:
	# Check if the file exists
	if FileAccess.file_exists(file_path):
		var data: PackedByteArray = FileAccess.get_file_as_bytes(file_path)
		return data
	else:
		return []
#endregion

#region Level reading helpers
func _parse_timer(bytes: PackedByteArray) -> int:
	if bytes[0] < 255:
		return (256 * bytes[0]) + bytes[1] + 200
	if bytes[0] == 255:
		return 200 - (256 - bytes[1])
	return 0

func _parse_zone(zone_config_block: PackedByteArray, zone_bounds_block: PackedByteArray, zone_bg_front: PackedByteArray, zone_bg_back: PackedByteArray) -> Dictionary:
	
	var zone: Dictionary = {}
	zone.id = zone_config_block[13]
	zone.pos_x = (zone_config_block[0] << 8) | zone_config_block[1]
	zone.pos_y = (zone_config_block[2] << 8) | zone_config_block[3]
	zone.size_x = (zone_config_block[4] << 8) | zone_config_block[5]
	zone.size_y = (zone_config_block[6] << 8) | zone_config_block[7]
	zone.theme = zone_config_block[9]
	zone.lighting = zone_config_block[11]
	zone.music = zone_config_block[22]
	zone.echo = zone_config_block[23] / 16
	
	if bool(zone_config_block[23] % 16):
		zone.boss_room = true

	# Handle spotlight/darkness
	var spotlight_setting: int = zone_config_block[17]
	
	if spotlight_setting >= 32:
		zone.is_dark = true
		spotlight_setting -= 32

	if spotlight_setting >= 16:
		zone.fg_spotlight = true
		spotlight_setting -= 16
	
	zone.spotlight_setting = spotlight_setting
	
	return zone

func _int_from_bytes(data: PackedByteArray, reverse_endian: bool = false) -> int:
	var result: int = 0
	for i: int in range(data.size()):
		if reverse_endian:
			result |= data[i] << (8 * i)  # Reverse byte order (little-endian)
		else:
			result |= data[i] << (8 * (data.size() - i - 1))  # Big-endian byte order
	return result

func _get_bits(byte_array: PackedByteArray) -> Array[bool]:
	var bits: Array[bool] = []
	for byte: int in byte_array:
		for i: int in range(8):
			var bit: bool = ((byte >> (7 - i)) & 1) == 1
			bits.append(bit)
	return bits
#endregion
