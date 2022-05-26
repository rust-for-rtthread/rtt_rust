use crate::api::*;
use core::time::Duration;

pub fn sleep(time: Duration) {
    let mut time = time.as_millis();
    const MAX_DELAY: u128 = i32::MAX as u128;
    const MAX_DELAY_P1: u128 = i32::MAX as u128 + 1;
    loop {
        match time {
            1..=MAX_DELAY => {
                let _ = thread_m_delay(time as i32);
                return;
            }
            0 => return,
            MAX_DELAY_P1..=u128::MAX => {
                let _ = thread_m_delay(i32::MAX);
                time -= i32::MAX as u128;
            }
        }
    }
}
