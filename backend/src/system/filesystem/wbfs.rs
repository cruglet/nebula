use std::ffi::CStr;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

static PREDEFINED_HEADERS: [&[u8]; 6] = [
    b"SMNE01",
    b"SMNP01",
    b"SMNJ01",
    b"SMNK01",
    b"SMNW01",
    b"SMNC01",
];

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn validate_wbfs(path: *const i8) -> i8 {
    if path.is_null() {
        return -1;
    }

    let c_str = CStr::from_ptr(path);
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let file_result = File::open(path_str);
    if file_result.is_err() {
        return -1;
    }

    let mut file = file_result.unwrap();

    if file.seek(SeekFrom::Start(512)).is_err() {
        return -1;
    }

    let length = 8;
    let mut buffer = vec![0u8; length];
    if file.read_exact(&mut buffer).is_err() {
        return -1;
    }

    for (index, check_string) in PREDEFINED_HEADERS.iter().enumerate() {
        if buffer.windows(check_string.len()).any(|window| window == *check_string) {
            return index as i8;
        }
    }

    -1
}
