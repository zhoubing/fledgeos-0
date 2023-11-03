#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]

use core::fmt;
use core::fmt::Write;
use core::panic::PanicInfo;
use x86_64::instructions::hlt;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    // intrinsics::abort()
    let mut cursor = Cursor {
        position: 0,
        foreground: Color::White,
        background: Color::Red,
    };
    for _ in 0..(80 * 25) {
        cursor.print(b" ")
    }
    cursor.position = 0;
    write!(cursor, "{}", _info).unwrap();

    loop {
        unsafe {
            hlt();
        }
    }
}

//VGA兼容文本模式下的彩色调色板，位模式和颜色值之间的映射关系是由VGA标准定义的
#[allow(unused)]
#[derive(Copy, Clone)]
#[repr(u8)] //指示编译器要使用一个单字节表示这些值
enum Color {
    Black = 0x0,
    White = 0xf,
    Blue = 0x1,
    BrightBlue = 0x9,
    Green = 0x2,
    BrightGreen = 0xA,
    Cyan = 0x3,
    BrightCyan = 0xB,
    Red = 0x4,
    BrightRed = 0xC,
    Magenta = 0x5,
    BrightMagenta = 0xD,
    Brown = 0x6,
    Yellow = 0xE,
    Gray = 0x7,
    DarkGray = 0x8,
}

struct Cursor {
    position: isize,
    foreground: Color,
    background: Color,
}

impl fmt::Write for Cursor {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s.as_bytes());
        Ok(())
    }
}

impl Cursor {
    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }

    fn print(&mut self, text: &[u8]) {
        let color = self.color();
        let fb = 0xb8000 as *mut u8;
        for &character in text {
            unsafe {
                fb.offset(self.position).write_volatile(character);
                fb.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // test();
    // output_text();

    panic!("test panic");

    loop {
        hlt()
    }
}

fn test() {
    let fb = 0xb81000 as *mut u8;
    unsafe {
        fb.offset(1).write_volatile(0x30)
    }
}

fn output_text() {
    let text = b"Rust in Action";
    let mut cursor = Cursor {
        position: 0,
        foreground: Color::BrightCyan,
        background: Color::Black,
    };
    cursor.print(text);
}