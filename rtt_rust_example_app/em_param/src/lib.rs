#![no_std]

extern crate alloc;

use alloc::string::String;
use rtt_main::rtt_main;
use rtt_rs2::param::Param;
use rtt_rs2::println;

#[rtt_main(appname = "rust_em_param",
            cmd = true,
            run = true,
            desc = "Rust example app.")]
fn main(param: Param) {
    for i in param {
        println!("{}", String::from_utf8_lossy(&*i))
    }
}
