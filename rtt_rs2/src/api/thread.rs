use super::base::*;
use crate::bind::*;
use core::ptr;
use cstr_core::*;
use cty::*;

// Thread handle, defined by c code
pub type APIRawThread = rt_thread_t;

// Thread entry function, Defined by rtt c code
pub type ThreadEntry = extern "C" fn(parameter: *mut c_void);

// Create a new thread
// Return None:OOM Some():thread handle
#[inline]
pub fn thread_create(
    name: &str,
    entry: ThreadEntry,
    param: *mut c_void,
    stack_size: u32,
    priority: u8,
    tick: u32,
) -> Option<APIRawThread> {
    let name = CString::new(name).unwrap();
    let raw;
    unsafe {
        raw = rt_thread_create(
            name.as_ptr(),
            Some(entry),
            param,
            stack_size,
            priority,
            tick,
        );
    }
    if raw == ptr::null_mut() {
        None
    } else {
        Some(raw)
    }
}

// Delete a thread from system
#[inline]
pub fn thread_delete(th: APIRawThread) -> RttCResult {
    unsafe { rt_thread_delete(th).into() }
}

// Get current thread
#[inline]
pub fn thread_self() -> Option<APIRawThread> {
    let ret;
    unsafe {
        ret = rt_thread_self();
    }
    if ret == ptr::null_mut() {
        None
    } else {
        Some(ret)
    }
}

// Find thread by name
#[inline]
pub fn thread_find(name: &str) -> Option<APIRawThread> {
    let name = CString::new(name).unwrap();
    let ret;
    unsafe { ret = rt_thread_find(name.as_ptr() as _) }
    if ret == ptr::null_mut() {
        None
    } else {
        Some(ret)
    }
}

// Startup a thread
#[inline]
pub fn thread_startup(th: APIRawThread) -> RttCResult {
    unsafe { rt_thread_startup(th).into() }
}

// Thread have a sleep
#[inline]
pub fn thread_delay(ticks: usize) -> RttCResult {
    unsafe { rt_thread_delay(ticks as _).into() }
}

// Thread have a ms sleep
#[inline]
pub fn thread_m_delay(ms: i32) -> RttCResult {
    unsafe { rt_thread_mdelay(ms as _).into() }
}

// Control thread
// TODO
#[inline]
pub fn thread_control() {}

// Yield
#[inline]
pub fn thread_yield() {
    unsafe {
        rt_thread_yield();
    }
}

// Suspend a thread
#[inline]
pub fn thread_suspend(th: APIRawThread) -> RttCResult {
    unsafe { rt_thread_suspend(th).into() }
}

// Resume a thread
#[inline]
pub fn thread_resume(th: APIRawThread) -> RttCResult {
    unsafe { rt_thread_resume(th).into() }
}

// Run Scheduler
#[inline]
pub fn schedule() {
    unsafe {
        rt_schedule();
    }
}

// Critical
#[inline]
pub fn enter_critical() {
    unsafe {
        rt_enter_critical();
    }
}

// Critical
#[inline]
pub fn exit_critical() {
    unsafe {
        rt_exit_critical();
    }
}

#[inline]
pub fn critical_context<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let out;
    enter_critical();
    out = f();
    exit_critical();
    out
}
