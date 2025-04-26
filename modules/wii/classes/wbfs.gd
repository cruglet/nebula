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
	
	#0x5-0x8
	var sectors: int = disc_file.get_32()
	
	#0x9
	var hd_sector_size: int = 1 << disc_file.get_8()
	
	#0xA
	var wbfs_sector_size: int = 1 << disc_file.get_8()
	
	#0xB-0xC
	disc_file.get_16()
	
	#0xC
	# these next 500 bytes here are actually represent
	# whether a WBFS slot is occupied, but realistically the first 
	# one should be the only one that is parsed.
	if disc_file.get_8() != 1:
		Singleton.error.emit(ERROR_HEADER + "Could not find a game within the WBFS file.")
	#endregion
	
	#region Game Info
	disc_file.seek(0x200)
	print(disc_file.get_buffer(0x100))
	#print(ga)
	#game.id = disc_file.get_buffer(0x16).get_string_from_ascii()
	#disc_file.get_buffer(16)
	#game.name = disc_file.get_buffer(0xD9).get_string_from_ascii()
	#
	#disc_file.seek(disc_file.get_position() + 0x64000)
	#
	#print(disc_file.get_buffer(0x1000))
	#endregion
	
	game.data_offset = wbfs_sector_size * 1 #(n)
	
	return game

static func extract(disc_file: FileAccess, start_offset: int) -> void:
	var game_data: PackedByteArray
	
	var data: PackedByteArray = disc_file.get_buffer(disc_file.get_length())
	
	var x: PackedByteArray
	x.resize(8)
	x.reverse()
	
	x.encode_u64(0, 369098752)
	
	print("String found at: %s" % Packer.search(data, x))
	#disc_file.seek(start_offset)
	#while true:
		#var block_data: PackedByteArray = disc_file.get_buffer(BLOCK_SIZE)
		#if block_data.size() == 0:
			#break
		#game_data.append_array(block_data)
