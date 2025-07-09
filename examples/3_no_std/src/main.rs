#![no_std]
#![no_main]

use core::{
    fmt::{
        Write,
    },
    result::Result,
    panic::PanicInfo,
};

macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::io::_print($crate::format_args!($($arg)*));
    }};
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn _start() -> Result<(), ()> {
    Ok(())
}
