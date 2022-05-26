//! Passing information between threads

use crate::api::*;
use crate::{panic_on_atomic_context, RTTError};
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::mem::{size_of, MaybeUninit};

const RT_WAITING_FOREVER: isize = -1;

unsafe impl<T> Send for Queue<T> where T: Send {}
unsafe impl<T> Sync for Queue<T> where T: Send {}

#[derive(Debug)]
pub struct Queue<T> {
    queue: APIRawQueue,
    /* only for store item type */
    item_type: PhantomData<*const UnsafeCell<[T; 8]>>,
}

impl<T> Queue<T> {
    pub fn new(max_size: usize) -> Result<Queue<T>, RTTError> {
        queue_create("Unnamed", max_size as _, size_of::<T>() as _)
            .ok_or(RTTError::OutOfMemory)
            .map(|m| Queue {
                queue: m,
                item_type: PhantomData,
            })
    }

    pub fn new_with_name(name: &str, max_size: usize) -> Result<Queue<T>, RTTError> {
        queue_create(name, max_size as _, size_of::<T>() as _)
            .ok_or(RTTError::OutOfMemory)
            .map(|m| Queue {
                queue: m,
                item_type: PhantomData,
            })
    }

    pub fn try_send(&self, item: T) -> Result<(), (RTTError, T)> {
        self._send(item, 0)
    }

    pub fn send(&self, item: T, max_wait: i32) -> Result<(), (RTTError, T)> {
        panic_on_atomic_context("queue send");
        self._send(item, max_wait)
    }

    pub fn send_wait_forever(&self, item: T) -> Result<(), (RTTError, T)> {
        panic_on_atomic_context("queue send wait forever");
        self._send(item, RT_WAITING_FOREVER as _)
    }

    fn _send(&self, item: T, max_wait: i32) -> Result<(), (RTTError, T)> {
        let inner = MaybeUninit::new(item);
        let ret = queue_send_wait(
            self.queue,
            inner.as_ptr() as *const c_void,
            size_of::<T>() as _,
            max_wait,
        );
        return if !is_eok(ret) {
            unsafe { Err((RTTError::QueueSendTimeout, inner.assume_init())) }
        } else {
            Ok(())
        };
    }

    pub fn try_recv(&self) -> Result<T, RTTError> {
        self._receive(0)
    }

    pub fn recv(&self, max_wait: i32) -> Result<T, RTTError> {
        panic_on_atomic_context("queue recv");
        self._receive(max_wait)
    }

    pub fn recv_wait_forever(&self) -> Result<T, RTTError> {
        panic_on_atomic_context("queue recv wait forever");
        self._receive(RT_WAITING_FOREVER as _)
    }

    fn _receive(&self, max_wait: i32) -> Result<T, RTTError> {
        let mut inner = MaybeUninit::<T>::uninit();
        let ret = queue_receive_wait(
            self.queue,
            inner.as_mut_ptr() as *mut c_void,
            size_of::<T>() as _,
            max_wait,
        );
        return if is_eok(ret) {
            Ok(unsafe { inner.assume_init() })
        } else {
            Err(RTTError::QueueReceiveTimeout)
        };
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        queue_delete(self.queue);
    }
}
