extends Panel

const FETCH_TEXT_SPEED: float = 0.5

@export var fetching_label: Label

var loading_text_anim_timer: Timer
var _fetch_text_iter: int = 0


func _ready() -> void:
	loading_text_anim_timer = Timer.new()
	add_child(loading_text_anim_timer)
	
	loading_text_anim_timer.timeout.connect(_on_fetching_text_timeout)
	loading_text_anim_timer.start(FETCH_TEXT_SPEED)


func _on_fetching_text_timeout() -> void:
	_fetch_text_iter = wrapi(_fetch_text_iter + 1, 0, 4)
	fetching_label.text = "Fetching available modules online" + ".".repeat(_fetch_text_iter)
