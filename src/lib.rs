//! # nostd-printf
//!
//! Rust crate containing an embedded version of printf which can be used in
//! `no_std` projects which aren't linked to `libc`.
#![cfg_attr(not(test), no_std)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Unit tests
#[cfg(test)]
mod tests {
    use crate::{printf, set_putchar};
    use {core::ffi::c_char, once_cell::sync::Lazy, std::sync::Mutex};

    static OUTPUT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

    #[unsafe(no_mangle)]
    unsafe extern "C" fn putchar(c: c_char) {
        let c = c as u8 as char;
        OUTPUT.lock().unwrap().push(c);
    }

    fn setup() {
        OUTPUT.lock().unwrap().clear();
        unsafe {
            set_putchar(Some(putchar));
        }
    }

    fn expect(expected: &str) {
        assert_eq!(OUTPUT.lock().unwrap().as_str(), expected);
    }

    #[test]
    fn test_printf() {
        setup();
        unsafe {
            printf(c"%s-%d\n".as_ptr(), c"test".as_ptr(), 123);
            expect("test-123\n");
        }
    }
}
