use std::ffi::{CString, CStr};
use std::str;
use libc::{c_char};

pub fn str_to_chars(s: &str) -> *const c_char {
  CString::new(s.as_bytes()).unwrap().as_ptr()
}

pub fn chars_to_str<'a>(chars: *const c_char) -> &'a str {
  let slice = unsafe { CStr::from_ptr(chars) }.to_bytes();
  str::from_utf8(slice).unwrap()
}