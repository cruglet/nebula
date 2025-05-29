class_name Packer extends Node
## Binary-related encoding/decoding operations


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


static func load_rgb5a4(bytes: PackedByteArray, width: int, height: int) -> ImageTexture:
	var decoded: PackedByteArray = []
	for i: int in range(0, bytes.size(), 2):
		var word: int = (bytes[i] << 8) | bytes[i + 1]
		var r: int = ((word >> 11) & 0x1F) * 255 / 31
		var g: int = ((word >> 6) & 0x1F) * 255 / 31
		var b: int = ((word >> 1) & 0x1F) * 255 / 31
		var a: int = (word & 0x01) * 255  # Some variants have 4-bit alpha, others 1-bit
		decoded.push_back(r)
		decoded.push_back(g)
		decoded.push_back(b)
		decoded.push_back(a)
	var img: Image = Image.create_from_data(width, height, false, Image.FORMAT_RGBA8, decoded)
	return ImageTexture.create_from_image(img)


static func prepare_rgb4a3_luts() -> Array[PackedInt32Array]:
	var rgb4a3lut: PackedInt32Array = []
	rgb4a3lut.resize(0x10000)
	var rgb4a3lut_no_alpha: PackedInt32Array = []
	rgb4a3lut_no_alpha.resize(0x10000)
	
	for d: int in range(0x8000):
		var alpha: int
		var red: int
		var green: int
		var blue: int
		if true:
			alpha = d >> 12
			alpha = alpha << 5 | alpha << 2 | alpha >> 1
		else:
			alpha = 0xFF
		red = ((d >> 8) & 0xF) * 17
		green = ((d >> 4) & 0xF) * 17
		blue = (d & 0xF) * 17
		rgb4a3lut[d] = blue | (green << 8) | (red << 16) | (alpha << 24)
	
	for d: int in range(0x8000):
		var red: int = (d >> 10) << 3 | (d >> 2) & 0x7
		var green: int = ((d >> 5) & 0x1F) << 3 | ((d >> 5) & 0x1F) >> 2
		var blue: int = (d & 0x1F) << 3 | (d & 0x1F) >> 2
		rgb4a3lut[d + 0x8000] = blue | (green << 8) | (red << 16) | 0xFF000000
	
	var result: Array[PackedInt32Array] = [rgb4a3lut, rgb4a3lut_no_alpha]
	return result

static func rgb4a3_decode(tex: PackedByteArray, use_alpha: bool) -> ImageTexture:
	var luts: Array[PackedInt32Array] = prepare_rgb4a3_luts()
	var rgb4a3lut: PackedInt32Array = luts[0]
	var rgb4a3lut_no_alpha: PackedInt32Array = luts[1]
	var lut: PackedInt32Array = rgb4a3lut if use_alpha else rgb4a3lut_no_alpha

	var tx: int = 0
	var ty: int = 0
	var tex_index: int = 0
	var dest: PackedInt32Array = PackedInt32Array()
	dest.resize(262144)

	for i: int in range(16384):
		var temp1: int = (i / 256) % 8
		if temp1 == 0 or temp1 == 7:
			for skip: int in range(32):
				tex_index += 1
		else:
			var temp2: int = i % 8
			if temp2 == 0 or temp2 == 7:
				for skip: int in range(32):
					tex_index += 1
			else:
				for y: int in range(ty, ty + 4):
					for x: int in range(tx, tx + 4):
						if tex_index < tex.size() and tex_index + 1 < tex.size():
							var val1: int = tex[tex_index]
							tex_index += 1
							var val2: int = tex[tex_index]
							tex_index += 1
							var pixel_value: int = (val1 << 8) | val2
							if pixel_value < lut.size():
								dest[x + y * 1024] = lut[pixel_value]
		tx += 4
		if tx >= 1024:
			tx = 0
			ty += 4

	var img_data: PackedByteArray = PackedByteArray()
	img_data.resize(1024 * 256 * 4)

	for i: int in range(dest.size()):
		var pixel: int = dest[i]
		var a: int = (pixel >> 24) & 0xFF
		var r: int = (pixel >> 16) & 0xFF
		var g: int = (pixel >> 8) & 0xFF
		var b: int = pixel & 0xFF
		
		var byte_index: int = i * 4
		img_data[byte_index] = r
		img_data[byte_index + 1] = g
		img_data[byte_index + 2] = b
		img_data[byte_index + 3] = a

	var image: Image = Image.create_from_data(1024, 256, false, Image.FORMAT_RGBA8, img_data)
	var texture: ImageTexture = ImageTexture.new()
	texture.set_image(image)
	return texture
