extern crate libc;

use std::os::raw;
use std::ffi::CString;
use std::mem::{transmute, size_of};

#[link(name = "kleeRuntest")]
extern {
    fn klee_make_symbolic(data: *const raw::c_void, length: libc::size_t, name: *const raw::c_char);
    fn klee_warning_once(name: *const raw::c_char);
    fn klee_set_forking(state: bool);
}

pub unsafe fn any(data: *const raw::c_void, length: usize, name: &str) {
    klee_make_symbolic(data, length as libc::size_t, CString::new(name).unwrap().as_ptr());
}

pub fn warning_once(name: &str) {
    unsafe { klee_warning_once(CString::new(name).unwrap().as_ptr()); }
}

pub fn set_forking(state: bool) {
    unsafe { klee_set_forking(state); }
}

pub fn i32(name: &str) -> i32 {
    let res : i32 = 0;
    symbol(&res, name);
    return res;
}

pub fn bool(name: &str) ->bool {
    let res : bool = false;
    symbol(&res, name);
    return res;
}

pub fn symbol<T>(data: &T, name: &str) {
    unsafe{ any(transmute(data as *const T), size_of::<T>(), name); }
}

pub fn assert_warn(condition: bool, msg: &str) {
    if condition {
        warning(msg);
    }
}
