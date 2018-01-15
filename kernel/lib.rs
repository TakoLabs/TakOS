#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod drivers;

use drivers::vga;
use drivers::pic;

#[no_mangle]
pub extern fn kmain(multiboot_info_addr: usize) {
	// ATTENTION: we have a very small stack and no guard page

	vga::clear_screen();
	info!("TakOS starting...");
	println!("TakOS starting...");

	info!("Getting multiboot informations.");
	let boot_info = unsafe{ multiboot2::load(multiboot_info_addr) };
	let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
	let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");
	let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
	let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();
	let multiboot_start = multiboot_info_addr;
	let multiboot_end = multiboot_start + (boot_info.total_size as usize);

	info!("Memory areas:");
	for area in memory_map_tag.memory_areas() {
	    info!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
	}


	info!("Kernel sections:");
	for section in elf_sections_tag.sections() {
	    info!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
	        section.addr, section.size, section.flags);
	}

	info!("Kernel start: 0x{:x}, Kernel end: 0x{:x}", kernel_start, kernel_end);
	info!("Multiboot start: 0x{:x}, Multiboot end: 0x{:x}", multiboot_start, multiboot_end);


	println!("Initializing interrupt...");
	info!("Initializing interrupt...");
	info!("initialize pics...");
	pic::initialize();

	loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    error!("\n\nPANIC in {} at line {}:", file, line);
    error!("    {}", fmt);
    loop{}
}
