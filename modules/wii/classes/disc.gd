class_name WiiDisc extends Object

enum DiscType {
	DISC_TYPE_NONE,
	DISC_TYPE_WBFS,
	DISC_TYPE_ISO,
}

const SECTOR_SIZE: int = 0x8000;
const SECTOR_COUNT: int = 0x46090;
const DISC_HEADER_SIZE: int = 256;
const COMMON_KEY: String = "ebe42a225e8593e448d9c5457381aaf7"

var type: Variant
var disc_path: String
var game_id: String

static func open(path: String) -> WiiDisc:
	var new_disc: WiiDisc = WiiDisc.new()
	var disc_file: FileAccess = FileAccess.open(path, FileAccess.READ)
	new_disc.disc_path = path
	disc_file.big_endian = true
	
	var game_id: String = ""
	match new_disc.get_disc_type(disc_file):
		DiscType.DISC_TYPE_WBFS:
			game_id = WBFS.get_game_id(disc_file)
		DiscType.DISC_TYPE_ISO:
			game_id = ISO.get_game_id(disc_file)
		_:
			Singleton.toast_notification("WBFS Error", "Invalid disc file! Please make sure you are opening a valid .wbfs file.")
			return WiiDisc.new()
	
	new_disc.game_id = game_id
	disc_file.close()
	return new_disc


# TODO this should be obtained by parsing "opening.bnr," but this works for now.
func get_banner() -> Texture2D:
	var banner: Texture2D = Nebula.GAME_LIST.get(Nebula.find_in_game_list(game_id)).banner
	
	if banner: 
		return banner
	else:
		return Texture2D.new()


func get_disc_type(file: FileAccess) -> DiscType:
	if WBFS.is_valid(file):
		return DiscType.DISC_TYPE_WBFS
		
	return DiscType.DISC_TYPE_NONE

#
func extract(to: String) -> void:
	if !disc_path:
		return
	
	var iso: ISO = ISO.new()
	var disc_file: FileAccess = FileAccess.open(disc_path, FileAccess.READ)
	disc_file.big_endian = true
	
	match get_disc_type(disc_file):
		DiscType.DISC_TYPE_WBFS:
			var wbfs: WBFS = WBFS.open(disc_file)
			iso.parse_wbfs(wbfs)
		DiscType.DISC_TYPE_ISO:
			Singleton.toast_notification("Under construction!", "ISO files are not supported yet.")
			return
		DiscType.DISC_TYPE_NONE:
			Singleton.toast_notification("File error", "Invalid/incompatible file detected.")
			return
	
	for filename: String in iso.filesystem:
		var new_file_location: String = to.path_join(filename)
		
		if not DirAccess.dir_exists_absolute(new_file_location.get_base_dir()):
			var error: Error = DirAccess.make_dir_recursive_absolute(new_file_location.get_base_dir())
			if error != OK:
				Singleton.toast_notification("Error code %s" % error, "Could not create directory to extract the game content.")
				return
		
		var new_file: FileAccess = FileAccess.open(new_file_location, FileAccess.WRITE)
		var file_info: Dictionary = iso.filesystem.get(filename)
		var file_data: PackedByteArray = iso.get_decrypted_data(file_info.offset, file_info.size)
		
		new_file.store_buffer(file_data)
		
		new_file.close()
	print("Extraction complete!")
	EventBus.project_ready.emit.call_deferred()
