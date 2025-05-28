class_name ISO
extends Object

## Refer to nebula-md/documentation for more documentation and extra sources:
## https://github.com/cruglet/nebula-md/tree/documentation

# Constants for WBFS parsing
const SHA1_BLOCK_SIZE: int = 0x400 # Size of SHA1 hash block (1024 bytes)
const CLUSTER_SIZE: int = 0x8000 # Size of a cluster (32768 bytes)
const DATA_BLOCK_SIZE: int = CLUSTER_SIZE - SHA1_BLOCK_SIZE # Size of data portion in a cluster (31744 bytes)

# Variables populated during WBFS parsing
var wii_partition_offset: int # Offset to the Wii partition in the WBFS
var wii_partition_data_size: int # Size of the partition data
var wii_partition_data_offset: int # Offset to the actual data within the partition
var total_data_clusters: int # Total number of clusters in the data
var decryption_key: PackedByteArray # Key used for decrypting the partition data
var wbfs_data: WBFS

## Maps file paths to their metadata (size and offset). [br]
## Files are stored with their offsets and size instead of
## their actual content so that your computer doesn't explode
var filesystem: Dictionary = {} 
var game_id: String

## Parse a WBFS disk image to extract an ISO image. [br]
func parse_wbfs(wbfs: WBFS) -> void:
	wbfs_data = wbfs
	# Parse WBFS header to locate the partition
	var partition_count: int = Packer.decode_u32_be(wbfs.get_data(0x40000, 4))
	var partition_table_offset: int = Packer.decode_u32_be(wbfs.get_data(0x40004, 4)) << 2
	wii_partition_offset = Packer.decode_u32_be(wbfs.get_data(partition_table_offset, 4)) << 2
	
	# Extract and decrypt the title key from the partition's ticket
	extract_title_key_from_wbfs(wbfs)

	# Get partition data information
	wii_partition_data_offset = Packer.decode_u32_be(wbfs.get_data(wii_partition_offset + 0x2B8, 4)) << 2
	wii_partition_data_size = Packer.decode_u32_be(wbfs.get_data(wii_partition_offset + 0x2BC, 4)) << 2

	total_data_clusters = wii_partition_data_size / CLUSTER_SIZE

	# Extract filesystem information from the first data cluster
	var filesystem_offset: int = Packer.decode_u32_be(get_decrypted_data(0x424, 4, wbfs)) << 2
	var filesystem_size: int = Packer.decode_u32_be(get_decrypted_data(0x428, 4, wbfs)) << 2
	var max_filesystem_size: int = Packer.decode_u32_be(get_decrypted_data(0x42C, 4, wbfs)) << 2

	# Parse the filesystem to build the file entries dictionary
	filesystem = parse_iso_filesystem_from_wbfs(wbfs, filesystem_offset)

	game_id = wbfs.game_id

## Extracts tht title key from the partition's ticket
func extract_title_key_from_wbfs(wbfs: WBFS) -> void:
	var ticket_data: PackedByteArray = wbfs.get_data(wii_partition_offset, 0x2A4)
	
	# Extract the encrypted title key and initialization vector
	var encrypted_title_key: PackedByteArray = ticket_data.slice(0x01BF, 0x01CF)
	var title_key_iv: PackedByteArray = ticket_data.slice(0x01DC, 0x01E4)
	
	# Pad the IV to the required 16 bytes
	title_key_iv.append_array([0, 0, 0, 0, 0, 0, 0, 0])
	
	# Decrypt the title key using the common key
	decryption_key = Packer.aes_cbc_decrypt(
		encrypted_title_key,
		WiiDisc.COMMON_KEY.hex_decode(),
		title_key_iv
	)

## Retrieves and decrypts data from the WBFS image at the specified offset and size. [br]
## [param offset]: Offset within the partition data [br]
## [param size]: The size of data to retrieve, in bytes. [br]
## [param wbfs]: A WBFS instance
func get_decrypted_data(offset: int, size: int, wbfs: WBFS = wbfs_data) -> PackedByteArray:
	var decrypted_data: PackedByteArray = []
	
	# Calculate which cluster contains our offset
	var current_cluster: int = offset / DATA_BLOCK_SIZE
	
	# Calculate the offset within the data portion of the cluster
	var cluster_data_offset: int = offset % DATA_BLOCK_SIZE
	var bytes_remaining: int = size
	
	while bytes_remaining > 0:
		# Get the cluster's decrypted data
		var cluster_data: PackedByteArray = decrypt_cluster(current_cluster, wbfs)

		# Calculate how many bytes to take from this cluster
		var bytes_to_take: int = min(cluster_data.size() - cluster_data_offset, bytes_remaining)

		# Add the required slice of data to our result
		decrypted_data.append_array(cluster_data.slice(cluster_data_offset, cluster_data_offset + bytes_to_take))

		# Move to the next cluster
		cluster_data_offset = 0
		bytes_remaining -= bytes_to_take
		current_cluster += 1

	return decrypted_data


