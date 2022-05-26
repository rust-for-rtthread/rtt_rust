//! Dump defines from bindings.rs
//! That file from build.rs creating

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use cty as c_types;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
