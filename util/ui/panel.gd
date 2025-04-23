@tool
class_name nPanel extends Panel

enum TYPE {
	PRIMARY,
	SECONDARY
}

@export var _name: String:
	set(n):
		name = n
		_name = n

@export var type: TYPE:
	set(t):
		match t:
			TYPE.PRIMARY:
				theme_type_variation = &"nPanelPrimary"
			TYPE.SECONDARY:
				theme_type_variation = &"nPanelSecondary"
		type = t
		
@export_tool_button("Finalize", "ImportCheck") var f: Callable:
	get():
		return finalize

func _ready() -> void:
	theme = Singleton.theme
	_name = name

func finalize() -> void:
	set_script(null)
