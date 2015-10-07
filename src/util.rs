use std::ffi::{CString, CStr};
use std::str;
use libc::{c_char, c_int};

use err::HdfsErr;
use native::*;
use dfs::HdfsFS;

pub fn str_to_chars(s: &str) -> *const c_char {
  CString::new(s.as_bytes()).unwrap().as_ptr()
}

pub fn chars_to_str<'a>(chars: *const c_char) -> &'a str {
  let slice = unsafe { CStr::from_ptr(chars) }.to_bytes();
  str::from_utf8(slice).unwrap()
}

pub fn bool_to_c_int(val: bool) -> c_int {
  if val { 1 } else { 0 }
}

pub struct HdfsUtil;

/// HDFS Utility
impl HdfsUtil {

  /// Copy file from one filesystem to another.
  ///
  /// #### Params
  /// * ```srcFS``` - The handle to source filesystem.
  /// * ```src``` - The path of source file.
  /// * ```dstFS``` - The handle to destination filesystem.
  /// * ```dst``` - The path of destination file.
  pub fn copy(src_fs: &HdfsFS, src: &str, dst_fs: &HdfsFS, dst: &str)
      -> Result<bool, HdfsErr> {

    let res = unsafe {
      hdfsCopy(src_fs.raw, str_to_chars(src), dst_fs.raw, str_to_chars(dst))
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Move file from one filesystem to another.
  ///
  /// #### Params
  /// * ```srcFS``` - The handle to source filesystem.
  /// * ```src``` - The path of source file.
  /// * ```dstFS``` - The handle to destination filesystem.
  /// * ```dst``` - The path of destination file.
  pub fn mv(src_fs: &HdfsFS, src: &str, dst_fs: &HdfsFS, dst: &str)
      -> Result<bool, HdfsErr> {

    let res = unsafe {
      hdfsMove(src_fs.raw, str_to_chars(src), dst_fs.raw, str_to_chars(dst))
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }
}