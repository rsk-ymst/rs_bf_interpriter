use std::{fmt::{Error, format}, ops::Index, io::{self, stdin, BufRead}};

// use crate::tokenizer;


#[derive(Debug, Clone, Copy)]
pub enum Token {
    INC, // + Increment
    DEC, // - Decrement
    GTS, // > Grater-Than Sign
    LTS, // < Less-Than Sign
    BOL, // [ Begin Of a Loop
    EOL, // ] End of a Loop
    DOT, // . DOT
    CMM, // , Comma
    None
}pub struct MemManager {
    mem: Vec<u8>,
    head: usize,
    tokens: Vec<Token>,
    tokens_cursor: usize,
    stdin_buf: Vec<u8>,
}

pub fn tokenize(input: Vec<u8>) -> Vec<Token> {
    let x: Vec<char> = input.iter().map(|x| {
        match ascii_to_char(*x) {
            Ok(x) => x,
            Err(e) => '_',
        }
    }).collect();

    x.iter().map(|x| {
        match *x {
            '+' => Token::INC,
            '-' => Token::DEC,
            '>' => Token::GTS,
            '<' => Token::LTS,
            '[' => Token::BOL,
            ']' => Token::EOL,
            '.' => Token::DOT,
            ',' => Token::CMM,
            _ => Token::None,
        }
    }).collect()
}

pub fn tokenize_v2(input: Vec<u8>) -> Vec<Token> {
    input.iter().map(|&c| match c {
        b'+' => Token::INC,
        b'-' => Token::DEC,
        b'>' => Token::GTS,
        b'<' => Token::LTS,
        b'[' => Token::BOL,
        b']' => Token::EOL,
        b'.' => Token::DOT,
        b',' => Token::CMM,
        _ => Token::None,
    }).collect()
}

impl MemManager {
    pub fn init(input: Vec<u8>) -> MemManager {
        // print!("init");
        let tokens = tokenize_v2(input);
        println!("{:?}", tokens);

        MemManager {
            mem: vec![0; 1000],
            head: 0,
            tokens: tokens,
            tokens_cursor: 0,
            stdin_buf: Vec::new(),
        }
    }

    fn inc (&mut self) {
        // self.mem[self.head] += 1;
        self.mem[self.head] = (self.mem[self.head] + 1) % 255;
        Self::print_current_status(&self, '+');
    }

    fn dec (&mut self) {
        if self.mem[self.head] <= 0 {
            panic!("dec error");
        }

        self.mem[self.head] = (self.mem[self.head] - 1) % 255;
        Self::print_current_status(&self, '-');
    }

    fn move_right (&mut self) {
        // println!("current: {} {}", self.head, self.mem[self.head]);

        // print!(">");
        self.head += 1;
        Self::print_current_status(&self, '>');
    }

    fn move_left (&mut self) {
        // println!("current: {} {}", self.head, self.mem[self.head]);
        if self.head == 0 {
            return;
        }
        // print!("<");
        self.head -= 1;
        Self::print_current_status(&self, '<');
    }

    fn stdin (&mut self) {
        // print!("input: ");
        if self.stdin_buf.len() != 0 {
            self.mem[self.head] = self.stdin_buf.remove(0);
            return;
        }

        let mut c = String::new();

        io::stdin()
            .read_line(&mut c)
            .expect("Failed to read line");

        self.stdin_buf = c.into_bytes();

        self.mem[self.head] = self.stdin_buf.remove(0);

        Self::print_current_status(&self, ',');
    }

    fn show_head_char(&self) {
        // Self::print_current_status(&self, '.');
        print!("{}", self.mem[self.head] as char);
        Self::print_current_status(&self, '.');
    }

    fn is_current_cell_alive(&self) -> bool {
        self.mem[self.head] > 0
    }

