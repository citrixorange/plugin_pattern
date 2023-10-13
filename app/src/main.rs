extern crate counter_static_lib;

use counter_static_lib::{Counter, ICounter};

fn main() {
    let mut test_instance = Counter::new(0);
    let version = test_instance.version();
    println!("DLL Version: {}", version.expect("Failed to retrieve DLL version"));
    println!("Stack current value: {}", test_instance.get_value().expect("Failed to instanciate Stack"));
    test_instance.increment();
    println!("Stack current value: {}", test_instance.get_value().expect("Failed to instanciate Stack"));
}
