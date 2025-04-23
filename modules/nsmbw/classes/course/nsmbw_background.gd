class_name NSMBWBackground
extends Object

var scroll_rate: Vector2i
var position: Vector2i
var instance: int
var zoom: int

const OFFSET: int = 24

static func from_blocks(bg_data: PackedByteArray) -> NSMBWBackground:
	var bg: PackedByteArray = bg_data.slice(0, OFFSET)
	
	var background: NSMBWBackground = NSMBWBackground.new()
	
	background.scroll_rate.x = bg[3]
	background.scroll_rate.y = bg[5]
	background.position.x = Packer.decode_s16_be(bg_data, 8)
	background.position.y = Packer.decode_s16_be(bg_data, 6)
	background.instance = bg[10]
	background.zoom = bg[19]

	return background
