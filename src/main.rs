mod memory;

fn main() {
    let mut ram = memory::Ram::init();
    let file_name = "Tetris.gb".to_string();
    ram.load_rom(file_name);
}
