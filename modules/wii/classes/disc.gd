class_name WiiDisc extends Object

var type: Variant

var game_info: Dictionary
var disc_path: String

const SECTOR_SIZE: int = 0x8000;
const SECTOR_COUNT: int = 143432 * 2;
const DISC_HEADER_SIZE: int = 256;

static func open(path: String) -> WiiDisc:
	var new_disc: WiiDisc = WiiDisc.new()
	var disc_file: FileAccess = FileAccess.open(path, FileAccess.READ)
	new_disc.disc_path = path
	disc_file.big_endian = true
	
	match disc_file.get_error():
		OK:
			new_disc.game_info = WBFS.dump(disc_file)
		_:
			printerr(FileAccess.get_open_error())
	disc_file.close()
	return new_disc
	
func is_valid() -> bool:
	if type:
		return true
	else:
		return false
