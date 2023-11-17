#![feature(asm_const)]
#![no_std]
#![no_main]
use core::str;

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;


static mut ABI_TABLE: usize = 0;

fn hello() {
    unsafe {
        core::arch::asm!("
            li      t0, {abi_num}
            slli    t0, t0, 3
            mv      a7, {abi_table}
            add     t1, a7, t0
            ld      t1, (t1)
            jalr    t1",
            abi_num = const SYS_HELLO,
            abi_table = in(reg) ABI_TABLE,
            clobber_abi("C"),
        )
    }
}

fn putchar(c: char) {
    let arg0: u8 = c as u8;
    unsafe {
        core::arch::asm!("
            li      t0, {abi_num}
            slli    t0, t0, 3
            mv      a7, {abi_table}
            add     t1, a7, t0
            ld      t1, (t1)
            jalr    t1",
            abi_num = const SYS_PUTCHAR,
            abi_table = in(reg) ABI_TABLE,
            in("a0") arg0,
            clobber_abi("C"),
        )
    }
}

fn terminate() {
    unsafe {
        core::arch::asm!("
            li      t0, {abi_num}
            slli    t0, t0, 3
            mv      a7, {abi_table}
            add     t1, a7, t0
            ld      t1, (t1)
            jalr    t1",
            abi_num = const SYS_TERMINATE,
            abi_table = in(reg) ABI_TABLE,
            clobber_abi("C"),
        )
    }
}
// fn puts(s: &str) {
//     for &byte in s.as_bytes() {
//         putchar(byte as char);
//     }
// }

fn puts(s: &str) {
    let mut i = 0;

    // 逐字符输出字符串，直到遇到 null 结尾
    while let Some(c) = s.as_bytes().get(i) {
        putchar(*c as char);
        i += 1;
    }
}


#[no_mangle]
unsafe extern "C" fn _start(abi_table:usize) -> () {
    unsafe {
        ABI_TABLE = abi_table;
    }
    let c = 'D';
    let s = "arceos";
    putchar(c);
    core::arch::asm!("
    nop",
    clobber_abi("C"),
)
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

