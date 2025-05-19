class_name ISO
extends Object
## Sources:
## https://wiibrew.org/wiki/Wii_disc

const CLUSTER_SIZE: int = 0x8000

var filesystem: Dictionary = {}


func parse_wbfs(wbfs: WBFS) -> ISO:
	var iso: ISO = ISO.new()
	var partition_count: int = Packer.decode_u32_be(wbfs.get_data(0x40000, 4))
	var partition_info_offset: int = Packer.decode_u32_be(wbfs.get_data(0x40004, 4)) << 2
	var partition_offset: int = Packer.decode_u32_be(wbfs.get_data(partition_info_offset, 4)) << 2
	
	# Ticket parsing
	var ticket: PackedByteArray = wbfs.get_data(partition_offset, 0x2A4)
	
	var encrypted_title_key: PackedByteArray = ticket.slice(0x01BF, 0x01CF)
	
	# "Ticket id"
	var encrypted_title_iv: PackedByteArray = ticket.slice(0x01D0, 0x01D8)
	encrypted_title_iv.append_array([0, 0, 0, 0, 0, 0, 0, 0])
	
	var title_key: PackedByteArray = Packer.aes_cbc_decrypt(
		encrypted_title_key,
		Packer.hex_string_to_bytes(WiiDisc.COMMON_KEY), 
		encrypted_title_iv
	)
	
	# Get TMD (Title Metadata) size and offset
	var tmd_size: int = Packer.decode_u32_be(wbfs.get_data(partition_offset + 0x2A4, 4))
	var tmd_offset: int = Packer.decode_u32_be(wbfs.get_data(partition_offset + 0x2A8, 4)) << 2
	var tmd: PackedByteArray = wbfs.get_data(partition_offset + tmd_offset, tmd_size)
	
	# Get data offset and size
	var data_offset: int = Packer.decode_u32_be(wbfs.get_data(partition_offset + 0x2B8, 4)) << 2
	var data_size: int = Packer.decode_u32_be(wbfs.get_data(partition_offset + 0x2BC, 4)) << 2
	
	# Check if disc is encrypted (0x61 in header is non-zero)
	var disc_header: PackedByteArray = wbfs.get_data(0, 0x100)
	var is_encrypted: bool = disc_header[0x61] == 0
	
	# Determine actual partition data offset
	var partition_data_offset: int = partition_offset + data_offset
	var actual_data_offset: int
	
	if is_encrypted:
		actual_data_offset = 0x20000  # 0x00020000 for normal discs
	else:
		actual_data_offset = 0x8000   # 0x00008000 for unencrypted discs
	
	# Decrypt and extract partition data
	if is_encrypted:
		# Process data in clusters of 0x8000 bytes
		var cluster_size: int = 0x8000
		var hash_size: int = 0x400
		var data_per_cluster: int = cluster_size - hash_size  # 0x7C00
		
		for i: int in range(0, data_size, cluster_size):
			var cluster_offset: int = partition_data_offset + actual_data_offset + i
			
			# Get hash data (H3 table entries)
			var hash_data: PackedByteArray = wbfs.get_data(cluster_offset, hash_size)
			
			# Decrypt hash data with null IV
			var null_iv: PackedByteArray = PackedByteArray()
			null_iv.resize(16)  # 16 bytes of zeros
			var decrypted_hash: PackedByteArray = Packer.aes_cbc_decrypt(hash_data, title_key, null_iv)
			
			# Get encrypted data
			var encrypted_data: PackedByteArray = wbfs.get_data(cluster_offset + hash_size, data_per_cluster)
			
			# Decrypt data using title key and IV from the hash data
			# For each 0x400 bytes of data, we need to use the corresponding hash entry as IV
			for j: int in range(0, data_per_cluster, 0x400):
				var data_block: PackedByteArray = encrypted_data.slice(j, j + 0x400)
				var data_iv: PackedByteArray
				
				# Calculate IV index in hash data (each hash is 20 bytes, but we use only first 16)
				var hash_index: int = j / 0x400
				data_iv = decrypted_hash.slice(hash_index * 20, hash_index * 20 + 16)
				
				var decrypted_block: PackedByteArray = Packer.aes_cbc_decrypt(data_block, title_key, data_iv)
				
				breakpoint
				# Append decrypted data to ISO
				#iso.append_data(decrypted_block)
	else:
		# For unencrypted discs, just copy the data directly
		var raw_data: PackedByteArray = wbfs.get_data(partition_data_offset + actual_data_offset, data_size)
		iso.append_data(raw_data)
	
	return iso
	
	
	
	
	## Create IV from title ID
	#var title_id_iv: PackedByteArray = []
	#title_id_iv.append_array(wbfs.get_data(partition_offset + 0x01DC, 0x8))
	## Padding the IV with zeros
	#for i in range(8):
		#title_id_iv.append(0)
	#
	## Decrypt the title key using the common key and title ID as IV
	#var title_key: PackedByteArray = Packer.aes_cbc_decrypt(
		#encrypted_title_key, 
		#Packer.hex_string_to_bytes(WiiDisc.COMMON_KEY), 
		#title_id_iv
	#)
	#
	## Get data offset from ticket
	#var data_offset: int = Packer.decode_u32_be(wbfs.get_data(partition_offset + 0x2B8, 4)) << 2
	#var data_cluster_offset: int = partition_offset + data_offset
	#
	## Read and decrypt a cluster
	#var cluster_data: PackedByteArray = wbfs.get_data(data_cluster_offset, 0x8000)
	#
	## Extract the IV for this cluster from offset 0x3D0
	#var cluster_iv: PackedByteArray = cluster_data.slice(0x3D0, 0x3E0)
	#
	## Get encrypted data from offset 0x400
	#var encrypted_data: PackedByteArray = cluster_data.slice(0x400, 0x8000)
	#
	## Decrypt the data using the title key and cluster-specific IV
	#var decrypted_data: PackedByteArray = Packer.aes_cbc_decrypt(
		#encrypted_data,
		#title_key,
		#cluster_iv
	#)
	## Now you can process decrypted_data
	## ...
	
	
	#var decrypted_chunk: PackedByteArray = Packer.aes_cbc_decrypt(encrypted_chunk, title_key)
	
	#wbfs.reconstruct("res://" + wbfs.game_id + ".iso")
	
	# The IV is actually stored in the *encrypted* data, not the other way around
	#var data_iv: PackedByteArray = encrypted_chunk.slice(0x3D0, 0x3DF)
	#var encrypted_data: PackedByteArray = decrypted_chunk.slice()

# TODO
func is_encrypted() -> bool:
	return true
