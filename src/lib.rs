use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use pulldown_cmark::{Options};

#[no_mangle]
pub extern "C" fn strtomarkdown(input_ptr: *const c_char, options_flags: u32) -> *mut c_char {
    let input_cstr = unsafe{CStr::from_ptr(input_ptr)};

    let input_str = std::str::from_utf8(input_cstr.to_bytes()).unwrap();
    let mut html_output = String::new();
    let options = Options::from_bits(options_flags).unwrap_or(Options::empty());
    //options.insert(Options::ENABLE_TABLES);
    //options.insert(Options::ENABLE_FOOTNOTES);
    //options.insert(Options::ENABLE_STRIKETHROUGH);
    //options.insert(Options::ENABLE_SMART_PUNCTUATION);
    //options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    //options.insert(Options::ENABLE_TASKLISTS);
    //options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    //options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    //options.insert(Options::ENABLE_MATH);
    //options.insert(Options::ENABLE_GFM);
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

