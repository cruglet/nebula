class_name CallTime
extends Node

static var call_time: float = 0

static func start() -> void:
	call_time = Time.get_unix_time_from_system()

static func end() -> void:
	print(Time.get_unix_time_from_system() - call_time)
