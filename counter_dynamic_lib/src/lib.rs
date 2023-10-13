#![allow(improper_ctypes_definitions)]

extern crate counter_static_lib;

use counter_static_lib::{ICounter, errors::errors::LibError};

#[repr(C)]
pub struct Counter {
    elem: i32
}

impl Counter {
    pub fn new(elem:i32) -> Self {
        Self {
            elem
        }
    }
}

impl ICounter for Counter {
    fn get_value(&self) -> Result<i32,LibError> {
        return Ok(self.elem);
    }

    fn increment(&mut self) {
        self.elem = self.elem + 1;
    }
}

#[no_mangle]
pub extern "C" fn version() -> String {
    let version: String = String::from(env!("CARGO_PKG_VERSION"));
    return version;
}

#[no_mangle]
pub extern "C" fn factory(elem: i32) -> Box<dyn ICounter> {
    return Box::new(Counter::new(elem));
}