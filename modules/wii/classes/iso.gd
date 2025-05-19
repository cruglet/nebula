class_name ISO
extends Object
## Sources:
## https://wiibrew.org/wiki/Wii_disc

var filesystem: Dictionary = {}

func parse_wbfs(wbfs: WBFS) -> ISO:
	var iso: ISO = ISO.new()
	
	var partition_count: int = Packer.decode_u32_be(wbfs.get_data(0x40000, 4))
	var partition_info_offset: int = Packer.decode_u32_be(wbfs.get_data(0x40004, 4)) << 2
	
	var partition_offset: int = Packer.decode_u32_be(wbfs.get_data(partition_info_offset, 4)) << 2
	var encrypted_title_key: PackedByteArray = wbfs.get_data(partition_offset + 0x01BF, 0x10)
	
	var title_key: PackedByteArray = WiiDisc.decrypt(encrypted_title_key, wbfs.game_id)
	
	
	#for i: int in range(10):
	var raw_block: PackedByteArray = wbfs.get_data(partition_offset + 0x20000, 0x8000)
	var encrypted_chunk: PackedByteArray = raw_block.slice(0x800, 0x800 + 0x7000)
	
	var decrypted_chunk: PackedByteArray = Packer.aes_cbc_decrypt(encrypted_chunk, title_key)
	
	breakpoint
	
	return iso

# TODO
func is_encrypted() -> bool:
	return true
