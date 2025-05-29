extends Node

const UNKOWN_TILE: NSMBWTile = preload("uid://b4cludkl2agxj")

@export var canvas: GraphEdit
@export var layers: Array[TileMapLayer]
var level: NSMBWLevel
var tileset: TileSet = TileSet.new()
var object_patterns: Dictionary[int, Array] = {}


func _ready() -> void:
	await owner.ready
	level = owner.level


func load_tiles(area_num: int) -> void:
	# Map tilesets to sources in a Godot TileSet
	_load_tilesets(area_num)
	
	# Assign the TileSet to each layer
	for i: int in range(3):
		layers[i].tile_set = tileset
	
	_load_tiles(area_num)


func _load_tilesets(area_num: int) -> void:
	tileset.tile_size = Vector2i(24, 24)
	var current_tileset: int = 0
	for nsmbw_tileset: NSMBWTileset in level.areas[area_num].tilesets:
		var patterns: Array[TileMapPattern] = []
		var tileset_atlas: TileSetAtlasSource = TileSetAtlasSource.new()
		
		patterns.resize(nsmbw_tileset.objects.size())
		
		tileset_atlas.texture = nsmbw_tileset.texture
		tileset_atlas.texture_region_size = Vector2i(24, 24)
		
		# Create tiles in the atlas source first
		if nsmbw_tileset.texture:
			var texture_size: Vector2i = nsmbw_tileset.texture.get_size()
			var tiles_x: int = texture_size.x / 32
			var tiles_y: int = texture_size.y / 32
			
			for y: int in range(tiles_y):
				for x: int in range(tiles_x):
					var atlas_coords: Vector2i = Vector2i(x, y)
					tileset_atlas.create_tile(atlas_coords)
			
			# Set the margins and separation if needed
			tileset_atlas.margins = Vector2i(4, 4)
			tileset_atlas.separation = Vector2i(8, 8)
		
		# Parse objects as TileMapPatterns
		for object_index: int in range(nsmbw_tileset.objects.size()):
			var object: Array = nsmbw_tileset.objects[object_index]
			
			if object.size() == 0:
				patterns[object_index] = null
				continue
			
			var pattern: TileMapPattern = TileMapPattern.new()
			
			pattern.set_size(Vector2i(object[0].size(), object.size()))
			
			for y: int in range(object.size()):
				for x: int in range(object[y].size()):
					var atlas_index: int = object[y][x].atlas_index
					var texture_size: Vector2i = nsmbw_tileset.texture.get_size()
					
					var atlas_pos: Vector2i = Vector2i(
						atlas_index % 32,
						atlas_index / 32
					)
					
					# Verify the atlas position exists
					if tileset_atlas.has_tile(atlas_pos):
						pattern.set_cell(Vector2i(x, y), current_tileset, atlas_pos)
			
			patterns[object_index] = pattern
			
		object_patterns[current_tileset] = patterns
		tileset.add_source(tileset_atlas, current_tileset)
		current_tileset += 1


func _load_tiles(area_num: int) -> void:
	var tile_layers: Array[NSMBWTileLayer] = level.areas[area_num].tile_layers
	
	for current_tile_layer: int in range(tile_layers.size()):
		var tile_layer: NSMBWTileLayer = tile_layers[current_tile_layer]
		
		for tile: NSMBWTile in tile_layer.tiles:
			
			if not object_patterns.has(tile.tileset):
				continue
			
			_place_tile(current_tile_layer, tile)
			#
			#if not object_patterns.has(tile.tileset):
				#continue
				#
			#var patterns_array: Array = object_patterns[tile.tileset]
			#
			#if tile.object_id >= patterns_array.size():
				#continue
				#
			#var tilemap_pattern: TileMapPattern = patterns_array[tile.object_id]
			#
			# Only set pattern if it's not null
			#if tilemap_pattern != null:
				
				
				# Set individual cells from the pattern
				#for pattern_y: int in range(tilemap_pattern.get_size().y):
					#for pattern_x: int in range(tilemap_pattern.get_size().x):
						#var cell_pos: Vector2i = tile.position + Vector2i(pattern_x, pattern_y)
						#var source_id: int = tilemap_pattern.get_cell_source_id(Vector2i(pattern_x, pattern_y))
						#var atlas_coords: Vector2i = tilemap_pattern.get_cell_atlas_coords(Vector2i(pattern_x, pattern_y))
						#
						#if source_id != -1:
							#layers[current_tile_layer].set_cell(cell_pos, source_id, atlas_coords)


func _place_tile(layer: int, tile: NSMBWTile) -> void:
	var pattern: TileMapPattern = TileMapPattern.new()
	
	if object_patterns.has(tile.tileset) and object_patterns.get(tile.tileset) and tile.object_id < object_patterns.get(tile.tileset).size():
		pattern = object_patterns.get(tile.tileset)[tile.object_id]
	else:
		return
	
	if !pattern:
		return
	
	var pos_offset: Vector2i = tile.position
	for cell_x: int in range(tile.size.x):
		for cell_y: int in range(tile.size.y):
			layers[layer].set_cell(pos_offset + Vector2i(cell_x, cell_y), tile.tileset, pattern.get_cell_atlas_coords(Vector2i(cell_x, cell_y) % pattern.get_size()))
