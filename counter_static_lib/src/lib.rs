extern crate libc;

use crate::errors::errors::LibError;

use libc::{c_void};
use std::ffi::CString;
use std::env;

pub mod errors;

pub trait ICounter {
    fn get_value(&self) -> Result<i32, LibError>;
    fn increment(&mut self);
}

type Version = unsafe extern "C" fn() -> String;
type CounterFactory = unsafe extern "C" fn(elem: i32) -> *mut dyn ICounter;

pub struct Counter {
    handle: Result<*mut c_void, LibError>,
    version: Result<Version, LibError>,
    counter: Result<Box<dyn ICounter>, LibError>
}

impl Counter {
    pub fn new(elem: i32) -> Self {

        let lib_name: CString;

        let dll_filename: &str = "libcounter_dynamic_lib.so";

        if let Some(path) = env::var("DLL_PATH").ok() {
            lib_name = CString::new(path.to_owned() + "/" + dll_filename).expect("CString creation failed");
        } else {
            println!("Failed to load DLL Path from Environment Variables");
            return Self {
                handle: Err(LibError::PathDLLError),
                version: Err(LibError::PathDLLError),
                counter: Err(LibError::PathDLLError) 
            };
        }

        let lib_handle = unsafe {
            libc::dlopen(lib_name.as_ptr(), libc::RTLD_LAZY)
        };

        if lib_handle.is_null() {
            println!("Failed to load the shared library: {:?}", lib_name);
            return Self {
                handle: Err(LibError::LoadDLLError),
                version: Err(LibError::LoadDLLError),
                counter: Err(LibError::LoadDLLError) 
            };
        }

        let version_func_name = CString::new("version").expect("CString creation failed");

        let version_func_ptr = unsafe {
            libc::dlsym(lib_handle, version_func_name.as_ptr())
        };

        if version_func_ptr.is_null() {
            println!("Failed to find the function: {:?}", version_func_name);
            return Self {
                handle: Ok(lib_handle),
                version: Err(LibError::DLLSymbolNotFoundError),
                counter: Err(LibError::DLLSymbolNotFoundError) 
            };
        }

        let version_function: Version = unsafe {
            std::mem::transmute(version_func_ptr)
        };

        let factory_func_name = CString::new("factory").expect("CString creation failed");

        let factory_func_ptr = unsafe {
            libc::dlsym(lib_handle, factory_func_name.as_ptr())
        };

        if factory_func_ptr.is_null() {
            println!("Failed to find the struct factory: {:?}", factory_func_name);
            return Self {
                handle: Ok(lib_handle),
                version: Ok(version_function),
                counter: Err(LibError::DLLSymbolNotFoundError) 
            };
        }

        let counter_factory: CounterFactory = unsafe {
            std::mem::transmute(factory_func_ptr)
        };


        let counter_obj = unsafe {
            counter_factory(elem)
        };

        return Self {
            handle: Ok(lib_handle),
            version: Ok(version_function),
            counter: Ok(unsafe{Box::from_raw(counter_obj)})
        };

    }

    pub fn version(&self) -> Result<String, LibError> {
        match self.version {
            Ok(call) => {
                unsafe {
                    return Ok(call());
                }
            },
            Err(err) => {
                return Err(err);
            }
        }
    }
}

impl ICounter for Counter {
    fn get_value(&self) -> Result<i32,LibError> {
        match &self.counter {
            Ok(c) => {
                return c.get_value();
            },
            Err(err) => {
                return Err(*err);
            }
        }
    }

    fn increment(&mut self) {
        match &mut self.counter {
            Ok(c) => {
                c.increment();
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}

impl Drop for Counter {
    fn drop(&mut self) {

        self.counter = Err(LibError::GenericError);
        // Unload the shared library
        match self.handle {
            Ok(h) => {

                unsafe {
                    libc::dlclose(h);
                }
            },
            Err(_err) => {
                println!("DLL has not been opened succesfull.\nDropping Test...");
            }
        }
    }
}