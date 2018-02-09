use arch;
use super::Io;
use core::marker::{PhantomData};

pub struct Port<T> {
    port: u16,
    value: PhantomData<T>
}

impl<T> Port<T> {
    pub const fn new(port: u16) -> Self {
        Port::<T> {
            port: port,
            value: PhantomData
        }
    }
}

impl Io for Port<u8> {
    type Value = u8;

    fn read(&self) -> u8 {
        unsafe { arch::io::inb(self.port) }
    }

    fn write(&self, value: u8) {
        unsafe { arch::io::outb(value, self.port); }
    }
}

impl Io for Port<u16> {
    type Value = u16;

    fn read(&self) -> Self::Value {
        unsafe { arch::io::inw(self.port) }
    }

    fn write(&self, value: Self::Value) {
        unsafe { arch::io::outw(value, self.port); }
    }
}

impl Io for Port<u32> {
    type Value = u32;

    fn read(&self) -> Self::Value {
        unsafe { arch::io::inl(self.port) }
    }

    fn write(&self, value: Self::Value) {
        unsafe { arch::io::outl(value, self.port); }
    }
}
