class_name Dict extends Node

static func get_total_size(data: Dictionary) -> int:
	var total: int = 0
	for value: Variant in data.values():
		if typeof(value) == TYPE_DICTIONARY:
			total += get_total_size(value)
		elif typeof(value) == TYPE_PACKED_BYTE_ARRAY:
			total += value.size()
	return total

static func insert_nested(dict: Dictionary, path: Array[String], key: String, value: Variant) -> void:
	for p: String in path:
		if !dict.has(p):
			dict[p] = {}
		dict = dict[p]
	dict[key] = value
