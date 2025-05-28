extends Node

const UNKNOWN_SPRITE: NSMBWSprite = preload("uid://dng7q0mv0wshv")

@export var canvas: GraphEdit
@export var tile_loader: Node

var level: NSMBWLevel


func _ready() -> void:
	await owner.ready
	level = owner.level
	load_area(0)


func load_area(area_num: int) -> void:
	# Clean all level nodes
	for child: Node in canvas.get_children():
		if not child.is_in_group(&"editor_immune") and child is GraphElement:
			child.queue_free()
	
	tile_loader.load_tiles(area_num)
	#load_sprites(area_num)

#func load_metadata(area_num: int) -> void:
	#pass


#func load_sprites(area_num: int) -> void:
	#var sprites: Array[NSMBWSprite] = level.areas[area_num].sprites
	#
	#for sprite: NSMBWSprite in sprites:
		#var editor_object: GraphElement = GraphElement.new()
		#var editor_object_texture: TextureRect = TextureRect.new()
		#editor_object.hide()
		#
		#editor_object.position_offset = sprite.position
		#
		#if sprite.type not in NSMBWSprite.sprite_list:
			#editor_object_texture.texture = UNKNOWN_SPRITE.texture
			#editor_object.size = UNKNOWN_SPRITE.size
		#else:
			#print("this isnt supposed to print")
			#
		#editor_object_texture.expand_mode = TextureRect.EXPAND_IGNORE_SIZE
		#editor_object_texture.set_anchors_preset(Control.PRESET_FULL_RECT)
		#
		#canvas.clear_connections()
		#canvas.add_child(editor_object)
		#editor_object.add_child(editor_object_texture)
		#editor_object.show()
