extern crate libc;

use std::os::raw;
use std::ffi::CString;
use std::mem::{transmute, size_of};
use std::default::Default;

#[link(name = "kleeRuntest")]
extern {
    fn klee_make_symbolic(data: *mut raw::c_void, length: libc::size_t, name: *const raw::c_char);
    fn klee_set_forking(state: bool);
}

pub unsafe fn any(data: *mut raw::c_void, length: usize, name: &str) {
    klee_make_symbolic(data, length as libc::size_t, CString::new(name).unwrap().as_ptr());
}

pub fn set_forking(state: bool) {
    unsafe { klee_set_forking(state); }
}

pub fn some<T: Default>(name: &str) -> T {
    let mut new_symbol = T::default();
    symbol(&mut new_symbol, name);
    return new_symbol;
}

pub fn symbol<T>(data: *mut T, name: &str) {
    unsafe{ any(transmute(data), size_of::<T>(), name); }
}
