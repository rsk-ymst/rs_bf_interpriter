use std::{env, fs::File, io::Read};

mod interpreter;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    let mut contents_ascii_vec: Vec<u8> = Vec::new();

    let mut file = File::open(filename).expect("file not found...");
    file.read_to_end(&mut contents_ascii_vec).expect("file error");

    // println!("size: {:?}", contents_ascii_vec.len());

    let mut interpreter = interpreter::Interpreter::init(contents_ascii_vec);
    interpreter.run();
}
