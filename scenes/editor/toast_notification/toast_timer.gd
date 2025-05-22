extends Node

static var current_toast: ToastNotification

@export var toast_progress: ProgressBar
@export var toast_header_label: Label
@export var toast_description_label: Label
@export var animation_player: AnimationPlayer

@onready var timer: Timer


func _ready() -> void:
	await owner.ready
	toast_header_label.text = owner.toast_header
	toast_description_label.text = owner.toast_description

	current_toast = owner
	animation_player.play(&"enter")


func _on_begin() -> void:
	timer = Timer.new()
	timer.timeout.connect(_on_finish, CONNECT_ONE_SHOT)
	toast_progress.max_value = owner.time
	
	add_child(timer)
	
	timer.start(owner.time)


func _process(delta: float) -> void:
	if current_toast and current_toast != owner:
		if timer:
			timer.paused = true
		_on_finish()
		return
	if timer:
		toast_progress.value = owner.time - timer.time_left


func _on_finish() -> void:
	toast_progress.hide()
	animation_player.play(&"exit")
	animation_player.animation_finished.connect(func(_a: Variant) -> void:
		owner.queue_free()
	)
