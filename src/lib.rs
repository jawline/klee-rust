extern crate libc;

use std::os::raw;
use std::ffi::CString;
use std::mem::transmute;

#[link(name = "rust")]
extern {
  fn klee_make_symbolic(data: *const raw::c_void, length: raw::size_t, name: *const raw::c_char);
}

pub fn any(data: *const raw::c_void, length: usize, name: &str) {
  let name_cstr = CString::new(name).unwrap();
  unsafe {
    klee_make_symbolic(data, length as raw::size_t, name_cstr.as_ptr());
  }
}

pub fn i32(data: *const i32, name: &str) {
  let name_cstr = CString::new(name).unwrap();
  unsafe {
    klee_make_symbolic(transmute(data), 4, name_cstr.as_ptr());
  }
}

pub fn u32(data: *const u32, name: &str) {
  let name_cstr = CString::new(name).unwrap();
  unsafe {
    klee_make_symbolic(transmute(data), 4, name_cstr.as_ptr());
  }
}
