extends PanelContainer

@export var logs_spin_box: SpinBox

func _ready() -> void:
	logs_spin_box.value = EngineConfig.max_logs

func _on_logs_spin_box_value_changed(value: float) -> void:
	EngineConfig.max_logs = value