## Decrypts a specific cluster from the WBFS image.
func decrypt_cluster(cluster_index: int, wbfs: WBFS) -> PackedByteArray:
	# Calculate offsets for the SHA1 block and data block
	var cluster_start_offset: int = cluster_index * CLUSTER_SIZE
	var sha1_block_offset: int = cluster_start_offset
	var data_block_offset: int = sha1_block_offset + SHA1_BLOCK_SIZE

	# Get the initialization vector for decryption from the SHA1 block
	var cluster_iv_offset: int = wii_partition_offset + wii_partition_data_offset + sha1_block_offset + 0x3D0
	var cluster_iv: PackedByteArray = wbfs.get_data(cluster_iv_offset, 16)

	# Get the encrypted data block
	var encrypted_data_offset: int = wii_partition_offset + wii_partition_data_offset + data_block_offset
	var encrypted_data: PackedByteArray = wbfs.get_data(encrypted_data_offset, DATA_BLOCK_SIZE)

	# Decrypt the data block
	return Packer.aes_cbc_decrypt(encrypted_data, decryption_key, cluster_iv)


## Parses the ISO's filesystem to build a dictionary of file entries.
func parse_iso_filesystem_from_wbfs(wbfs: WBFS, filesystem_offset: int) -> Dictionary:
	var file_system_entries: Dictionary = {}

	# Read the filesystem table header to get the total number of entries
	var filesystem_header: PackedByteArray = get_decrypted_data(filesystem_offset, 12, wbfs)
	var total_entry_count: int = Packer.decode_u32_be(filesystem_header, 8)

	# Read the entire filesystem table
	var filesystem_table: PackedByteArray = get_decrypted_data(
		filesystem_offset, total_entry_count * 12, wbfs)

	# Calculate the offset to the string table (names of files and directories)
	var string_table_offset: int = filesystem_offset + total_entry_count * 12

	# Stack for tracking directory hierarchy (path and next sibling index)
	var directory_stack: Array[Dictionary] = []
	directory_stack.append({ "path": "", "next": total_entry_count }) # Root directory covers all entries

	# Parse each entry in the filesystem table (starting from 1, as 0 is the root)
	for entry_index: int in range(1, total_entry_count):
		# Pop finished directories from the stack
		while entry_index >= directory_stack[-1]["next"]:
			directory_stack.pop_back()

		# Calculate the offset of this entry in the filesystem table
		var entry_offset: int = entry_index * 12
		var entry_data: PackedByteArray = filesystem_table.slice(entry_offset, entry_offset + 12)

		# First 4 bytes contain type (high byte) and name offset (low 3 bytes)
		var type_name_field: int = Packer.decode_u32_be(entry_data)
		var entry_type: int = (type_name_field >> 24) & 0xFF
		var name_offset: int = type_name_field & 0x00FF_FFFF

		# Read the name from the string table
		var entry_name: String = read_null_string(string_table_offset + name_offset, wbfs)

		if entry_type == 1: # Directory
			var next_sibling_index: int = Packer.decode_u32_be(entry_data, 8)
			var directory_path: String = directory_stack[-1]["path"] + entry_name + "/"
			directory_stack.append({ "path": directory_path, "next": next_sibling_index })
		else: # File
			var file_offset: int = Packer.decode_u32_be(entry_data, 4) * 4 # Offset is multiplied by 4
			var file_size: int = Packer.decode_u32_be(entry_data, 8)
			var file_path: String = directory_stack[-1]["path"] + entry_name

			file_system_entries[file_path] = { "size": file_size, "offset": file_offset }

	return file_system_entries


## Reads a string at a given offset. [br]
## - This was made a seperate function to be able to read between clusters
func read_null_string(offset: int, wbfs: WBFS) -> String:
	var string_value: String = ""
	var current_offset: int = offset

	while true:
		var character_data: PackedByteArray = get_decrypted_data(current_offset, 1, wbfs)
		if character_data[0] == 0: # Null terminator
			break
		string_value += char(character_data[0])
		current_offset += 1

	return string_value


static func get_game_id(file: FileAccess) -> String:
	var cursor_pos: int = file.get_position()
	file.seek(0)
	var id: String = file.get_buffer(0x6).get_string_from_ascii()
	file.seek(cursor_pos)
	return id
