use drivers::io::{UnsafePort};

struct Pic {
    offset: u8,
    command: UnsafePort<u8>,
    data: UnsafePort<u8>
}

impl Pic {
	unsafe end_of_interrupt(&mut self) {
		self.command.write();
	}
}