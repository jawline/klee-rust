extern crate libc;

use std::ffi::CString;

#[link(name = "rust")]
extern {
  fn klee_make_symbolic(data: *const libc::c_void, length: usize, name: *const libc::c_char);
}

pub fn klee_make_symbolic_any(data: *const libc::c_void, length: usize, name: &str) {
  let name_cstr = CString::new(name).unwrap();
  unsafe {
    klee_make_symbolic(data, length, name_cstr.as_ptr());
  }
}
