class_name WiiDisc extends Object

var type: Variant
var game_info: Dictionary
var disc_path: String

const SECTOR_SIZE: int = 0x8000;
const SECTOR_COUNT: int = 0x46090;
const DISC_HEADER_SIZE: int = 256;
const COMMON_KEY: String = "ebe42a225e8593e448d9c5457381aaf7"


static func open(path: String) -> WiiDisc:
	var new_disc: WiiDisc = WiiDisc.new()
	var disc_file: FileAccess = FileAccess.open(path, FileAccess.READ)
	new_disc.disc_path = path
	disc_file.big_endian = true
	
	match disc_file.get_error():
		OK:
			#new_disc.game_info = 
			var wbfs: WBFS = WBFS.open(disc_file)
			var iso: ISO = ISO.new()
			iso.parse_wbfs(wbfs)
		_:
			printerr(FileAccess.get_open_error())
	disc_file.close()
	return new_disc


static func decrypt(title_key: PackedByteArray, game_id: String) -> PackedByteArray:
	var aes: AESContext = AESContext.new()
	var key: PackedByteArray = Packer.hex_string_to_bytes(COMMON_KEY)
	var iv: PackedByteArray = []
	var game_id_bytes: PackedByteArray = game_id.substr(0,4).to_ascii_buffer()
	iv.append_array(game_id_bytes)
	iv.append_array([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
	aes.start(AESContext.MODE_CBC_DECRYPT, key, iv)
	return aes.update(title_key)


func is_valid() -> bool:
	if type:
		return true
	else:
		return false
