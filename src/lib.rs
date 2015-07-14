extern crate libc;

use std::ffi::CString;
use std::mem::transmute;

#[link(name = "rust")]
extern {
  fn klee_make_symbolic(data: *const libc::c_void, length: libc::size_t, name: *const libc::c_char);
}

pub fn any(data: *const libc::c_void, length: usize, name: &str) {
  let name_cstr = CString::new(name).unwrap();
  unsafe {
    klee_make_symbolic(data, length, name_cstr.as_ptr());
  }
}

pub fn i32(data: *const i32, length: usize, name: &str) {
  let name_cstr = CString::new(name).unwrap();
  unsafe {
    klee_make_symbolic(transmute(data), length, name_cstr.as_ptr());
  }
}

pub fn u32(data: *const u32, length: usize, name: &str) {
  let name_cstr = CString::new(name).unwrap();
  unsafe {
    klee_make_symbolic(transmute(data), length, name_cstr.as_ptr());
  }
}
