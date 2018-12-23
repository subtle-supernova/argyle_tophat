extern crate libc;

use std::ffi::CStr;
use std::str;

use self::libc::{c_char, c_int};

#[no_mangle]
pub extern fn print_hi() {
    println!("Hello from FFI");
}

fn get_string_fram_c_chars(s: *const c_char) -> Option<String> {
    if s.is_null() {
        None
    } else {
        let to_rust = unsafe { CStr::from_ptr(s).to_str() };
        let rust_str = match to_rust {
            Err(e) => { println!("Err!, {:?}", e); "I CRASHED".to_owned() },
            Ok(o) => o.to_owned(),
        };
        Some(rust_str)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CStructExample {
    words: *const c_char,
    number: *const c_int
}

impl From<CStructExample> for (Option<String>, i32) {
    fn from(cse: CStructExample) -> (Option<String>, i32) {
        (get_string_fram_c_chars(cse.words), cse.number as i32)
    }
}

fn ptr_to_str(ptr: *const c_char) -> String {
    let fromptr = unsafe { CStr::from_ptr(ptr) };
    match str::from_utf8(fromptr.to_bytes()) { 
        Err(e) => { println!("utf8 error {:?}", e); "UTF8 ERR".to_owned() },
        Ok(o) => o.to_owned()
    }
}

#[no_mangle]
pub extern fn print_cstruct_example(cse: CStructExample) {
    println!("derive debug print: {:?}", cse);
    println!("string conversion print: {:?} - {:?}", get_string_fram_c_chars(cse.words).unwrap(), cse.number as i32);
    println!("Using std::str: {:?}", ptr_to_str(cse.words))
}
