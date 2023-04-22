use std::{env, fs::File, io::Read};

// mod tokenizer;
mod interpreter;
mod consumer;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename: &str = &args[1];
    // println!("filename: {filename}");

    let mut file = File::open(filename).expect("file not found...");

    // let mut contents_string: String = String::new();
    let mut contents_ascii_vec: Vec<u8> = Vec::new();

    file.read_to_end(&mut contents_ascii_vec).expect("something went wrong reading the file");

    // println!("With text: \n{:?}", contents_string);
    println!("size: {:?}", contents_ascii_vec.len());

    let LENGTH: usize = contents_ascii_vec.len();
    let mut memory: Vec<u8> = vec![8; LENGTH].to_vec();

    let mut interpreter = interpreter::MemManager::init(contents_ascii_vec);
    interpreter.run();
}
