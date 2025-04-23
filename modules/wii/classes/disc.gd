class_name WiiDisc extends Object

var type: Variant

const SECTOR_SIZE: int = 0x8000;
const SECTOR_COUNT: int = 143432 * 2;
const DISC_HEADER_SIZE: int = 256;

static func open(path: String) -> WiiDisc:
	var new_disc: WiiDisc = WiiDisc.new()
	var disc_file: FileAccess = FileAccess.open(path, FileAccess.READ)
	disc_file.big_endian = true
	
	match disc_file.get_error():
		OK:
			WBFS.dump(disc_file)
			disc_file.close()
			return
		_:
			printerr(FileAccess.get_open_error())
	disc_file.close()
	return new_disc
	
func is_valid() -> bool:
	if type:
		return true
	else:
		return false
