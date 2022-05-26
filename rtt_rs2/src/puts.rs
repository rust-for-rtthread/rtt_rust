use core::cmp::min;

fn up_cast(a: usize, b: usize) -> usize {
    let r = a / b;
    return if a % b == 0 { r } else { r + 1 };
}

pub(crate) fn puts(str: &str, kp: fn(s: *const u8)) {
    let str = str.as_bytes();
    let mut buf = [0 as u8; 129];
    for i in 0..up_cast(str.len(), 128) {
        let end = min(128, str.len() - i * 128);
        for j in 0..end {
            buf[j] = str[(j + i * 128) as usize];
        }
        buf[end] = 0;
        kp(buf.as_ptr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::cmp::Ordering;
    use std::sync::Mutex;
    lazy_static! {
        static ref OUT: Mutex<String> = Mutex::new(String::new());
    }

    #[test]
    fn test_puts() {
        fn kputs(s: *const u8) {
            unsafe {
                use std::ffi::CStr;
                let a = CStr::from_ptr(s as _);
                let mut out = OUT.lock().unwrap();
                out.push_str(a.to_str().unwrap())
            }
        }

        {
            let data = r"
                This is a very long string.
                This is a very long string.
                This is a very long string.
                This is a very long string.
                This is a very long string.
                This is a very long string.
            ";
            assert!(data.len() > 128);
            puts(data, kputs);
            let mut out = OUT.lock().unwrap();
            print!("{}", out);
            assert_eq!(data.cmp(out.as_str()), Ordering::Equal);
            out.clear();
        }
        {
            let data = "hello";
            puts(data, kputs);
            let mut out = OUT.lock().unwrap();
            print!("{}", out);
            assert_eq!(data.cmp(out.as_str()), Ordering::Equal);
            out.clear();
        }
        {
            let data = "k";
            puts(data, kputs);
            let mut out = OUT.lock().unwrap();
            print!("{}", out);
            assert_eq!(data.cmp(out.as_str()), Ordering::Equal);
            out.clear();
        }
    }
}
