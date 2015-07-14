extern crate libc;

use std::os::raw;
use std::ffi::CString;
use std::mem::{transmute, size_of};

#[link(name = "klee")]
extern {
  fn klee_make_symbolic(data: *const raw::c_void, length: libc::size_t, name: *const raw::c_char);
  fn klee_int(name: *const raw::c_char) -> raw::c_int;
}

pub unsafe fn any(data: *const raw::c_void, length: usize, name: &str) {
  let name_cstr = CString::new(name).unwrap();
  klee_make_symbolic(data, length as libc::size_t, name_cstr.as_ptr());
}

pub fn i32(name: &str) -> i32 {
  let result;
  let name_cstr = CString::new(name).unwrap();
  unsafe {
    result = klee_int(name_cstr.as_ptr());
  }
  result
}

pub fn make_sym<T>(data: &T, name: &str) {
  unsafe{ any(transmute(data as *const T), sizeof::<T>(), name); }
}
