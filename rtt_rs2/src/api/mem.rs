//! For malloc.rs
use crate::bind::*;
use cty::*;

// Alloc memory
pub fn mem_alloc(bytes: usize) -> *mut c_void {
    unsafe { rt_malloc(bytes as _) }
}
// Free memory
pub fn mem_free(ptr: *mut c_void) {
    unsafe {
        rt_free(ptr);
    }
}