    fn print_current_status(&self, _token: char) {
        let token = self.tokens[self.tokens_cursor];
        let c = match token {
            Token::INC => '+',
            Token::DEC => '-',
            Token::GTS => '>',
            Token::LTS => '<',
            Token::DOT => '.',
            Token::CMM => ',',
            Token::BOL => '[',
            Token::EOL => ']',
            Token::None => '_'
        };

        // println!("{} | {:?} head: {} cell: {} ", self.tokens_cursor, c, self.head, self.mem[self.head]);
    }

    pub fn run(&mut self) -> Result<String, String> {
        while self.tokens_cursor < self.tokens.len() {
            // let target = Self::ascii_to_char(self.tokens[self.tokens_cursor])?;

            match self.tokens[self.tokens_cursor] {
                Token::INC => self.inc(),
                Token::DEC => self.dec(),
                Token::GTS => self.move_right(),
                Token::LTS => self.move_left(),
                Token::DOT => self.show_head_char(),
                Token::CMM => self.stdin(),
                Token::BOL => {
                    // println!("-----");
                    if !self.is_current_cell_alive() {
                        let mut count = 1;

                        while count > 0 {
                            self.tokens_cursor += 1;

                            match self.tokens[self.tokens_cursor] {
                                Token::BOL => count += 1,
                                Token::EOL => count -= 1,
                                _ => (),
                            }
                        }
                    }

                    self.print_current_status('[');
                },
                Token::EOL => {
                    if self.is_current_cell_alive() {
                        let mut count = 1;

                        while count > 0 {
                            self.tokens_cursor -= 1;

                            match self.tokens[self.tokens_cursor] {
                                Token::BOL => count -= 1,
                                Token::EOL => count += 1,
                                _ => (),
                            }
                        }
                    }

                    self.print_current_status(']');
                },
                _ => {},
            };

            self.tokens_cursor += 1;
        };

        Ok(format!("Ok"))
    }

    fn ascii_to_char(x: u8) -> Result<char, String> {
        if !x.is_ascii() {
            return Err(format!("ERR01: {x} is not ascii code").to_string());
        }

        Ok(x as char)
    }

    fn jump_forward(tokens: &[Token], cursor: usize) -> Result<usize, String> {
        let mut level = 0;

        for i in 0..tokens.len() {
            // let target = Self::ascii_to_char(tokens[i])?;

            match tokens[i] {
                Token::EOL => {
                    if level == 0 {
                        println!("jump!!!!!");
                        return Ok(i+cursor+1);
                    }
                    level -= 1;
                },
                Token::BOL => {
                    level += 1;
                },
                _ => {}
            }
        }

        panic!("Unmatched ']'");
    }

    fn jump_backward(tokens: &[Token]) -> Result<usize, String> {
        let mut level = 0;

        for i in (0..tokens.len()).rev() {
            // let target = Self::ascii_to_char(tokens[i])?;
            // println!("{:?}", tokens[i]);

            match tokens[i] {

            Token::EOL => {
                // println!("{:?} ----", tokens[i]);
                level += 1;
            },
            Token::BOL => {
                // println!("{:?} ----", tokens[i]);

                if level == 0 {
                    return Ok(i);
                }
                level -= 1;
                },
                _ => {}
            }
        }

        panic!("Unmatched '['");
    }

    fn char_to_ascii(x: char) -> Result<u8, String> {
        if !x.is_ascii() {
            return Err(format!("ERR02: {x} is not ascii code").to_string());
        }

        Ok(x as u8)
    }
}



