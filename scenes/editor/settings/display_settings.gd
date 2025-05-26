extends Node

@export var scale_slider: HSlider
@export var scale_display_label: Label

var scale_dragging: bool


func _ready() -> void:
	scale_slider.value = Nebula.Config.Editor.scale


func _on_scale_slider_value_changed(value: float) -> void:
	scale_display_label.text = "%sx" % value
	
	if !scale_dragging:
		Nebula.Config.Editor.scale = scale_slider.value
		Singleton.scale_changed.emit()


func _on_scale_slider_drag_ended(_value_changed: bool) -> void:
	scale_dragging = false
	Nebula.Config.Editor.scale = scale_slider.value
	Singleton.scale_changed.emit()


func _on_scale_slider_drag_started() -> void:
	scale_dragging = true
