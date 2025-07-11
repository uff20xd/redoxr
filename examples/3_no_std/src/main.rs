#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn exit(status: i32) -> ! {
    unsafe {

        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") status,
            options(noreturn)
        );
    }
}

fn sys_write(fd: i32, buf: *const u8, count: usize) -> usize {
    unsafe {
        let ret: usize;

        asm!(
            "syscall",
            in("rax") 1,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rax") ret,
            out("rcx") _,
            out("r11") _,
        );

        ret
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let msg = b"Hello, world!\n";
    let _ = sys_write(1, msg.as_ptr(), msg.len());

    exit(69)
}
