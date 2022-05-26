//! For Device
//! Notice: This file's function mainly be used by driver developers
//! I do not support using `c` to write driver
//! So, Here is no interface for rust to operation `c` driver
use super::base::*;
use crate::alloc::vec::Vec;
use crate::bind::*;
use crate::Box;
use core::ptr;
use cstr_core::*;
use cty::*;

// For device
// Device type : No contain all types
// If you wanted not in following
// Please add it from bind-gen file
pub enum RawDeviceType {
    CharDevice = 0,
    BlockDevice = 1,
}

// Device : raw pointer
type APIRawDevice = rt_device_t;

// Device operations table
pub struct DevInnerOps {
    init: Option<DevInit>,
    open: Option<DevOpen>,
    close: Option<DevClose>,
    read: Option<DevRead>,
    write: Option<DevWrite>,
    ctrl: Option<DevCtrl>,
}

// Will be hock to `c device` var named user_data
pub struct DevInnerDate {
    ops: DevInnerOps,
    data: *mut c_void,
}

pub fn set_user_data(dev: APIRawDevice, ptr: *mut c_void) -> *mut c_void {
    let old;
    unsafe {
        let inner = (*dev).user_data as *mut DevInnerDate;
        old = (*inner).data;
        (*inner).data = ptr;
    }
    old
}

pub fn get_user_data(dev: APIRawDevice) -> *mut c_void {
    unsafe {
        let inner = (*dev).user_data as *mut DevInnerDate;
        (*inner).data
    }
}

pub fn get_ops_table(dev: APIRawDevice) -> *mut DevInnerOps {
    unsafe {
        let inner = (*dev).user_data as *mut DevInnerDate;
        &mut (*inner).ops as _
    }
}

// Create a device struct
// This struct define from `c code`
pub fn device_create(dev_type: RawDeviceType) -> APIRawDevice {
    unsafe {
        let raw = rt_device_create(dev_type as _, 0);
        let inner = Box::new(DevInnerDate {
            ops: DevInnerOps {
                init: None,
                open: None,
                close: None,
                read: None,
                write: None,
                ctrl: None,
            },
            data: ptr::null_mut(),
        });
        let inner = Box::leak(inner);
        (*raw).user_data = inner as *mut DevInnerDate as _;
        raw
    }
}

// Destroy a device
pub fn device_destroy(dev: APIRawDevice) {
    let inner;
    unsafe {
        inner = (*dev).user_data as *mut DevInnerDate;
        let _ = Box::from_raw(inner);
    }
    unsafe { rt_device_destroy(dev) }
}

// // Bind device's user_data
// pub fn device_bind_user_data(dev: RawDevice, data: *mut c_void) {
//     unsafe {
//         let old_data = (*dev).user_data;
//         if old_data != ptr::null_mut::<c_void>() {
//             rt_free(old_data);
//         }
//         (*dev).user_data = data;
//     }
// }

// Set device's `init` function
type DevInit = fn(APIRawDevice) -> RttCResult;
pub fn device_set_ops_init(dev: APIRawDevice, f: DevInit) {
    let ops = get_ops_table(dev);
    unsafe {
        (*ops).init = Some(f);
    }
    extern "C" fn __init(dev: rt_device_t) -> rt_err_t {
        let f = unsafe { (*get_ops_table(dev)).init };
        return if let Some(f) = f {
            f(dev) as _
        } else {
            -(RT_ERROR as i32)
        };
    }
    unsafe { (*dev).init = Some(__init) }
}

// Set device's `open` function
type DevOpen = fn(APIRawDevice, flag: u16) -> RttCResult;
pub fn device_set_ops_open(dev: APIRawDevice, f: DevOpen) {
    let ops = get_ops_table(dev);
    unsafe {
        (*ops).open = Some(f);
    }
    extern "C" fn __open(dev: rt_device_t, flag: u16) -> rt_err_t {
        let f = unsafe { (*get_ops_table(dev)).open };
        return if let Some(f) = f {
            f(dev, flag) as _
        } else {
            -(RT_ERROR as i32)
        };
    }
    unsafe { (*dev).open = Some(__open) }
}

// Set device's `close` function
type DevClose = fn(APIRawDevice) -> RttCResult;
pub fn device_set_ops_close(dev: APIRawDevice, f: DevClose) {
    let ops = get_ops_table(dev);
    unsafe {
        (*ops).close = Some(f);
    }
    extern "C" fn __close(dev: rt_device_t) -> rt_err_t {
        let f = unsafe { (*get_ops_table(dev)).close };
        return if let Some(f) = f {
            f(dev) as _
        } else {
            -(RT_ERROR as i32)
        };
    }
    unsafe { (*dev).close = Some(__close) }
}

