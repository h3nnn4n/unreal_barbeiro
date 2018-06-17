use std::fs::File;
use std::io::Read;

// Memory map info, from: http://gbdev.gg8.se/wiki/articles/Memory_Map

//0000    3FFF    16KB ROM bank 00        From cartridge, fixed bank
//4000    7FFF    16KB ROM Bank 01~NN     From cartridge, switchable bank via MBC (if any)
//8000    9FFF    8KB Video RAM (VRAM)    Only bank 0 in Non-CGB mode
//                                        Switchable bank 0/1 in CGB mode

//A000    BFFF    8KB External RAM        In cartridge, switchable bank if any
//C000    CFFF    4KB Work RAM (WRAM)     bank 0
//D000    DFFF    4KB Work RAM (WRAM)     bank 1~N    Only bank 1 in Non-CGB mode
//                                        Switchable bank 1~7 in CGB mode

//E000    FDFF    Mirror of C000~DDFF (ECHO RAM)    Typically not used
//FE00    FE9F    Sprite attribute table (OAM)
//FEA0    FEFF    Not Usable
//FF00    FF7F    I/O Registers
//FF80    FFFE    High RAM (HRAM)
//FFFF    FFFF    Interrupts Enable Register (IE)

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn init() -> Memory {
        Memory { data: Vec::new() }
    }

    pub fn load_rom(&mut self, file_name: String) {
        let mut file = File::open(file_name).unwrap();

        let bytes_read = file.read_to_end(&mut self.data).unwrap();
        println!("Loaded {} bytes", bytes_read);
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000...0x3FFF => (),
            0x4000...0x7FFF => (),
            0x8000...0x9FFF => (),
            0xA000...0xBFFF => self.data[addr as usize] = data,
            0xC000...0xCFFF => self.data[addr as usize] = data,
            0xD000...0xDFFF => self.data[addr as usize] = data,
            _ => (),
        }
    }
}
