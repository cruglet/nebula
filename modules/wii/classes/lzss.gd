class_name LZSS
extends Object


static func decompress(data: PackedByteArray) -> PackedByteArray:
	match data.get(0):
		0x11: return _decompress_lz11(data)
		_: return []

static func _decompress_lz11(data: PackedByteArray) -> PackedByteArray:
	if data.is_empty() or data[0] != 0x11:
		return PackedByteArray()
	
	var length: int
	var displacement: int
	var copy_dest: int
	
	var byte_temp: int
	var byte_one: int
	var byte_two: int
	var byte_three: int
	
	var decompressed_size: int = 0
	var source_index: int = 1
	
	for i: int in range(3):
		if source_index >= data.size():
			return PackedByteArray()
		decompressed_size |= (data[source_index] as int) << (i * 8)
		source_index += 1
	
	if decompressed_size == 0:
		for i: int in range(4):
			if source_index >= data.size():
				return PackedByteArray()
			decompressed_size |= (data[source_index] as int) << (i * 8)
			source_index += 1
	
	if decompressed_size > 0x800000:
		return PackedByteArray()
	
	var decompressed_data: PackedByteArray = PackedByteArray()
	decompressed_data.resize(decompressed_size)
	var current_size: int = 0
	
	while current_size < decompressed_size:
		if source_index >= data.size():
			return PackedByteArray()
		
		var flags: int = data[source_index]
		source_index += 1
		
		for bit_position: int in range(8):
			if current_size >= decompressed_size:
				break
			
			var flag: int = flags & (0x80 >> bit_position)
			
			if flag > 0:
				if source_index >= data.size():
					return PackedByteArray()
				byte_one = data[source_index]
				source_index += 1
				
				match byte_one >> 4:
					0:
						length = (byte_one as int) << 4
						if source_index >= data.size():
							return PackedByteArray()
						byte_temp = data[source_index]
						source_index += 1
						length |= (byte_temp as int) >> 4
						length += 0x11
						
						displacement = ((byte_temp & 0x0F) as int) << 8
						if source_index >= data.size():
							return PackedByteArray()
						byte_two = data[source_index]
						source_index += 1
						displacement |= byte_two as int
					1:
						if source_index + 2 >= data.size():
							return PackedByteArray()
						byte_temp = data[source_index]
						source_index += 1
						byte_two = data[source_index]
						source_index += 1
						byte_three = data[source_index]
						source_index += 1
						
						length = ((byte_one & 0x0F) as int) << 12
						length |= (byte_temp as int) << 4
						length |= (byte_two >> 4) as int
						length += 0x111
						
						displacement = ((byte_two & 0x0F) as int) << 8
						displacement |= byte_three as int
					_:
						length = ((byte_one >> 4) + 1) as int
						displacement = ((byte_one & 0x0F) as int) << 8
						if source_index >= data.size():
							return PackedByteArray()
						byte_two = data[source_index]
						source_index += 1
						displacement |= byte_two as int
				
				if displacement as int > current_size:
					return PackedByteArray()
				
				copy_dest = current_size as int
				
				for offset: int in range(length):
					if current_size < decompressed_size:
						var source_idx: int = (copy_dest - displacement - 1 + offset) as int
						
						if source_idx >= 0 and source_idx < decompressed_data.size():
							decompressed_data[current_size] = decompressed_data[source_idx]
							current_size += 1
				
				if current_size > decompressed_size:
					break
			else:
				if source_index >= data.size():
					return PackedByteArray()
				
				decompressed_data[current_size] = data[source_index]
				source_index += 1
				current_size += 1
				
				if current_size > decompressed_size:
					break
			
			if current_size == decompressed_size:
				decompressed_data.resize(decompressed_size)
				return decompressed_data
	
	return PackedByteArray()
