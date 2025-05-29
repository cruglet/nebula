class_name NSMBWTileset extends Resource
# Tilesets have a 32 * 8 atlas
var name: String
var texture: Texture2D
var objects: Array

static func from_blocks(block_data: PackedByteArray, level: NSMBWLevel) -> Array[NSMBWTileset]:
	var tilesets: Array[NSMBWTileset]
	
	var textures_path: String = level.archive_path.get_base_dir().path_join("Texture")
	tilesets.resize(4)
	for i: int in range(4):
		var chunk: PackedByteArray = block_data.slice(i * 32, (i + 1) * 32)
		var tileset: NSMBWTileset = NSMBWTileset.new()
		
		tileset.name = chunk.get_string_from_utf8()
		
		if FileAccess.file_exists(textures_path.path_join(tileset.name + ".arc")):
			var texture_arc: ARC = ARC.open(textures_path.path_join(tileset.name + ".arc"))
			
			if !texture_arc.filesystem.has("BG_tex"):
				Singleton.toast_notification("Error loading tileset!", "Tileset does not seem to have textures attached.")
				tilesets.set(i, NSMBWTileset.new())
				continue
			
			# Object data
			for key: String in texture_arc.filesystem.get("BG_unt").keys():
				if tileset.name + ".bin" in key:
					tileset._parse_objects(texture_arc.filesystem.get("BG_unt").get(key))
			
			# Texture data
			for key: String in texture_arc.filesystem.get("BG_tex").keys():
				if tileset.name + "_tex" in key:
					var texture_data: PackedByteArray = texture_arc.filesystem.get("BG_tex").get(key)
					
					if key.ends_with(".LZ"):
						texture_data = LZSS.decompress(texture_data)
					
					tileset.texture = Packer.rgb4a3_decode(texture_data, true)
		
		tilesets.set(i, tileset)
	return tilesets


func get_tile(object_id: int) -> Texture2D:
	var atlas_texture: AtlasTexture = AtlasTexture.new()
	atlas_texture.atlas = texture
	
	var arr: Variant = objects[object_id - 1].get(0)
	var index: int = 0
	
	if arr:
		if arr.size():
			index = arr[0].get_or_add("atlas_texture", 0)
	
	if object_id > objects.size():
		var placeholder: PlaceholderTexture2D = PlaceholderTexture2D.new()
		placeholder.size = Vector2i(32, 32)
		return placeholder
	
	var position: Vector2i = Vector2i(
		32 * (index % 32),
		8 * (index / 32)
	)
	var size: Vector2i = Vector2i(32, 32)
	
	atlas_texture.region = Rect2(position, size * 16)
	
	return ImageTexture.create_from_image(atlas_texture.get_image())


func _parse_objects(data: PackedByteArray) -> Array:
	var col: Array[Array] = []
	var current_row: Array[Dictionary] = []
	var offset: int = 0
	
	var is_slope: bool = false
	
	while offset < data.size():
		var object_data: Dictionary = {}
		
		object_data.set("scale_behavior", data.get(offset))
		offset += 1
		
		if object_data.get("scale_behavior") >= 144:
			is_slope = true
		
		if is_slope:
			offset += 1
		
		object_data.set("atlas_index", data.get(offset))
		offset += 1
		
		object_data.set("object_type", data.get(offset))
		offset += 1
		
		current_row.append(object_data)
		
		# End of object row
		if data.get(offset) == 254:
			col.append(current_row)
			current_row = []
			offset += 1
		
		# End of object data
		if data.get(offset) == 255:
			objects.append(col)
			current_row = []
			col = []
			offset += 1
			is_slope = false
	
	return objects


func _to_string() -> String:
	return name
