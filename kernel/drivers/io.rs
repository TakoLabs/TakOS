use core::marker::{PhantomData};


unsafe fn inb(port: u16) -> u8 {
	let result: u8;

	asm!("inb %dx, %al" : 
		 "={al}"(result) : 
		 "{dx}"(port) :: 
		 "volatile");

	result
}

unsafe fn outb(value: u8, port: u16) { 
	asm!("outb %al, %dx" :: 
		 "{dx}"(port), "{al}"(value) :: 
		 "volatile");
}

unsafe fn inw(port: u16) -> u16 {
	let result: u16;

	asm!("inw %dx, %ax" : 
		 "={ax}"(result) : 
		 "{dx}"(port) :: 
		 "volatile");

	result
}

unsafe fn outw(value: u16, port: u16) {
	asm!("outw %ax, %dx" :: 
		 "{dx}"(port), "{ax}"(value) :: 
		 "volatile");
}

unsafe fn inl(port: u16) -> u32 {
	let result: u32;

	asm!("inl %dx, %eax" : 
		 "={eax}"(result) : 
		 "{dx}"(port) :: 
		 "volatile");

	result
}

unsafe fn outl(value: u32, port: u16) {
	asm!("outl %eax, %dx" :: 
		 "{dx}"(port), "{eax}"(value) :: 
		 "volatile");	
}



pub trait IO {
	unsafe fn port_in(port: u16) -> Self;
	unsafe fn port_out(port: u16, value: Self);
}

impl IO for u8 { 
	unsafe fn port_in(port: u16) -> u8 { inb(port) }
	unsafe fn port_out(port: u16, value: u8) { outb(value, port); }
}

impl IO for u16 {
	unsafe fn port_in(port: u16) -> u16 { inw(port) }
	unsafe fn port_out(port: u16, value: u16) { outw(value, port); }
}

impl IO for u32 { 
	unsafe fn port_in(port: u16) -> u32 { inl(port) }
	unsafe fn port_out(port: u16, value: u32) { outl(value, port); }
}


pub struct Port<T: IO> {
	port: u16,
	phantom: PhantomData<T>,
}

impl<T: IO> Port<T> {
	pub const unsafe fn new(port: u16) -> Port<T> {
		Port { port: port, phantom: PhantomData }
	}

	pub fn read(&mut self) -> T {
		unsafe { T::port_in(self.port) }
	}

	pub fn write(&mut self, value: T) {
		unsafe { T::port_out(self.port, value); }
	}
}


pub struct UnsafePort<T: IO> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: IO> UnsafePort<T> {
    pub const unsafe fn new(port: u16) -> UnsafePort<T> {
        UnsafePort { port: port, phantom: PhantomData }
    }

    pub unsafe fn read(&mut self) -> T {
        T::port_in(self.port)
    }

    pub unsafe fn write(&mut self, value: T) {
        T::port_out(self.port, value);
    }
}