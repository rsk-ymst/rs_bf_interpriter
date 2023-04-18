use std::{env, fs::File, io::Read};

mod tokenizer;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename: &str = &args[1];
    println!("filename: {filename}");

    let mut file = File::open(filename).expect("file not found...");

    let mut contents_string: String = String::new();
    let mut contents_adcii_vec: Vec<u8> = Vec::new();
    // file.read_to_string(&mut contents_string).expect("something went wrong reading the file");
    file.read_to_end(&mut contents_adcii_vec).expect("something went wrong reading the file");

    // println!("With text: \n{}", contents_string);
    println!("With text: \n{:?}", contents_adcii_vec);

    contents_adcii_vec.iter().for_each(|x| {
        if !x.is_ascii() {
            return;
        }

        let target_char = *x as char;

        match target_char {
            '+' => (),
            '-' => (),
            '<' => (),
            '>' => (),
            '.' => (),
            ',' => (),
            _ => (),
        }
    });
}
