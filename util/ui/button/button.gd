@tool
class_name nButton extends Button

enum TYPE {
	PRIMARY,
	SECONDARY,
	FLAT_ICON,
	FLAT_ICON_TAB,
}

const N_BUTTON_HOVER_BG: StyleBoxFlat = preload("uid://cgcc2tbcq15hl")

@export var type: TYPE = TYPE.PRIMARY:
	set(t):
		toggle_mode = false
		if button_down.is_connected(flat_icon_button_down):
			button_down.disconnect(flat_icon_button_down)
		if button_up.is_connected(flat_icon_button_up):
			button_up.disconnect(flat_icon_button_up)

		match t:
			TYPE.PRIMARY:
				theme_type_variation = &"nButtonPrimary" 
				mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
			TYPE.SECONDARY:
				theme_type_variation = &"nButtonSecondary" 
				mouse_default_cursor_shape = Control.CURSOR_POINTING_HAND
			TYPE.FLAT_ICON: 
				theme_type_variation = &"nButtonFlatIcon"
				button_up.connect(flat_icon_button_up)
				button_down.connect(flat_icon_button_down)
				mouse_default_cursor_shape = Control.CURSOR_ARROW
			TYPE.FLAT_ICON_TAB:
				theme_type_variation = &"nButtonFlatIconTab"
				button_down.connect(flat_icon_button_down)
				button_up.connect(flat_icon_button_up)
				mouse_default_cursor_shape = Control.CURSOR_ARROW
		type = t

@export var mod_color: Color = Color("#0058AB"):
	set(c):
		mod_color = c

@export var focus_width: int = 0:
	set(fw):
		var focus_stylebox: StyleBoxFlat = StyleBoxFlat.new()
		focus_stylebox.draw_center = false
		focus_stylebox.border_color = mod_color
		focus_stylebox.set_corner_radius_all(4)
		if fw > 0:
			focus_stylebox.set_border_width_all(fw)
		else:
			focus_stylebox.set_border_width_all(0)
		add_theme_stylebox_override("focus", focus_stylebox)
		focus_width = fw

@export var focus_margin: Vector2 = Vector2(4, 2):
	set(fm):
		var focus_stylebox: StyleBoxFlat = get_theme_stylebox(&"focus", theme_type_variation)
		focus_stylebox.expand_margin_left = fm.x
		focus_stylebox.expand_margin_right = fm.x
		focus_stylebox.expand_margin_bottom = fm.y
		focus_stylebox.expand_margin_top = fm.y
		focus_margin = fm

@export var hover_background: bool = true:
	set(hbg):
		if hbg:
			add_theme_stylebox_override(&"hover", N_BUTTON_HOVER_BG)
		else:
			remove_theme_stylebox_override(&"hover")
		hover_background = hbg

@onready var active: bool = false:
	set(a):
		if a or force_active:
			self_modulate = mod_color
		else:
			self_modulate = Color.WHITE
		active = a

@export var force_active: bool = false:
	set(fa):
		active = fa
		force_active = fa

@export var font_size: int = 16:
	set(fs):
		add_theme_font_size_override(&"font_size", fs)
		font_size = fs

func _ready() -> void:
	theme = Singleton.theme
	type = type

func flat_icon_button_down() -> void:
	active = true

func flat_icon_button_up() -> void:
	active = false
