use crate::api::RttCResult;
use crate::bind::*;
use core::ptr;
use cstr_core::*;

pub type APIRawMutex = rt_mutex_t;

#[inline]
pub fn mutex_create(name: &str) -> Option<APIRawMutex> {
    let s = CString::new(name).unwrap();
    let raw;
    unsafe {
        raw = rt_mutex_create(s.as_ptr(), 1);
    }
    if raw == ptr::null_mut() {
        None
    } else {
        Some(raw)
    }
}

#[inline]
pub fn mutex_delete(handle: APIRawMutex) {
    unsafe {
        rt_mutex_delete(handle);
    }
}

#[inline]
pub fn mutex_take(handle: APIRawMutex, tick: isize) -> RttCResult {
    unsafe { rt_mutex_take(handle, tick as _).into() }
}

#[inline]
pub fn mutex_release(handle: APIRawMutex) {
    unsafe {
        rt_mutex_release(handle);
    }
}
