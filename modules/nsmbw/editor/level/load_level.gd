extends Node

const UNKNOWN_SPRITE: NSMBWSprite = preload("uid://dng7q0mv0wshv")
const UNKOWN_TILE: NSMBWTile = preload("uid://b4cludkl2agxj")

@export var canvas: GraphEdit

var level: NSMBWLevel

func _ready() -> void:
	level = owner.level
	load_area(0)


func load_area(area_num: int) -> void:
	# Clean all level nodes
	for child: Node in canvas.get_children():
		if not child.is_in_group(&"editor_immune") and child is GraphElement:
			child.queue_free()
	
	load_tiles(area_num)
	load_sprites(area_num)

func load_tiles(area_num: int) -> void:
	var tiles: Array[NSMBWTileLayer] = level.areas[area_num].tile_layers
	
	for tile_layer: NSMBWTileLayer in tiles:
		for tile: NSMBWTile in tile_layer.tiles:
			var editor_object: GraphElement = GraphElement.new()
			var editor_object_texture: TextureRect = TextureRect.new()
			editor_object.hide()
			
			editor_object.position_offset = tile.position * 16
			
			if tile.object_id not in NSMBWTile.tile_list:
				editor_object_texture.texture = UNKOWN_TILE.texture
			else:
				print("this isnt supposed to print")
				
			editor_object.size = tile.size * 16
			editor_object_texture.stretch_mode = TextureRect.STRETCH_TILE
			editor_object_texture.texture_filter = CanvasItem.TEXTURE_FILTER_LINEAR
			editor_object_texture.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
			editor_object_texture.set_anchors_preset(Control.PRESET_FULL_RECT)
			
			canvas.clear_connections()
			canvas.add_child(editor_object)
			editor_object.add_child(editor_object_texture)
			editor_object.show()


func load_sprites(area_num: int) -> void:
	var sprites: Array[NSMBWSprite] = level.areas[area_num].sprites
	
	for sprite: NSMBWSprite in sprites:
		var editor_object: GraphElement = GraphElement.new()
		var editor_object_texture: TextureRect = TextureRect.new()
		editor_object.hide()
		
		editor_object.position_offset = sprite.position
		
		if sprite.type not in NSMBWSprite.sprite_list:
			editor_object_texture.texture = UNKNOWN_SPRITE.texture
			editor_object.size = UNKNOWN_SPRITE.size
		else:
			print("this isnt supposed to print")
			
		editor_object_texture.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
		editor_object_texture.set_anchors_preset(Control.PRESET_FULL_RECT)
		
		canvas.clear_connections()
		canvas.add_child(editor_object)
		editor_object.add_child(editor_object_texture)
		editor_object.show()
