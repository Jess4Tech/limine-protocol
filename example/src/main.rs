#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

use limine_protocol::{
    requests::{BootloaderInfoRequest, MemoryMapRequest, TerminalRequest},
    Request,
};

static TERMINAL: Request<TerminalRequest> = TerminalRequest::default().into();

static MEMORY_MAP: Request<MemoryMapRequest> = MemoryMapRequest::default().into();

static BOOTLOADER_INFO: Request<BootloaderInfoRequest> = BootloaderInfoRequest::default().into();

mod writer;
use writer::{print, println};

/// This is a panic handler.
/// There are many like it, but this one is ours!
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _start() -> ! {
    writer::TerminalWriter::init();

    println!("Hello from Rust!");

    println!("Bootloader Info: {:#?}", unsafe {
        BOOTLOADER_INFO.response.unwrap().as_ref()
    });

    println!("Memory Map: {:#?}", unsafe {
        MEMORY_MAP
            .response
            .unwrap()
            .as_ref()
            .get_memory_map()
            .unwrap()
    });

    loop {}
}
