extern crate libc;

use std::os::raw;
use std::ffi::CString;
use std::mem::{transmute, size_of};

#[link(name = "klee")]
extern {
  fn klee_make_symbolic(data: *const raw::c_void, length: libc::size_t, name: *const raw::c_char);
  fn klee_int(name: *const raw::c_char) -> raw::c_int;
  fn klee_warning(name: *const raw::c_char);
}

pub unsafe fn any(data: *const raw::c_void, length: usize, name: &str) {
  klee_make_symbolic(data, length as libc::size_t, CString::new(name).unwrap().as_ptr());
}

pub fn warning(name: &str) {
  unsafe { klee_warning(CString::new(name).unwrap().as_ptr()); }
}

pub fn i32(name: &str) -> i32 {
  let result;
  unsafe {
    result = klee_int(CString::new(name).unwrap().as_ptr());
  }
  result
}

pub fn sym<T>(data: &T, name: &str) {
  unsafe{ any(transmute(data as *const T), size_of::<T>(), name); }
}
