// TMP for DEBUGGING
use core::fmt;
use spin::Mutex;
use volatile::Volatile;
use super::{Port, Io};


const PORT_COM1: u16 = 0x3F8;


pub static LOGGER: Mutex<Serial> = Mutex::new(Serial::new());


pub struct Serial {
	isInit: bool
}

impl Serial {
	pub const fn new() -> Serial {
		Serial {
			isInit: false
		}
	}

	fn initialize(&mut self) {
		let mut com1_0: Port<u8> = unsafe { Port::new(PORT_COM1 + 0) };
		let mut com1_1: Port<u8> = unsafe { Port::new(PORT_COM1 + 1) };
		let mut com1_2: Port<u8> = unsafe { Port::new(PORT_COM1 + 2) };
		let mut com1_3: Port<u8> = unsafe { Port::new(PORT_COM1 + 3) };
		let mut com1_4: Port<u8> = unsafe { Port::new(PORT_COM1 + 4) };

		com1_1.write(0x00);
		com1_3.write(0x80);
		com1_0.write(0x03);
		com1_1.write(0x00);
		com1_3.write(0x03);
		com1_2.write(0xC7);
		com1_4.write(0x08);

		self.isInit = true;
	}

	pub fn write_byte(&mut self, c: u8) {
		if !self.isInit { self.initialize(); }
		let mut com1: Port<u8> = unsafe { Port::new(PORT_COM1) };
		com1.write(c);
	}

	pub fn write_str(&mut self, s: &str) {
		for c in s.bytes() {
            self.write_byte(c);
        }
	}
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
        	self.write_byte(byte)
        }
        Ok(())
    }
}

pub fn log(args: fmt::Arguments) {
    use core::fmt::Write;
    LOGGER.lock().write_fmt(args).unwrap();
}

macro_rules! log {
    ($($arg:tt)*) => ({
        io::serial::log(format_args!($($arg)*));
    });
}

macro_rules! info {
    ($fmt:expr) => (log!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (log!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! info {
    ($fmt:expr) => (log!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (log!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! error {
    ($fmt:expr) => (log!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (log!(concat!($fmt, "\n"), $($arg)*));
}
