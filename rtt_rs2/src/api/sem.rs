use crate::api::RttCResult;
use crate::bind::*;
use core::ptr;
use cstr_core::*;

pub type APIRawSem = rt_sem_t;

#[inline]
pub(crate) fn semaphore_create(name: &str) -> Option<APIRawSem> {
    let s = CString::new(name).unwrap();
    let raw;
    unsafe {
        raw = rt_sem_create(s.as_ptr(), 0, 0);
    }
    return if raw == ptr::null_mut() {
        None
    } else {
        Some(raw)
    };
}

#[inline]
pub(crate) fn semaphore_try_take(handle: APIRawSem) -> RttCResult {
    unsafe { rt_sem_trytake(handle).into() }
}

#[inline]
pub(crate) fn semaphore_take(handle: APIRawSem, tick: i32) -> RttCResult {
    unsafe { rt_sem_take(handle, tick).into() }
}

#[inline]
pub(crate) fn semaphore_release(handle: APIRawSem) -> RttCResult {
    unsafe { rt_sem_release(handle).into() }
}

#[inline]
pub(crate) fn semaphore_delete(handle: APIRawSem) {
    unsafe {
        let _ = rt_sem_delete(handle);
    }
}
