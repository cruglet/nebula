class_name NSMBWTileLayer extends Object

var tiles: Array[NSMBWTile]

static func from_blocks(fg_data: PackedByteArray, g_data: PackedByteArray, bg_data: PackedByteArray) -> Array[NSMBWTileLayer]:
	var tile_layer: Array[NSMBWTileLayer]
	tile_layer.append(process_layer(fg_data))
	tile_layer.append(process_layer(g_data))
	tile_layer.append(process_layer(bg_data))
	
	return tile_layer

static func process_layer(data: PackedByteArray) -> NSMBWTileLayer:
	const OFFSET: int = 10
	var tile_layer: NSMBWTileLayer = NSMBWTileLayer.new()
	
	var tile_idx: int = 0
	while tile_idx + OFFSET <= data.size():
		var chunk: PackedByteArray = data.slice(tile_idx, tile_idx + OFFSET)
		var tile: NSMBWTile = NSMBWTile.new()
		
		var tileset: int = int(chunk[0]) / 16
		tile.tileset = tileset
		tile.object_id = chunk[1]
		tile.position.x = Packer.decode_u16_be(chunk, 2)
		tile.position.y = Packer.decode_u16_be(chunk, 4)
		tile.size.x = Packer.decode_u16_be(chunk, 6)
		tile.size.y = Packer.decode_u16_be(chunk, 8)
		
		tile_layer.tiles.append(tile)
		tile_idx += OFFSET
	
	return tile_layer
