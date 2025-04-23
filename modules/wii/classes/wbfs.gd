class_name WBFS extends Object

const ERROR_HEADER: String = "[color=red]Error parsing WBFS file.\n[/color]"

var version: int
var game_id: String
var game_version: int
var game_data: PackedByteArray
var game_name: String

static func dump(disc_file: FileAccess) -> ISO:
	#region WBFS Header Parsing
	var game: ISO = ISO.new()
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
	# whether a slot is occupied, but realistically the first 
	# one should be the only one that is parsed.
	if disc_file.get_8() != 1:
		Singleton.error.emit(ERROR_HEADER + "Could not find a game within the WBFS file.")
	disc_file.seek(0x200)
	#endregion
	
	#region Game Header Parsing
	game.id = disc_file.get_buffer(6).get_string_from_ascii()
	disc_file.get_buffer(26)
	game.name = disc_file.get_buffer(32).get_string_from_ascii()
	
	# TODO figure out wtf is going on after this point
	# - parse disc info table stuff
	# - repack game "blocks" <-- prolly gonna be an ISO function though
	# -- for now just save the index that can serve as a pointer to the raw file data 
	#    in order to not absolutely demolish memory
	
	var table: PackedByteArray = disc_file.get_buffer(512)
	for i: int in range(table.size()):
		print(table[i])
		if table[i] == 0:
			continue
	return game
