use crate::api::RttCResult;
use crate::bind::*;
use core::ptr;
use cstr_core::*;
use cty::c_void;

pub type APIRawCDevive = rt_device_t;

#[derive(Copy, Clone)]
pub struct APIRawCDevOflag(u16);

#[derive(Copy, Clone)]
pub struct APIConstCDevOflag(u16);

pub const REMOVABLE: APIConstCDevOflag = APIConstCDevOflag(RT_DEVICE_FLAG_REMOVABLE as u16);
pub const STANDALONE: APIConstCDevOflag = APIConstCDevOflag(RT_DEVICE_FLAG_STANDALONE as u16);
pub const SUSPENDED: APIConstCDevOflag = APIConstCDevOflag(RT_DEVICE_FLAG_SUSPENDED as u16);
pub const STREAM: APIConstCDevOflag = APIConstCDevOflag(RT_DEVICE_FLAG_STREAM as u16);
pub const INT_RX: APIConstCDevOflag = APIConstCDevOflag(RT_DEVICE_FLAG_INT_RX as u16);
pub const DMA_RX: APIConstCDevOflag = APIConstCDevOflag(RT_DEVICE_FLAG_DMA_RX as u16);
pub const INT_TX: APIConstCDevOflag = APIConstCDevOflag(RT_DEVICE_FLAG_INT_TX as u16);
pub const DMA_TX: APIConstCDevOflag = APIConstCDevOflag(RT_DEVICE_FLAG_DMA_TX as u16);

impl APIRawCDevOflag {
    pub fn zero() -> Self {
        APIRawCDevOflag(0)
    }

    fn readonly(&mut self) -> &mut Self {
        self.0 &= !(RT_DEVICE_FLAG_RDWR as u16);
        self.0 |= RT_DEVICE_FLAG_RDONLY as u16;
        self
    }

    fn writeonly(&mut self) -> &mut Self {
        self.0 &= !(RT_DEVICE_FLAG_RDWR as u16);
        self.0 |= RT_DEVICE_FLAG_WRONLY as u16;
        self
    }

    fn readwrite(&mut self) -> &mut Self {
        self.0 |= RT_DEVICE_FLAG_RDWR as u16;
        self
    }

    fn flag(&mut self, flag: APIConstCDevOflag, set: bool) -> &mut Self {
        if set {
            self.0 |= flag.0;
        } else {
            self.0 &= !flag.0
        }
        self
    }
}

#[inline]
pub fn cdev_find(name: &str) -> Option<APIRawCDevive> {
    let s = CString::new(name).unwrap();
    let raw;
    unsafe {
        raw = rt_device_find(s.as_ptr());
    }
    if raw == ptr::null_mut() {
        None
    } else {
        Some(raw)
    }
}

#[inline]
pub fn cdev_open(handle: APIRawCDevive, oflag: APIRawCDevOflag) -> RttCResult {
    unsafe { rt_device_open(handle, oflag.0).into() }
}

#[inline]
pub fn cdev_close(handle: APIRawCDevive) -> RttCResult {
    unsafe { rt_device_close(handle).into() }
}

#[inline]
pub fn cdev_read(handle: APIRawCDevive, pos: isize, buf: &mut [u8]) -> usize {
    unsafe {
        rt_device_read(
            handle,
            pos as _,
            buf.as_mut_ptr() as *mut cty::c_void,
            buf.len() as _,
        ) as usize
    }
}

#[inline]
pub fn cdev_write(handle: APIRawCDevive, pos: isize, buf: &mut [u8]) -> usize {
    unsafe {
        rt_device_write(
            handle,
            pos as _,
            buf.as_ptr() as *const cty::c_void,
            buf.len() as _,
        ) as usize
    }
}

#[inline]
pub unsafe fn cdev_control(handle: APIRawCDevive, cmd: i32, arg: *mut c_void) -> RttCResult {
    rt_device_control(handle, cmd, arg).into()
}
