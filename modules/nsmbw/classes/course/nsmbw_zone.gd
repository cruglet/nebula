class_name NSMBWZone extends Object

var pos_x: int
var pos_y: int
var size_x: int
var size_y: int
var theme: int
var lighting: int
var id: int
var music: int

var upper_bound: int
var lower_bound: int
var lakitu_upper_bound: int
var lakitu_lower_bound: int
var multiplayer_fly_screen_adjust: int
var multiplayer_upper_bound: int
var multiplayer_lower_bound: int

var echo: int
var boss_room: bool
var is_dark: bool
var fg_spotlight: bool
var spotlight_config: int

var bg_front: NSMBWBackground
var bg_back: NSMBWBackground

const OFFSET: int = 24

static func from_blocks(zone_config_data: PackedByteArray, zone_bounds_data: PackedByteArray, bg_front_data: PackedByteArray, bg_back_data: PackedByteArray) -> Array[NSMBWZone]:
	var zones: Array[NSMBWZone]
	var count: int = min(zone_config_data.size(), zone_bounds_data.size()) / OFFSET
	
	for i: int in range(count):
		var pos: int = i * OFFSET
		var zc: PackedByteArray = zone_config_data.slice(pos, pos + OFFSET)
		var zb: PackedByteArray = zone_bounds_data.slice(pos, pos + OFFSET)
		
		var zone: NSMBWZone = NSMBWZone.new()
		
		# Config
		zone.pos_x = Packer.decode_u16_be(zc, 0)
		zone.pos_y = Packer.decode_u16_be(zc, 2)
		zone.size_x = Packer.decode_u16_be(zc, 4)
		zone.size_y = Packer.decode_u16_be(zc, 6)
		zone.theme = int(zc[8])
		zone.lighting = int(zc[9])
		zone.id = int(zc[11])
		zone.music = int(zc[18])
		
		var spotlight_val: int = zc[17]

		zone.is_dark = spotlight_val >= 32
		if zone.is_dark:
			spotlight_val -= 32
		zone.fg_spotlight = spotlight_val >= 16
		if zone.fg_spotlight:
			spotlight_val -= 16
		zone.spotlight_config = spotlight_val
		
		var echo_boss_byte: int = int(zc[23])
		zone.echo = echo_boss_byte / 16
		zone.boss_room = (echo_boss_byte % 16) != 0
		
		# Bounds
		zone.upper_bound = Packer.decode_u32_be(zb, 0)
		zone.lower_bound = Packer.decode_u32_be(zb, 4)
		zone.lakitu_upper_bound = Packer.decode_u32_be(zb, 8)
		zone.lakitu_lower_bound = Packer.decode_u32_be(zb, 12)
		zone.multiplayer_fly_screen_adjust = Packer.decode_u16_be(zb, 16)
		zone.multiplayer_upper_bound = Packer.decode_u16_be(zb, 18)
		zone.multiplayer_lower_bound = Packer.decode_u16_be(zb, 20)
		
		zone.bg_front = NSMBWBackground.from_blocks(bg_front_data.slice(pos, pos + OFFSET))
		zone.bg_back = NSMBWBackground.from_blocks(bg_back_data.slice(pos, pos + OFFSET))
		
		zones.append(zone)
	
	return zones
