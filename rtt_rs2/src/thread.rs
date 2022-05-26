//! Thread support using rt-thread API
//!
//! # Example
//! ```
//! use rtt_rs2::thread::Thread;
//! for i in 0..2 {
//!    let th = Thread::new().name("th").stack_size(4096).start(move ||{
//! 		for _ in 0..10
//!			{
//! 			print!("hello thread");
//! 			Thread::delay(100);
//!  	    }
//!    });
//! }
//! ```
//!

use crate::alloc::boxed::Box;
use crate::api::*;
use crate::{RTResult, RTTError};
use alloc::string::String;
use core::mem;

#[derive(Debug)]
pub struct Thread(APIRawThread);

impl Thread {
    /// Delay some ticks: ticks
    /// The clock cycle will depend on the configuration of the system
    pub fn delay(tick: usize) {
        let _ = thread_delay(tick);
    }

    /// Delay some millisecond
    pub fn ms_delay(ms: usize) {
        let _ = thread_m_delay(ms as i32);
    }

    pub fn new() -> ThreadBuilder {
        ThreadBuilder {
            th_name: "uname".into(),
            th_stack_size: 4096,
            th_priority: 10,
            th_ticks: 10,
        }
    }

    pub fn r#yield() {
        thread_yield();
    }

    /// # Note
    /// The system has the function of automatically reclaiming the end thread.
    /// If a thread is very short, it is likely to end before you do delete.
    /// At this time, the handle is invalid.
    /// If you try to delete it, an assertion will be generated.
    /// So make sure that the thread you want to delete is not finished.
    /// That's why the drop function is not implemented to delete threads.
    pub fn delete_thread(th: Self) {
        let _ = thread_delete(th.0);
    }

    /// # Note
    /// Please read the `Note` of `fn delete_thread`
    pub fn delete(&self) {
        let _ = thread_delete(self.0);
    }

    unsafe fn spawn_inner(
        name: String,
        stack_size: u32,
        priority: u8,
        ticks: u32,
        func: Box<dyn FnOnce()>,
    ) -> Result<Self, RTTError> {
        let func = Box::new(func);
        let param = &*func as *const _ as *mut _;

        extern "C" fn thread_func(param: *mut c_void) {
            unsafe {
                let run = Box::from_raw(param as *mut Box<dyn FnOnce()>);
                run();
            }
        }

        let th_handle = thread_create(
            name.as_ref(),
            thread_func,
            param,
            stack_size,
            priority,
            ticks,
        )
        .ok_or(RTTError::OutOfMemory)?;

        let ret = match Self::_startup(th_handle) {
            Ok(_) => {
                mem::forget(func);
                Ok(Thread(th_handle))
            }
            Err(e) => Err(e),
        };

        return ret;
    }

    fn _startup(th: APIRawThread) -> Result<(), RTTError> {
        let ret = thread_startup(th);
        return if is_eok(ret) {
            Ok(())
        } else {
            Err(RTTError::ThreadStartupErr)
        };
    }

    pub fn spawn<F>(
        name: String,
        stack_size: u32,
        priority: u8,
        ticks: u32,
        func: F,
    ) -> RTResult<Thread>
    where
        F: FnOnce() -> () + Send + 'static,
    {
        unsafe { Self::spawn_inner(name, stack_size, priority, ticks, Box::new(func)) }
    }
}

unsafe impl Send for Thread {}

pub struct ThreadBuilder {
    th_name: String,
    th_stack_size: u32,
    th_priority: u8,
    th_ticks: u32,
}

impl ThreadBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.th_name = name.into();
        self
    }

    pub fn stack_size(&mut self, stack_size: u32) -> &mut Self {
        self.th_stack_size = stack_size;
        self
    }

    pub fn priority(&mut self, priority: u8) -> &mut Self {
        self.th_priority = priority;
        self
    }

    pub fn ticks(&mut self, ticks: u32) -> &mut Self {
        self.th_ticks = ticks;
        self
    }

    pub fn start<F>(&self, func: F) -> RTResult<Thread>
    where
        F: FnOnce() -> (),
        F: Send + 'static,
    {
        Thread::spawn(
            self.th_name.clone(),
            self.th_stack_size,
            self.th_priority,
            self.th_ticks,
            func,
        )
    }
}
