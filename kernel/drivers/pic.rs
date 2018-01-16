use drivers::io::{Port, UnsafePort};
use spin::Mutex;

static PICS: Mutex<CascadedPics> = Mutex::new(unsafe { CascadedPics::new(0x20, 0x28) });

struct Pic {
    offset: u8,
    command: UnsafePort<u8>,
    data: UnsafePort<u8>
}

impl Pic {
	unsafe fn new(offset: u8, command_port: u16, data_port: u16) -> Pic {
        Pic {
            offset: offset,
            command: UnsafePort::new(command_port),
            data: UnsafePort::new(data_port)
        }
    }
}

struct CascadedPics {
    pics: [Pic; 2]
}

impl CascadedPics {
    const unsafe fn new(offset_master: u8, offset_slave: u8) -> CascadedPics {
        CascadedPics {
            pics: [
                Pic::new(offset_master, 0x20, 0x21),
                Pic::new(offset_slave, 0xA0, 0xA1)
            ]
        }
    }

    unsafe fn initialize(&mut self) {


        // Writing to these ports create a small delay.
        // We need todo like these because we don't have access to any timer.
        let mut wait_port: Port<u8> = Port::new(0x80);
        let mut wait = || { wait_port.write(0) };

        // Save our original interrupt masks, because I'm too lazy to
        // figure out reasonable values.  We'll restore these when we're
        // done.
        let saved_mask1 = self.pics[0].data.read();
        let saved_mask2 = self.pics[1].data.read();

        // Initialization of ICW1
        // |0|0|0|1|c|0|b|a|
        // With ICW4 a = 1  otherwise a = 0
        // Only one controler b = 1, if cascading b = 0
        // Triggered by edge c = 0, otherwise triggered by level c = 1
        let  icw1 = 0b0001_0001;
        self.pics[0].command.write(icw1);
        wait();
        self.pics[1].command.write(icw1);
        wait();

        // Initialization of ICW2
        // Define the offsets of the interrupt vector
        self.pics[0].data.write(self.pics[0].offset);
        wait();
        self.pics[1].data.write(self.pics[1].offset);
        wait();

        // Initialization of ICW3
        // Discribe how the two pics are connected
        // master: |x|x|x|x|x|x|x|x|  1 on the bit where the connection is.
        // slave: |0|0|0|0|0|x|x|x| the number of the connection.
        self.pics[0].data.write(0x04);
        wait();
        self.pics[1].data.write(0x02);
        wait();

        // Initialization of ICW4
        // |0|0|0|d|c|b|a|1| Define how the controler work
        // Automatic EOI, a = 1
        // TODO: Define others modes
        let icw4 = 0b0000_0001;
        self.pics[0].data.write(icw4);
        wait();
        self.pics[1].data.write(icw4);
        wait();

        // Restore our saved masks.
        self.pics[0].data.write(saved_mask1);
        self.pics[1].data.write(saved_mask2);
    }
}

pub unsafe fn initialize() {
    PICS.lock().initialize();
}
