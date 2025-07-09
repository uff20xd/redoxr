#![no_std]
#![no_main]

use core::{
    fmt::{
        Write,
    },
    result::Result,
    panic::PanicInfo,
    ffi::c_char,
    ffi::c_void,
    ffi::c_int,
};

//macro_rules! print {
//    ($string:tt, $($arg:tt)*) => {{
//        unsafe { printf(b$string, ($($arg)*)) };
//    }};
//}

//#[link(name = "c")]
unsafe extern "C" {
    //fn printf(fmt: *const c_char, ...) -> c_int;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
fn _start() {
    //unsafe { printf(b"hehheeh".as_ptr() as *const c_char) };
}