pub struct Interpreter;

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
        let mut current_start_loop_index = 0;
        let mut loop_stack: Vec<usize> = Vec::new();

        while cursor < tokens.len() {
            let target = Self::ascii_to_char(tokens[cursor])?;

            print!("{cursor} | {target} ");
            // println!("{target}");

            match target {
                '+' => memory_manager.inc(),
                '-' => memory_manager.dec(),
                '>' => memory_manager.move_right(),
                '<' => memory_manager.move_left(),
                '.' => memory_manager.show_head_char(),
                ',' => memory_manager.stdin(),
                '[' => {

                    // let (_start_loop_index, _end_loop_index) = Self::search_pair_elements_v2(cursor, &tokens[cursor+1..tokens.len()])?;

                    // if !loop_stack.contains(&start_loop_index) {
                    //     // loop_stack.push(start_loop_index);
                    //     cursor = Self::jump_backward(&tokens[cursor..], cursor);
                    // }
                    // println!("push {:?}", loop_stack);

                    if !memory_manager.is_current_cell_alive() {
                        cursor = Self::jump_forward(&tokens[cursor..], cursor)?;
                    }
                    // else {
                    //     current_start_loop_index = start_loop_index;
                    // }

                    memory_manager.print_current_status('[');
                },
                ']' => {
                    if memory_manager.is_current_cell_alive() {
                        // スタック末尾のインデックスを参照し、ループの先頭に戻る
                        cursor = Self::jump_backward(&tokens[..cursor])?;
                        // if let Some(target_start_index) = loop_stack.get(loop_stack.len() - 1) {
                        //     cursor = *target_start_index;
                        // }
                    } else {
                        // ループを抜ける。対象のidxは削除
                        loop_stack.pop();
                        // println!("pop {:?}", loop_stack);
                    }

                    memory_manager.print_current_status(']');
                    // println!("jump back -------------");
                },
                _ => {}
            }

            cursor += 1;
        }

        Ok(format!("Ok"))
    }

    pub fn search_pair_elements_v2(start: usize, slice: &[u8]) -> Result<(usize, usize), String> {
        let mut end = 0;
        let mut cursor: usize = 0;

        let mut stack: Vec<bool> = Vec::new();
        // println!("stack: {}", stack.len());

        while cursor < slice.len() {
            // if cursor >= slice.len() {
            //     return Ok(format!("ok"))
            // }

            let target = Self::ascii_to_char(slice[cursor])?;

            // println!("target: {target}");

            match target {
                '[' => {
                    stack.push(true);
                }
                ']' => {
                    if stack.len() == 0 {
                        // println!("stack is emp");
                        end = cursor;
                        break;
                    }

                    stack.pop();
                }
                _ => ()
            }

            cursor += 1;
        }

        if end == 0 {
            // println!("xxxxxxxxxxxx!!");
            return Err(format!("error"));
        }

        // println!("st: {start}, en: {}", end+start+1);
        Ok((start, end+start+1)) // endはゼロオリジンだから+1する必要がある。
    }

    fn ascii_to_char(x: u8) -> Result<char, String> {
        if !x.is_ascii() {
            return Err(format!("ERR01: {x} is not ascii code").to_string());
        }

        Ok(x as char)
    }

    fn jump_forward(tokens: &[u8], cursor: usize) -> Result<usize, String> {
        let mut level = 0;

        for i in 0..tokens.len() {
            let target = Self::ascii_to_char(tokens[i])?;

            match target {
                '[' => {
                    level += 1;
                },
                ']' => {
                    if level == 0 {
                        return Ok(i+cursor+1);
                    }
                    level -= 1;
                },
                _ => {}
            }
        }

        panic!("Unmatched ']'");
    }

    fn jump_backward(tokens: &[u8]) -> Result<usize, String> {
        let mut level = 0;

        for i in (0..tokens.len()).rev() {
            let target = Self::ascii_to_char(tokens[i])?;

            match target {
                '[' => {
                    if level == 0 {
                        return Ok(i);
                    }
                    level -= 1;
                },
                ']' => {
                    level += 1;
                },
                _ => {}
            }
        }

        panic!("Unmatched '['");
    }

    fn char_to_ascii(x: char) -> Result<u8, String> {
        if !x.is_ascii() {
            return Err(format!("ERR02: {x} is not ascii code").to_string());
        }

        Ok(x as u8)
    }
}

fn ascii_to_char(x: u8) -> Result<char, String> {
    if !x.is_ascii() {
        return Err(format!("ERR01: {x} is not ascii code").to_string());
    }

    Ok(x as char)
}
