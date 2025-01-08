extends Node
class_name ByteParser

## This function translates a PackedByteArray by grouping related bytes, specified by the format [br]
## The format goes: [br]
## l - long (i32) [br]
## h - short (i16) [br]
## b - 1 byte (i8) [br]
## x - skip byte [br]
## Xs - string (X bytes each) [br]
## * a capital letter represents the unsigned version of its lowercase counterpart [br]
## * capital X indicates a number
static func translate(format: String, data: PackedByteArray) -> Array:
	var expanded_format: String = _expand_format(format.replace(":", ""))
	var new_data: Array = []
	var offset: int = 0
	var index: int = 0
	
	while index < expanded_format.length():
		var char: String = expanded_format[index]
		
		if offset >= data.size():
			break
			
		match char.to_upper():
			"L":
				if offset + 4 <= data.size():
					if char == "l":
						new_data.append(get_int32_be(data, offset))
					else:
						new_data.append(get_uint32_be(data, offset))
					offset += 4
			"H":
				if offset + 2 <= data.size():
					if char == "h":
						new_data.append(get_int16_be(data, offset))
					else:
						new_data.append(get_uint16_be(data, offset))
					offset += 2
			"B":
				if offset + 1 <= data.size():
					if char == "b":
						# this isnt right but i dont care right now
						new_data.append(data[offset])
					else:
						new_data.append(data[offset])
					offset += 1
			"X": 
				if offset + 1 <= data.size():
					offset += 1
			_:
				# Check if it's a string format (Xs)
				if char.is_valid_int():
					var length: int = 0
					var num_str: String = ""
					
					while index < expanded_format.length() and expanded_format[index].is_valid_int():
						num_str += expanded_format[index]
						index += 1
					
					length = num_str.to_int()
					
					if index < expanded_format.length() and expanded_format[index] == "s" and offset + length <= data.size():
						new_data.append(get_string(data, offset, length))
						offset += length
					
					index -= 1
		index += 1
	return new_data

static func _expand_format(format: String) -> String:
	var expanded: String = ""
	var index: int = 0
	
	while index < format.length():
		# Get the count if this character is a number
		var count: int = 0
		var num_str: String = ""
		
		# Collect all consecutive digits
		while index < format.length() and format[index].is_valid_int():
			num_str += format[index]
			index += 1
			
		# If we have a number and the next char isn't 's', expand it
		if num_str != "" and index < format.length():
			var char: String = format[index]
			if char != "s":
				count = num_str.to_int()
				expanded += char.repeat(count)
			else:
				# For strings, keep the original format (like "16s")
				expanded += num_str + "s"
		else:
			# If no number or at end of string, just add the character
			if num_str != "":
				expanded += num_str  # Add back any collected numbers if we hit the end
			else:
				expanded += format[index]
			
		index += 1
		
	return expanded

## signed 32-bit integer from bytes 
static func get_int32_be(data: PackedByteArray, offset: int) -> int:
	var value: int = (
		(data[offset] << 24) |
		(data[offset + 1] << 16) |
		(data[offset + 2] << 8) |
		data[offset + 3]
	)
	# Handle sign bit
	if value & 0x80000000:
		value -= 0x100000000
	return value

## unsigned 32-bit integer from bytes
static func get_uint32_be(data: PackedByteArray, offset: int) -> int:
	return (
		(data[offset] << 24) |
		(data[offset + 1] << 16) |
		(data[offset + 2] << 8) |
		data[offset + 3]
	)

## signed 16-bit integer from bytes
static func get_int16_be(data: PackedByteArray, offset: int) -> int:
	var value: int = (
		(data[offset] << 8) |
		data[offset + 1]
	)
	# Handle sign bit
	if value & 0x8000:
		value -= 0x10000
	return value

## unsigned 16-bit integer from bytes
static func get_uint16_be(data: PackedByteArray, offset: int) -> int:
	return (
		(data[offset] << 8) | 
		data[offset + 1]
	)

## string from bytes
static func get_string(data: PackedByteArray, offset: int, length: int) -> String:
	var bytes: PackedByteArray = data.slice(offset, offset + length)
	return bytes.get_string_from_ascii()
