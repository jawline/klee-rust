extern crate libc;

#[link(name = "rust")]
extern {
  fn klee_make_symbolic(data: *const libc::c_void, length: usize, name: *const libc::c_char);
}
