use std::fs::File;
use std::io::Read;

pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn init() -> Ram {
        Ram { data: Vec::new() }
    }

    pub fn load_rom(&mut self, file_name: String) {
        let mut file = File::open(file_name).unwrap();

        let bytes_read = file.read_to_end(&mut self.data).unwrap();
        println!("Loaded {} bytes", bytes_read);
    }
}