// Set device's `read` function
type DevRead = fn(APIRawDevice, pos: usize, len: usize) -> Vec<u8>;
pub fn device_set_ops_read(dev: APIRawDevice, f: DevRead) {
    let ops = get_ops_table(dev);
    unsafe {
        (*ops).read = Some(f);
    }
    extern "C" fn __read(
        dev: rt_device_t,
        pos: rt_off_t,
        buffer: *mut c_void,
        size: rt_size_t,
    ) -> rt_size_t {
        let f = unsafe { (*get_ops_table(dev)).read };
        return if let Some(f) = f {
            let ret = f(dev, pos as _, size as _);
            let r_len = ret.len();
            let mut buf = buffer as *mut u8;
            for i in ret {
                unsafe {
                    *buf = i;
                    buf = buf.offset(1);
                }
            }
            r_len as _
        } else {
            0
        };
    }
    unsafe { (*dev).read = Some(__read) }
}

// Set device's `write` function
type DevWrite = fn(APIRawDevice, pos: usize, data: Vec<u8>) -> usize;
pub fn device_set_ops_write(dev: APIRawDevice, f: DevWrite) {
    let ops = get_ops_table(dev);
    unsafe {
        (*ops).write = Some(f);
    }
    extern "C" fn __write(
        dev: rt_device_t,
        pos: rt_off_t,
        buffer: *const c_void,
        size: rt_size_t,
    ) -> rt_size_t {
        let f = unsafe { (*get_ops_table(dev)).write };
        return if let Some(f) = f {
            let r_buf = buffer as *const u8;
            let mut buf = Vec::new();
            unsafe {
                for i in 0..size {
                    buf.push(*(r_buf.offset(i as _)))
                }
            }
            f(dev, pos as _, buf) as _
        } else {
            0
        };
    }
    unsafe {
        (*dev).write = Some(__write);
    }
}

// Set device's `control` function
type DevCtrl = fn(APIRawDevice, cmd: i32, data: *mut c_void) -> RttCResult;
pub fn device_set_ops_control(dev: APIRawDevice, f: DevCtrl) {
    let ops = get_ops_table(dev);
    unsafe {
        (*ops).ctrl = Some(f);
    }
    extern "C" fn __ctrl(dev: rt_device_t, cmd: c_int, args: *mut c_void) -> rt_err_t {
        let f = unsafe { (*get_ops_table(dev)).ctrl };
        return if let Some(f) = f {
            f(dev, cmd, args) as _
        } else {
            0
        };
    }
    unsafe {
        (*dev).control = Some(__ctrl);
    }
}

// Get the needed function
type DevRxIndicateCB = unsafe extern "C" fn(dev: APIRawDevice, size: u32) -> i32;
pub fn device_get_rx_indicate_func(dev: APIRawDevice) -> Option<DevRxIndicateCB> {
    unsafe { (*dev).rx_indicate }
}

// Get the needed function
type DevTxCompleteCB = unsafe extern "C" fn(dev: APIRawDevice, buffer: *mut c_void) -> i32;
pub fn device_get_tx_complete_func(dev: APIRawDevice) -> Option<DevTxCompleteCB> {
    unsafe { (*dev).tx_complete }
}

// Call RX indicate function
pub fn device_call_rx_indicate(dev: APIRawDevice, size: u32) -> i32 {
    let func = device_get_rx_indicate_func(dev);
    if let Some(raw_f) = func {
        unsafe {
            return raw_f(dev, size);
        }
    }
    return 0;
}

// Call TX complete function
pub fn device_call_tx_complete(dev: APIRawDevice) -> i32 {
    let func = device_get_tx_complete_func(dev);
    if let Some(raw_f) = func {
        unsafe {
            return raw_f(dev, ptr::null_mut());
        }
    }
    return 0;
}

// Register device to `c framework`
pub fn device_register(dev: APIRawDevice, name: &str, flag: u16) -> RttCResult {
    let c_name = CString::new(name).unwrap();
    unsafe { rt_device_register(dev, c_name.as_ptr(), flag).into() }
}

// UnRegister device from `c framework`
pub fn device_unregister(dev: APIRawDevice) -> RttCResult {
    unsafe { rt_device_unregister(dev).into() }
}
