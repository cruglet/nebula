class_name PropertyCategory
extends Control


@export var category_name: String
@export var icon: Texture2D

@export_group("Internal")
@export var category_name_label: Label
@export var category_icon_rect: TextureRect


func _ready() -> void:
	category_name_label.text = category_name
	category_icon_rect.texture = icon
