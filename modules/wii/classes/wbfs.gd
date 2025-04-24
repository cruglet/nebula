class_name WBFS extends Object

const ERROR_HEADER: String = "[color=red]Error parsing WBFS file.\n[/color]"
const BLOCK_SIZE: int = 256 * 1024


static func dump(disc_file: FileAccess) -> Dictionary:
	var game: Dictionary = {
		"id": "",
		"name": "",
		"data_offset": 0
	}
	
	#region WBFS Header Parsing
	#0x0-0x4
	if !disc_file.get_buffer(4).get_string_from_ascii() == "WBFS":
		Singleton.error.emit(ERROR_HEADER + "WBFS magic could not be found, please make sure\n you are opening a valid *.wbfs file.", "Ok", false)
		return game
	
	#0x5-0x9
	var sectors: int = disc_file.get_32()
	var hd_sector_size_shift: int = disc_file.get_8()
	var wbfs_sector_size_shift: int = disc_file.get_8()
	var wbfs_sector_size: int = 1 << wbfs_sector_size_shift
	
	#0xA (wbfs_version) + 0xB
	disc_file.get_8()
	disc_file.get_8()
	
	#0xC
	# these next 500 bytes here are actually represent
	# whether a WBFS slot is occupied, but realistically the first 
	# one should be the only one that is parsed.
	if disc_file.get_8() != 1:
		Singleton.error.emit(ERROR_HEADER + "Could not find a game within the WBFS file.")
	#endregion
	
	#region Game Info
	disc_file.seek(0x200)
	game.id = disc_file.get_buffer(6).get_string_from_ascii()
	disc_file.get_buffer(26)
	game.name = disc_file.get_buffer(32).get_string_from_ascii()
	#endregion
	
	game.data_offset = wbfs_sector_size * 1 #(n)
	
	return game

static func extract(disc_file: FileAccess, start_offset: int) -> void:
	disc_file.seek(start_offset)
	while true:
		var block_data: PackedByteArray = disc_file.get_buffer(BLOCK_SIZE)
		if block_data.size() == 0:
			break
		#game.data.append_array(block_data)
