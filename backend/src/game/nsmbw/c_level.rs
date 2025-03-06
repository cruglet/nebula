use std::path::Path;

use crate::game::nsmbw::level::NSMBWLevel;

#[no_mangle]
pub extern "C" fn test_func() {
	let mut level = NSMBWLevel::new();
	let level_path = Path::new("./bin/01-02.arc");
	level.open(level_path);
	level.print_info();
}
