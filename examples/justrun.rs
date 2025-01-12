use std::ffi::CStr;

extern crate mec_mrbc_sys;

fn main() {
    let file = "test.rb".to_string();
    let out_file = format!("examples/{}.mrb\0", file);
    let in_file = format!("examples/{}\0", file);

    let args = [
        CStr::from_bytes_with_nul(b"mrbc\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"-v\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"-o\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(out_file.as_bytes()).unwrap().as_ptr(),
        CStr::from_bytes_with_nul(in_file.as_bytes()).unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"\0").unwrap().as_ptr(),
    ];

    unsafe {
        mec_mrbc_sys::mrbc_main(args.len() as i32, args.as_ptr() as *mut *mut i8);
    }
}