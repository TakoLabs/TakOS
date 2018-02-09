mod port;
#[macro_use]
pub mod serial;

pub use self::port::Port;

pub trait Io {
    type Value;

    fn read(&self) -> Self::Value;
    fn write(&self, value: Self::Value);
}
