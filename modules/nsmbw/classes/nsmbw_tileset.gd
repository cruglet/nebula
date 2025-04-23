class_name NSMBWTileset extends Object

var name: String

static func from_blocks(block_data: PackedByteArray) -> Array[NSMBWTileset]:
	var tilesets: Array[NSMBWTileset]
	tilesets.resize(4)
	for i: int in range(4):
		var chunk: PackedByteArray = block_data.slice(i * 32, (i + 1) * 32)
		var tileset: NSMBWTileset = NSMBWTileset.new()
		
		tileset.name = chunk.get_string_from_utf8()
		
		tilesets.set(i, tileset)
	return tilesets

func _to_string() -> String:
	return name
