class_name ISO
extends Object
## Sources:
## https://wiibrew.org/wiki/Wii_disc

const SHA1_SIZE: int = 0x400
const CLUSTER_SIZE: int = 0x8000

# Handled by parse_wbfs
var partition_offset: int
var data_size: int
var data_offset: int
var total_clusters: int
var title_key: PackedByteArray

# The product of parse_wbfs
var filesystem: Dictionary = {}


func parse_wbfs(wbfs: WBFS) -> ISO:
	var iso: ISO = ISO.new()
	var partition_count: int = Packer.decode_u32_be(wbfs.get_data(0x40000, 4))
	var partition_info_offset: int = Packer.decode_u32_be(wbfs.get_data(0x40004, 4)) << 2
	partition_offset = Packer.decode_u32_be(wbfs.get_data(partition_info_offset, 4)) << 2
	
	# Ticket parsing
	var ticket: PackedByteArray = wbfs.get_data(partition_offset, 0x2A4)
	var encrypted_title_key: PackedByteArray = ticket.slice(0x01BF, 0x01CF)
	# "Ticket id"
	var encrypted_title_iv: PackedByteArray = ticket.slice(0x01DC, 0x01E4)
	encrypted_title_iv.append_array([0, 0, 0, 0, 0, 0, 0, 0])
	
	title_key = Packer.aes_cbc_decrypt(
		encrypted_title_key,
		WiiDisc.COMMON_KEY.hex_decode(),
		encrypted_title_iv
	)
	
	data_offset = Packer.decode_u32_be(wbfs.get_data(partition_offset + 0x2B8, 4)) << 2
	data_size = Packer.decode_u32_be(wbfs.get_data(partition_offset + 0x2BC, 4)) << 2
	
	total_clusters = data_size / CLUSTER_SIZE
	
	# The first cluster of user data is important, since it provides the filesystem info.
	var filesystem_offset: int = Packer.decode_u32_be(_get_encrypted_wbfs_data(0x424, 4, wbfs)) << 2
	var filesystem_size: int = Packer.decode_u32_be(_get_encrypted_wbfs_data(0x428, 4, wbfs)) << 2
	var max_filesystem_size: int = Packer.decode_u32_be(_get_encrypted_wbfs_data(0x42C, 4, wbfs)) << 2
	
	# Parse the filesystem
	filesystem = parse_iso_filesystem(wbfs, filesystem_offset)
	
	return iso


func _get_encrypted_wbfs_data(offset: int, size: int, wbfs: WBFS) -> PackedByteArray:
	var data: PackedByteArray = PackedByteArray()
	# Calculate which cluster contains our offset
	var current_cluster: int = offset / (CLUSTER_SIZE - SHA1_SIZE)
	# Calculate the offset within the data portion of the cluster
	var cursor_offset: int = offset % (CLUSTER_SIZE - SHA1_SIZE)
	var remaining: int = size
	
	while remaining > 0:
		# The first 0x400 bytes (SHA1_SIZE) contains the encryption data for the data
		var cluster_sha1_offset: int = current_cluster * CLUSTER_SIZE
		# The remaining 0x7C00 bytes (CLUSTER_SIZE - SHA1_SIZE) contains the actual data
		var cluster_data_offset: int = cluster_sha1_offset + SHA1_SIZE
		# Get the IV for the cluster data
		var cluster_iv: PackedByteArray = wbfs.get_data(partition_offset + data_offset + cluster_sha1_offset + 0x3D0, 16)
		var encrypted: PackedByteArray = wbfs.get_data(partition_offset + data_offset + cluster_data_offset, CLUSTER_SIZE - SHA1_SIZE)
		var decrypted: PackedByteArray = Packer.aes_cbc_decrypt(encrypted, title_key, cluster_iv)
		var slice_len: int = min(decrypted.size() - cursor_offset, remaining)
		data.append_array(decrypted.slice(cursor_offset, cursor_offset + slice_len))
		cursor_offset = 0
		remaining -= slice_len
		current_cluster += 1
	return data


func parse_iso_filesystem(wbfs: WBFS, fst_offset: int) -> Dictionary:
	var result: Dictionary = {}
	
	var first_12: PackedByteArray = _get_encrypted_wbfs_data(fst_offset, 12, wbfs)
	var total_entries: int = Packer.decode_u32_be(first_12, 8)
	
	var fst_all: PackedByteArray = _get_encrypted_wbfs_data(
		fst_offset, total_entries * 12, wbfs)
	
	var str_tbl_off: int = fst_offset + total_entries * 12
	
	# stack items: {path:String, next:int}
	var dir_stack: Array[Dictionary] = []
	dir_stack.append({"path": "", "next": total_entries})  # root covers all
	
	for i: int in range(1, total_entries):
		# close finished directories
		while i >= dir_stack[-1]["next"]:
			dir_stack.pop_back()
		
		var e_off: int = i * 12
		var e: PackedByteArray = fst_all.slice(e_off, e_off + 12)
		
		var type_name: int = Packer.decode_u32_be(e)
		var e_type: int = (type_name >> 24) & 0xFF
		var name_off: int = type_name & 0x00FF_FFFF
		
		# read null‑terminated name
		var name_pos: int = str_tbl_off + name_off
		var name: String = ""
		while true:
			var ch: PackedByteArray = _get_encrypted_wbfs_data(name_pos, 1, wbfs)
			if ch[0] == 0:
				break
			name += char(ch[0])
			name_pos += 1
		
		if e_type == 1:  # directory
			var next_idx: int = Packer.decode_u32_be(e, 8)
			var dir_path: String = dir_stack[-1]["path"] + name + "/"
			dir_stack.append({"path": dir_path, "next": next_idx})
		else: # file
			var file_off: int = Packer.decode_u32_be(e, 4) * 4
			var file_size: int = Packer.decode_u32_be(e, 8)
			var full_path: String = dir_stack[-1]["path"] + name
			result[full_path] = {"size": file_size, "offset": file_off}
	return result


# TODO
func is_encrypted() -> bool:
	return true
