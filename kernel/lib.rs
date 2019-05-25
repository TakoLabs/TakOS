#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
extern crate lazy_static;

pub mod arch;
pub use arch::*;

#[macro_use]
pub mod io;
pub use io::*;

#[macro_use]
pub mod drivers;
pub use drivers::*;

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn kmain(multiboot_info_addr: usize) {
	vga::clear_screen();

	println!("TakOS starting...");
	info!("TakOS starting...");

	println!("Getting multiboot informations.");
	info!("Getting multiboot informations.");
	let boot_info = unsafe{ multiboot2::load(multiboot_info_addr) };
	let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
	let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");
	let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
	let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();
	let multiboot_start = multiboot_info_addr;
	let multiboot_end = multiboot_start + (boot_info.total_size as usize);

	println!("Memory areas:");
	info!("Memory areas:");
	for area in memory_map_tag.memory_areas() {
	    println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
	    info!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
	}


	println!("Kernel sections:");
	info!("Kernel sections:");
	for section in elf_sections_tag.sections() {
	    println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}", section.addr, section.size, section.flags);
	    info!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}", section.addr, section.size, section.flags);
	}

	println!("Kernel start: 0x{:x}, Kernel end: 0x{:x}", kernel_start, kernel_end);
	println!("Multiboot start: 0x{:x}, Multiboot end: 0x{:x}", multiboot_start, multiboot_end);
	info!("Kernel start: 0x{:x}, Kernel end: 0x{:x}", kernel_start, kernel_end);
	info!("Multiboot start: 0x{:x}, Multiboot end: 0x{:x}", multiboot_start, multiboot_end);


	println!("Initializing interrupt...");
	info!("Initializing interrupt...");
	println!(" initialize pics...");
	info!(" initialize pics...");
	//unsafe { pic::initialize(); }

	loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
