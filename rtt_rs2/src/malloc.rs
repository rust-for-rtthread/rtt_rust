//! The basic memory alloc/free function uses rt-thread API

use crate::api::*;
use crate::panic_on_atomic_context;
use core::alloc::{GlobalAlloc, Layout};

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    panic!("OOM!");
}

pub struct RttAlloc;

unsafe impl GlobalAlloc for RttAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        panic_on_atomic_context("malloc");
        mem_alloc(layout.size() as usize) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        panic_on_atomic_context("dealloc");
        mem_free(ptr as *mut c_void)
    }
}
