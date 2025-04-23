@tool
class_name nLabel extends Label

enum TYPE {
	PRIMARY,
	SECONDARY,
}

@export var type: TYPE = TYPE.PRIMARY:
	set(t):
		match t:
			TYPE.PRIMARY: theme_type_variation = &"nLabelPrimary"
			TYPE.SECONDARY: theme_type_variation = &"nLabelSecondary"
		type = t

@export var color_override: Color:
	set(c):
		add_theme_color_override("font_color", c)
		if c == Color.BLACK:
			remove_theme_color_override("font_color")
		color_override = c

func _ready() -> void:
	theme = Singleton.theme
	type = type
