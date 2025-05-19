class_name WBFS extends Object

const ERROR_HEADER: String = "[color=red]Error parsing WBFS file.\n[/color]"
const BLOCK_SIZE: int = 256 * 1024

var sector_size: int = -1
# this could be an array technically but I'm not 100% sure if all WBFS
# files have their wlba table in ascending numerical order so just in
# case ima do it like this
var wlba_map: Dictionary[int, int] = {}
var disc_file: FileAccess
var total_blocks: int
var game_id: String

static func open(wbfs_file: FileAccess) -> WBFS:
	var wbfs: WBFS = WBFS.new()
	
	wbfs.disc_file = wbfs_file
	
	#region WBFS Header Parsing
	#0x0-0x4
	if !wbfs_file.get_buffer(4).get_string_from_ascii() == "WBFS":
		Singleton.error.emit(ERROR_HEADER + "WBFS magic could not be found, please make sure\n you are opening a valid *.wbfs file.", "Ok", false)
		return wbfs
	
	var sectors: int = wbfs_file.get_32() #0x5-0x8
	var hd_sector_size: int = 1 << wbfs_file.get_8() #0x9
	var wbfs_sector_size: int = 1 << wbfs_file.get_8() #0xA
	var blocks_per_disc: int = (WiiDisc.SECTOR_COUNT * WiiDisc.SECTOR_SIZE + wbfs_sector_size - 1) / wbfs_sector_size
	
	wbfs_file.get_16() #0xB-0xC
	
	#0xC
	# these next 500 bytes here actually represent
	# whether a WBFS slot is occupied, but realistically the first 
	# one should be the only one that is parsed.
	if wbfs_file.get_8() != 1:
		Singleton.error.emit(ERROR_HEADER + "Could not find a game within the WBFS file.")
	#endregion
	
	#region Disc Header
	wbfs_file.seek(hd_sector_size)
	
	wbfs.game_id = wbfs_file.get_buffer(6).get_string_from_ascii()
	
	wbfs_file.get_8() # Disc Number
	wbfs_file.get_8() # Disc Version
	wbfs_file.get_8() # Audio Streaming
	wbfs_file.get_8() # Streaming buffer size
	wbfs_file.get_buffer(14) # Unused?
	
	var wii_magic: int = wbfs_file.get_32()
	var gamecube_magic: int = wbfs_file.get_32()
	
	var game_name: String = wbfs_file.get_buffer(64).get_string_from_ascii()
	
	if !(game_name and wbfs.game_id):
		Singleton.error.emit(ERROR_HEADER + "Invalid/corrupted game file.")
		return WBFS.new()
	
	var hashing_disabled: bool = bool(wbfs_file.get_8())
	var encryption_disabled: bool = bool(wbfs_file.get_8())
	#endregion
	
	#region WLBA Table parsing
	wbfs_file.seek(hd_sector_size + 256)
	
	for i: int in range(1, blocks_per_disc / 2):
		var wlba: int = wbfs_file.get_16() # Grab the current slot
		# Check if the slot is occupied
		if wlba > 0:
			# Assign the slot to its block address.
			wbfs.wlba_map.set(wlba, i)
			wbfs.total_blocks = max(wbfs.total_blocks, i)
	#endregion
	
	
	wbfs.sector_size = wbfs_sector_size
	
	return wbfs

## Translates ISO offsets into WBFS offsets and then returns the requested data.
func get_data(address: int, size: int) -> PackedByteArray:
	var sector_index: int = address / sector_size
	var sector_offset: int = address % sector_size

	# Look for the WBFS block that maps to this ISO block (sector_index + 1)
	var iso_block_index: int = sector_index + 1
	var wbfs_block: int = -1
	
	for wbfs_block_num: int in wlba_map.keys():
		if wlba_map[wbfs_block_num] == iso_block_index:
			wbfs_block = wbfs_block_num
			break

	if wbfs_block == -1:
		push_error("Invalid WBFS block index for ISO block %d" % iso_block_index)
		return PackedByteArray()

	var wbfs_address: int = wbfs_block * sector_size + sector_offset
	disc_file.seek(wbfs_address)
	var buf: PackedByteArray = disc_file.get_buffer(size)
	return buf


func reconstruct(path: String) -> void:
	var write_file: FileAccess = FileAccess.open(path, FileAccess.WRITE)
	write_file.big_endian = true
	write_file.resize(total_blocks * sector_size)

	for wlba_index: int in wlba_map:
		var iso_block_index: int = wlba_map[wlba_index]
		var read_offset: int = wlba_index * sector_size
		var write_offset: int = (iso_block_index - 1) * sector_size

		disc_file.seek(read_offset)
		var buffer: PackedByteArray = disc_file.get_buffer(sector_size)

		write_file.seek(write_offset)
		write_file.store_buffer(buffer)

	# ensure last byte exists so the file isn't sparse
	#write_file.seek(total_blocks * sector_size - 1)
	#write_file.store_8(0)
