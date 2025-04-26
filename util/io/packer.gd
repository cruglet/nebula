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
