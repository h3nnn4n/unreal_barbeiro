mod memory;

fn main() {
    let mut ram = memory::Memory::init();
    let file_name = "Tetris.gb".to_string();
    ram.load_rom(file_name);

    for addr in 0..10 {
        println!("{:?}", ram.read_byte(addr));
    }
}
