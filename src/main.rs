use std::{env, fs::File, io::Read};

mod tokenizer;
mod mem_manager;
mod consumer;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename: &str = &args[1];
    println!("filename: {filename}");

    let mut file = File::open(filename).expect("file not found...");

    let mut contents_string: String = String::new();
    let mut contents_ascii_vec: Vec<u8> = Vec::new();
    // file.read_to_string(&mut contents_string).expect("something went wrong reading the file");
    file.read_to_end(&mut contents_ascii_vec).expect("something went wrong reading the file");

    println!("With text: \n{:?}", contents_ascii_vec);

    let LENGTH: usize = contents_ascii_vec.len();
    let mut memory: Vec<u8> = vec![8; LENGTH].to_vec();

    let mut memory_manager = mem_manager::MemManager::init(contents_ascii_vec.len());

    println!("{:?}", memory);

    contents_ascii_vec.iter().for_each(|x| {
        if !x.is_ascii() {
            return;
        }

        let target_char = *x as char;
        match target_char {
            '+' => memory_manager.inc(),
            '-' => memory_manager.dec(),
            '>' => memory_manager.move_right(),
            '<' => memory_manager.move_left(),
            '.' => memory_manager.show_head_char(),
            ',' => memory_manager.,
            '[' => consumer::new(),
            ']' => (),
        }
    });
}
