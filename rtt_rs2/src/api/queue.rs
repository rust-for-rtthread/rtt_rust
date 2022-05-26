use crate::api::{c_void, RttCResult};
use crate::bind::*;
use core::ptr;
use cstr_core::*;

pub type APIRawQueue = rt_mq_t;

#[inline]
pub(crate) fn queue_create(name: &str, len: u32, message_size: u32) -> Option<APIRawQueue> {
    let s = CString::new(name).unwrap();
    let raw;
    unsafe { raw = rt_mq_create(s.as_ptr(), message_size, len, 0) }
    if raw == ptr::null_mut() {
        None
    } else {
        Some(raw)
    }
}

#[inline]
pub(crate) fn queue_send_wait(
    handle: APIRawQueue,
    msg: *const c_void,
    msg_size: u32,
    tick: i32,
) -> RttCResult {
    unsafe { rt_mq_send_wait(handle, msg, msg_size, tick).into() }
}

#[inline]
pub(crate) fn queue_receive_wait(
    handle: APIRawQueue,
    msg: *mut c_void,
    msg_size: u32,
    tick: i32,
) -> RttCResult {
    unsafe { rt_mq_recv(handle, msg, msg_size, tick).into() }
}

#[inline]
pub(crate) fn queue_delete(handle: APIRawQueue) {
    unsafe {
        rt_mq_delete(handle);
    }
}
