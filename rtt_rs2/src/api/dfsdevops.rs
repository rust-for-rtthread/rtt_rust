use crate::api::RttCResult;
use crate::bind::*;
use alloc::format;
use cstr_core::*;

pub type APIRawDFSDevive = i32;

#[derive(Clone, Copy)]
pub struct APIRawDFSDevOFlag(i32);

impl APIRawDFSDevOFlag {
    pub fn zero() -> Self {
        APIRawDFSDevOFlag(0)
    }
    fn readonly(&mut self) -> &mut Self {
        self.0 &= !(O_RDWR as i32);
        self.0 |= O_RDONLY as i32;
        self
    }

    fn writeonly(&mut self) -> &mut Self {
        self.0 &= !(O_RDWR as i32);
        self.0 |= O_WRONLY as i32;
        self
    }

    fn readwrite(&mut self) -> &mut Self {
        self.0 |= O_RDWR as i32;
        self
    }
}

#[inline]
pub fn dfsdev_open(name: &str, oflag: APIRawDFSDevOFlag) -> Option<APIRawDFSDevive> {
    let name = CString::new(format!("/dev/{}", name)).unwrap();
    let raw = unsafe { open(name.as_ptr(), oflag.0) };
    if raw < 0 {
        None
    } else {
        Some(raw)
    }
}

#[inline]
pub fn dfsdev_read(handle: APIRawDFSDevive, buf: &mut [u8]) -> usize {
    unsafe { read(handle, buf.as_mut_ptr() as *mut cty::c_void, buf.len() as _) as usize }
}

#[inline]
pub fn dfsdev_write(handle: APIRawDFSDevive, buf: &[u8]) -> usize {
    unsafe { write(handle, buf.as_ptr() as *mut cty::c_void, buf.len() as _) as usize }
}

#[inline]
pub fn dfsdev_poll_in(handle: APIRawDFSDevive) -> RttCResult {
    let mut pfd = pollfd {
        fd: handle,
        events: POLL_IN as _,
        revents: 0,
    };
    let ret = unsafe {
        poll(&mut pfd as *mut pollfd, 1, -1)
    };
    return if ret == 0 {
        if pfd.revents & (POLL_IN as i16) != 0 {
            0.into()
        } else {
            RttCResult::Error
        }
    } else {
        ret.into()
    }
}

#[inline]
pub fn dfsdev_poll_out(_handle: APIRawDFSDevive) -> RttCResult {
    unimplemented!()
}

#[inline]
pub fn dfsdev_close(handle: APIRawDFSDevive) -> RttCResult {
    unsafe { close(handle).into() }
}
