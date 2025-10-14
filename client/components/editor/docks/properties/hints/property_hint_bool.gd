class_name NebulaPropertyHintBool
extends NebulaPropertyHint

@export var property_value: bool

@export_group("Internal")
@export var property_checkbox: CheckBox
@export var property_label: Label


func _ready() -> void:
	property_label.text = property_name
	property_checkbox.set_pressed_no_signal(property_value)


func _on_check_box_toggled(toggled_on: bool) -> void:
	property_value = toggled_on
