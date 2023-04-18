use std::{fmt::Error, ops::Index, io};


pub struct MemManager {
    mem: Vec<u8>,
    head: usize,
}

impl MemManager {
    pub fn init(cap: usize) -> MemManager {
        MemManager {
            mem: vec![0; cap],
            head: 0,
        }
    }

    pub fn inc (&mut self) {
        self.mem[self.head] += 1;
    }

    pub fn dec (&mut self) {
        self.mem[self.head] -= 1;
    }

    pub fn move_right (&mut self) {
        self.head += 1;
    }

    pub fn move_left (&mut self) {
        self.head -= 1;
    }

    pub fn show_head_char(&self) {
        println!("{}", self.mem[self.head] as char);
    }
}

struct Interpreter {

}

impl Interpreter {
    pub fn consume(tokens: &mut Vec<u8>, memory_manager: &mut MemManager) {
        tokens.iter().enumerate().for_each(|(index, x)| {
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
                '[' => {
                    let end_loop_index = match Self::search_pair_elements(&tokens[index..tokens.len()]) {
                        Ok(end) => end,
                        Err(e) => return (),
                    };

                    Interpreter::consume(&mut tokens[index..end_loop_index].to_vec(), memory_manager);
                },
                // ']' => ,
            }
        });
    }

    pub fn consume_v2(tokens: &mut Vec<u8>, memory_manager: &mut MemManager) -> Result<String, String> {
        let mut cursor: usize = 0;

        loop {
            let target = match Self::ascii_to_char(tokens[cursor]) {
                Ok(c) => c,
                Err(e) => return Err(e),
            };

            match target {
                '+' => memory_manager.inc(),
                '-' => memory_manager.dec(),
                '>' => memory_manager.move_right(),
                '<' => memory_manager.move_left(),
                '.' => memory_manager.show_head_char(),
                '[' => {
                    let end_loop_index = match Self::search_pair_elements(&tokens[cursor..tokens.len()]) {
                        Ok(end) => end,
                        Err(e) => return Err(e),
                    };

                    loop {
                        if tokens[cursor] <= 0 {
                            break;
                        }

                        Interpreter::consume(&mut tokens[cursor..end_loop_index].to_vec(), memory_manager);
                    }
                },
            }

            cursor += 1;
        }

    }


    pub fn search_pair_elements(slice: &[u8]) -> Result<usize, String> {
        let mut ret = 0;

        slice.into_iter().enumerate().for_each(|(index, x)| {
            let mut target = match Self::ascii_to_char(*x) {
                Ok(char_str) => char_str,
                Err(e) => return (),
            };

            match target {
                ']' => {
                    ret = index;
                    return;
                }
            }
        });

        if ret == 0 {
            return Ok(0);
        }

        Ok(ret)
    }

    fn ascii_to_char(x: u8) -> Result<char, String> {
        if !x.is_ascii() {
            return Err(format!("ERR01: {x} is not ascii code").to_string());
        }

        Ok(x as char)
    }

    fn char_to_ascii(x: char) -> Result<u8, String> {
        if !x.is_ascii() {
            return Err(format!("ERR02: {x} is not ascii code").to_string());
        }

        Ok(x as u8)
    }
}
