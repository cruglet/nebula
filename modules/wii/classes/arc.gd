class_name ARC extends Node

enum ARC_TYPE {
	NIL,
	U8,
}

const U8_HEADER: PackedByteArray = [85, 170, 56, 45] # "U\xAA8-"

var filesystem: Dictionary = {}
var type: ARC_TYPE
var path: String


static func open(file_path: String) -> ARC:
	var data: Dictionary = {}
	var raw_data: PackedByteArray = FileAccess.get_file_as_bytes(file_path)
	if raw_data == PackedByteArray() or raw_data.size() < 0x20:
		push_error("Invalid or empty ARC file.")
		return

	var offset: int = 0
	while offset <= raw_data.size() - 4:
		var magic: PackedByteArray = raw_data.slice(offset, offset + 4)
		if magic == U8_HEADER:
			break
		offset += 1

	if offset > raw_data.size() - 4 or offset + 16 > raw_data.size():
		push_error("U8 header incomplete or corrupted.")
		return

	var rootnode_offset: int = Packer.decode_u32_be(raw_data, offset + 4)
	var data_offset: int = Packer.decode_u32_be(raw_data, offset + 12)

	var node_base: int = offset + rootnode_offset
	var root_node_size: int = Packer.decode_u32_be(raw_data, node_base + 8)

	var nodes: Array[Dictionary] = []
	var node_offset: int = node_base + 12

	for i: int in range(root_node_size - 1):
		var node: Dictionary = {
			"type": Packer.decode_u16_be(raw_data, node_offset),
			"name_offset": Packer.decode_u16_be(raw_data, node_offset + 2),
			"data_offset": Packer.decode_u32_be(raw_data, node_offset + 4),
			"size": Packer.decode_u32_be(raw_data, node_offset + 8)
		}
		nodes.append(node)
		node_offset += 12

	var string_table_offset: int = node_offset
	var string_table_size: int = data_offset - (string_table_offset - offset)
	var string_table: PackedByteArray = raw_data.slice(string_table_offset, string_table_offset + string_table_size)

	var path_stack: Array[String] = []
	var count_stack: Array[int] = [root_node_size]
	var current_index: int = 0

	for node: Dictionary in nodes:
		current_index += 1

		var name_offset: int = int(node["name_offset"])
		var _name: String = ""
		while name_offset < string_table.size():
			var byte_val: int = string_table[name_offset]
			if byte_val == 0:
				break
			_name += char(byte_val)
			name_offset += 1

		var node_type: int = int(node["type"])
		var node_data_offset: int = int(node["data_offset"])
		var node_size: int = int(node["size"])

		if node_type == 0x0100:
			path_stack.append(_name)
			count_stack.append(node_size)
		elif node_type == 0x0000:
			var file_data: PackedByteArray = raw_data.slice(node_data_offset, node_data_offset + node_size)
			Dict.insert_nested(data, path_stack, _name, file_data)

		while count_stack.size() > 0 and current_index + 1 == count_stack[-1]:
			count_stack.pop_back()
			if path_stack.size() > 0:
				path_stack.pop_back()

	var new_arc: ARC = ARC.new()
	new_arc.filesystem = data
	new_arc.type = ARC_TYPE.U8
	new_arc.path = file_path
	return new_arc


func print_files(filesize: bool = false, indent: int = 0, data: Dictionary = {}) -> void:
	if !data:
		data = filesystem

	for key: Variant in data.keys():
		var value: Variant = data[key]
		var prefix: String = "\t".repeat(indent)
		if typeof(value) == TYPE_DICTIONARY:
			print("📁 [%s]" % (prefix + str(key)))
			print_files(filesize, indent + 1, value)
		else:
			var size_str: String = ""
			if filesize:
				size_str = " <%s>" % String.humanize_size(value.size())
			print("%s📄 %s %s" % [prefix, str(key), size_str])
	if data == filesystem and filesize:
		print("FULL SIZE: " + String.humanize_size(Dict.get_total_size(filesystem)))


func is_valid() -> bool:
	if [ARC_TYPE.U8].has(type):
		return true

	return false
