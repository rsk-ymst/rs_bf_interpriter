use std::{fmt::{Error, format}, ops::Index, io};


pub struct MemManager {
    mem: Vec<u8>,
    head: usize,
}

impl MemManager {
    pub fn init() -> MemManager {
        print!("init");
        MemManager {
            mem: vec![0; 1000],
            head: 0,
        }
    }

    pub fn inc (&mut self) {
        Self::print_current_status(&self);
        // print!("+");
        // println!("current: {} {}", self.head, self.mem[self.head]);
        self.mem[self.head] += 1;
    }

    pub fn dec (&mut self) {
        Self::print_current_status(&self);
        // print!("-");
        // println!("current: {} {}", self.head, self.mem[self.head]);

        self.mem[self.head] -= 1;
    }

    pub fn move_right (&mut self) {
        Self::print_current_status(&self);
        // println!("current: {} {}", self.head, self.mem[self.head]);

        // print!(">");
        self.head += 1;
    }

    pub fn move_left (&mut self) {
        Self::print_current_status(&self);
        // println!("current: {} {}", self.head, self.mem[self.head]);

        // print!("<");
        self.head -= 1;
    }

    pub fn show_head_char(&self) {
        Self::print_current_status(&self);

        print!("{}", self.mem[self.head] as char);
    }

    pub fn is_current_cell_alive(&self) -> bool {
        self.mem[self.head] > 0
    }

    fn print_current_status(&self) {
        println!("current: {} {}\n----------------------", self.head, self.mem[self.head]);
    }
}

pub struct Interpreter {

}

impl Interpreter {
    // pub fn consume(tokens: &mut Vec<u8>, memory_manager: &mut MemManager) {
    //     tokens.iter().enumerate().for_each(|(index, x)| {
    //         if !x.is_ascii() {
    //             return;
    //         }

    //         let target_char = *x as char;
    //         match target_char {
    //             '+' => memory_manager.inc(),
    //             '-' => memory_manager.dec(),
    //             '>' => memory_manager.move_right(),
    //             '<' => memory_manager.move_left(),
    //             '.' => memory_manager.show_head_char(),
    //             '[' => {
    //                 let end_loop_index = match Self::search_pair_elements(&tokens[index..tokens.len()]) {
    //                     Ok(end) => end,
    //                     Err(e) => return (),
    //                 } + index;

    //                 loop {
    //                     // if !memory_manager.is_current_cell_alive()
    //                     // Interpreter::consume(&mut tokens[index..end_loop_index].to_vec(), memory_manager);
    //                 }
    //             },
    //             // ']' => ,
    //         }
    //     });
    // }

    pub fn consume_v2(tokens: &mut Vec<u8>, memory_manager: &mut MemManager) -> Result<String, String> {
        let mut cursor: usize = 0;

        loop {
            if cursor >= tokens.len() {
                return Ok(format!("ok"))
            }

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
                    let (start_loop_index, end_loop_index) = match Self::search_pair_elements_v2(cursor, &tokens[cursor..tokens.len()]) {
                        Ok(end) => end,
                        Err(e) => return Err(e),
                    };

                    loop {
                        if !memory_manager.is_current_cell_alive() {
                            // println!("break!!!");
                            break;
                        }

                        Interpreter::consume_v2(&mut tokens[start_loop_index+1..end_loop_index].to_vec(), memory_manager)?;
                    }

                    cursor = end_loop_index+1;
                    continue;
                },
                ']' => {println!("something is wrong"); ()},
                _ => ()
            }

            println!("cursor: {cursor}, token: {target}");
            cursor += 1;
        }

    }

    // pub fn search_pair_elements(slice: &[u8]) -> Result<(usize, usize), String> {
    //     let mut ret = 0;

    //     slice.into_iter().enumerate().for_each(|(index, x)| {
    //         let mut target = match Self::ascii_to_char(*x) {
    //             Ok(char_str) => char_str,
    //             Err(e) => return (),
    //         };

    //         match target {
    //             ']' => {
    //                 ret = index;
    //                 return;
    //             },
    //             _ => ()
    //         }
    //     });

    //     if ret == 0 {
    //         println!("xxxxxxxxxxxx!!");
    //         return Err(format!("error"));
    //     }

    //     println!("] is at {ret}");
    //     Ok(ret)
    // }


    pub fn search_pair_elements_v2(start: usize, slice: &[u8]) -> Result<(usize, usize), String> {
        let mut end = 0;
        let mut cursor: usize = 0;

        loop {
            if cursor >= slice.len() {
                return Ok(format!("ok"))
            }

            let target = match Self::ascii_to_char(slice[cursor]) {
                Ok(c) => c,
                Err(e) => return Err(e),
            };

            match target {
                '[' => {
                    // ここでflag建てる
                }
                ']' => {
                    // flag立ってたらcontinue, そうでなければflagをfalseにして以下の処理。
                    end = cursor;
                    break;
                },
                _ => ()
            }

            cursor += 1;
        };

        if end == 0 {
            println!("xxxxxxxxxxxx!!");
            return Err(format!("error"));
        }

        // println!("] is at {end}");

        Ok((start, end+start))
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
