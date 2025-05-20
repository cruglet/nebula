class_name Packer extends Node

static func decode_u16_be(array: PackedByteArray, offset: int = 0) -> int:
	var be_arr: PackedByteArray = array.slice(offset, offset + 2)
	be_arr.reverse()
	
	return be_arr.decode_u16(0)
	
static func decode_s16_be(array: PackedByteArray, offset: int = 0) -> int:
	var be_arr: PackedByteArray = array.slice(offset, offset + 2)
	be_arr.reverse()
	
	return be_arr.decode_s16(0)
	
static func decode_u32_be(array: PackedByteArray, offset: int = 0) -> int:
	var be_arr: PackedByteArray = array.slice(offset, offset + 4)
	be_arr.reverse()
	
	return be_arr.decode_u32(0)

static func decode_s32_be(array: PackedByteArray, offset: int = 0) -> int:
	var be_arr: PackedByteArray = array.slice(offset, offset + 4)
	be_arr.reverse()
	
	return be_arr.decode_s32(0)

static func decode_u64_be(array: PackedByteArray, offset: int = 0) -> int:
	var be_arr: PackedByteArray = array.slice(offset, offset + 8)
	be_arr.reverse()
	
	return be_arr.decode_u64(0)

static func decode_s64_be(array: PackedByteArray, offset: int = 0) -> int:
	var be_arr: PackedByteArray = array.slice(offset, offset + 8)
	be_arr.reverse()
	
	return be_arr.decode_s64(0)

static func decode_float_be(array: PackedByteArray, offset: int = 0) -> int:
	var be_arr: PackedByteArray = array.slice(offset, offset + 4)
	be_arr.reverse()
	
	return be_arr.decode_float(0)

static func search(array: PackedByteArray, sequence: PackedByteArray) -> int:
	var len_array: int = array.size()
	var len_sequence: int = sequence.size()

	for i: int in range(len_array - len_sequence + 1):
		if array.slice(i, i + len_sequence) == sequence:
			return i
	
	return -1

static func search_file(file: FileAccess, sequence: PackedByteArray, offset: int = 0) -> int:
	var original_position: int = file.get_position()
	file.seek(offset)
	while file.get_position() < file.get_length():
		var check_sequence: PackedByteArray = file.get_buffer(sequence.size())
		if check_sequence == sequence:
			return file.get_position() - sequence.size()
	file.seek(original_position)
	return -1

static func aes_cbc_decrypt(encrypted_data: PackedByteArray, key: PackedByteArray, iv: PackedByteArray = []) -> PackedByteArray:
	assert(encrypted_data.size() % 16 == 0)
	var aes: AESContext = AESContext.new()
	var _iv: PackedByteArray = iv.duplicate()
	if _iv.size() != 16:
		_iv.resize(16)
		_iv.fill(0)
	aes.start(AESContext.MODE_CBC_DECRYPT, key, _iv)
	var result: PackedByteArray = aes.update(encrypted_data)
	return result
