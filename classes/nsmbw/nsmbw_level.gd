extends Node
class_name NSMBWLevel

var _sp: Subprocess = Subprocess.new()

var _dump_success: bool = false

var level: Dictionary = {}

signal dump_finished

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
		dump_finished.emit()
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
			course[layer] = null

func _read_level_blocks(file_path: String) -> Array[PackedByteArray]:
	var file_data: PackedByteArray = FileAccess.get_file_as_bytes(file_path)
	var blocks: Array[PackedByteArray] = []
	const BLOCKS: int = 14
	
	if file_data.size() < BLOCKS * 8:
		push_error("File too small to contain metadata for all blocks!")
		return blocks
	
	for i: int in range(BLOCKS):
		var meta_offset: int = i * 8
		
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
		
		#print("Block %d: Offset=%d, Size=%d" % [i, block_offset, block_size])
		
		if block_size == 0:
			blocks.append(PackedByteArray())
		elif block_offset + block_size <= file_data.size():
			var block_data: PackedByteArray = file_data.slice(block_offset, block_offset + block_size)
			
			blocks.append(block_data)
		else:
			push_error("Invalid block metadata for block %d: Offset=%d, Size=%d" % [i, block_offset, block_size])
			blocks.append(PackedByteArray()) 
	
	return blocks

func _read_metadata(block: PackedByteArray) -> Dictionary:
	if block.size() < 128:
		push_error("Block is too small to contain tileset names...")
		return {}
	
	var loaded_tilesets: Array = ByteParser.translate("32s32s32s32s", block)
	var tilesets: Dictionary = {}
	
	for i: int in range(4):
		tilesets["tileset%d" % i] = loaded_tilesets[i]
		
	return tilesets

func _read_options(block: PackedByteArray) -> Dictionary:
	if block.size() < 20:
		push_error("Block is too small to contain options data...")
		return {}

	var loaded_options: Array = ByteParser.translate("2L:H:h:6B", block)
	var options: Dictionary = {}
	var events_a: int = loaded_options[0]
	var events_b: int = loaded_options[1]
	
	options.can_wrap = bool(loaded_options[2])
	options.is_credits = bool(loaded_options[4])
	options.time_limit = _parse_timer(block.slice(10, 12))
	options.start_entrance = loaded_options[7]
	options.is_ambush = bool(loaded_options[8])
	options.startup_event_states = events_a | (events_b << 32)
	
	return options

func _read_entrances(block: PackedByteArray) -> Array[Dictionary]:
	const OFFSET: int = 20
	var entrances: Array[Dictionary] = []
	var i: int = 0
	while i < len(block):
		if block:
			var chunk: Array = ByteParser.translate("2H:4x:4B:x:3B:H:B:B", block.slice(i, i + OFFSET + 1))
			var chunk_2: PackedByteArray = block.slice(i, i + OFFSET + 1)
			var entrance: Dictionary = {}
			
			# Primary settings
			entrance.x = chunk[0]
			entrance.y = chunk[1]
			entrance.id = chunk[2]
			entrance.destination_area = chunk[3]
			entrance.destination_entrance = chunk[4]
			entrance.type = chunk[5]
			entrance.zone = chunk[6]
			entrance.layer = chunk[7]
			entrance.path = chunk[8]
			entrance.exit_to_map = bool(chunk[10])
			entrance.connected_pipe_direction = chunk[11]
			
			# Data settings
			var settings: Array[bool] = _get_bits(block.slice(i + 16, i + OFFSET + 19))
			entrance.enterable = settings[8]
			entrances.append(entrance)
		i += OFFSET
	return entrances

func _read_sprites(block: PackedByteArray) -> Array[Dictionary]:
	const OFFSET: int = 16
	var sprites: Array[Dictionary]
	var i: int = 0
	while (i < len(block)):
		var chunk: Array = ByteParser.translate("3H8Bxx", block.slice(i, i + OFFSET + 1))
		var sprite: Dictionary = {}
		
		# This indicates that we reached the end of the block
		if chunk[0] == 65535:
			break
		
		sprite.type = chunk[0]
		sprite.x = chunk[1]
		sprite.y = chunk[2]
		sprite.data = chunk.slice(3,12)
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
	var zone_config: Array = ByteParser.translate("6H:4B:x:4B:x:2B", zone_config_block)
	
	#region Zone Config
	zone.id = zone_config[7]
	zone.pos_x = zone_config[0]
	zone.pos_y = zone_config[1]
	zone.size_x = zone_config[2]
	zone.size_y = zone_config[3]
	zone.theme = zone_config[4]
	zone.lighting = zone_config[5]
	zone.music = zone_config[14]
	zone.echo = zone_config[15] / 16
	zone.boss_room = bool(zone_config[15] % 16)
		
	# Handle spotlight/darkness
	zone.is_dark = false
	zone.fg_spotlight = false
	
	var spotlight_setting: int = zone_config[10]
	
	if spotlight_setting >= 32:
		zone.is_dark = true
		spotlight_setting -= 32
		
	if spotlight_setting >= 16:
		zone.fg_spotlight = true
		spotlight_setting -= 16
		
	zone.spotlight_setting = spotlight_setting
	#endregion
	#region Zone Bounds
	var zone_bounds: Array = ByteParser.translate("4L:xx:3H:x", zone_bounds_block)
	zone.upper_bound = zone_bounds[0]
	zone.lower_bound = zone_bounds[1]
	zone.lakitu_upper_bound = zone_bounds[2]
	zone.lakitu_lower_bound = zone_bounds[3]
	zone.multiplayer_upper_bound = zone_bounds[5]
	zone.multiplayer_lower_bound = zone_bounds[6]
	zone.multiplayer_screen_adjust = -1
	zone.only_fly_scrolling = false
	
	if zone_bounds[4] < 15:
		zone.multiplayer_fly_screen_adjust = zone_bounds[4]
	#endregion
	
	
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
