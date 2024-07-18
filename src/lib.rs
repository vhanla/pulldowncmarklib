use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use pulldown_cmark::{Event, Parser, Options};

#[no_mangle]
pub extern "C" fn invert_string(input_ptr: *const c_char) -> *mut c_char {
    let input_cstr = unsafe{CStr::from_ptr(input_ptr)};

    let input_str = std::str::from_utf8(input_cstr.to_bytes()).unwrap();//input_cstr.to_string_lossy().into_owned();
    // input_cstr.to_str() does similar too

    //let inverted_str: String = input_str.chars().rev().collect();


    //let output_cstr = CString::new(inverted_str).expect("CString::new failes");
    let mut html_output = String::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_GFM);
    let parser = pulldown_cmark::Parser::new_ext(&input_str, options);
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let output_cstr = CString::new(html_output).expect("");


    output_cstr.into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *const c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
    let _ = CString::from_raw(ptr as *mut _);
    }
}

