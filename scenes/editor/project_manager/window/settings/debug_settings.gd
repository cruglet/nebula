extends PanelContainer

@export var logs_spin_box: SpinBox

func _ready() -> void:
	logs_spin_box.value = NebulaConfig.max_logs

func _on_logs_spin_box_value_changed(value: float) -> void:
	NebulaConfig.max_logs = value
