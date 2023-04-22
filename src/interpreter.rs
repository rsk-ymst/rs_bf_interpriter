use std::{
    io::{self},
};

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
    None,
}

pub struct Interpreter {
    mem: Vec<u8>,
    head: usize,
    tokens: Vec<Token>,
    tokens_cursor: usize,
    stdin_buf: Vec<u8>,
}

pub fn tokenize(input: Vec<u8>) -> Vec<Token> {
    input
        .iter()
        .map(|&c| match c {
            b'+' => Token::INC,
            b'-' => Token::DEC,
            b'>' => Token::GTS,
            b'<' => Token::LTS,
            b'[' => Token::BOL,
            b']' => Token::EOL,
            b'.' => Token::DOT,
            b',' => Token::CMM,
            _ => Token::None,
        })
        .collect()
}

impl Interpreter {
    pub fn init(input: Vec<u8>) -> Interpreter {
        let tokens = tokenize(input);
        // println!("{:?}", tokens);

        Interpreter {
            mem: vec![0; 1000],
            head: 0,
            tokens: tokens,
            tokens_cursor: 0,
            stdin_buf: Vec::new(),
        }
    }

    fn inc(&mut self) {
        self.mem[self.head] = (self.mem[self.head] + 1) % 255;
        Self::print_current_status(&self, '+');
    }

    fn dec(&mut self) {
        if self.mem[self.head] <= 0 {
            panic!("dec error");
        }

        self.mem[self.head] = (self.mem[self.head] - 1) % 255;
        Self::print_current_status(&self, '-');
    }

    fn move_right(&mut self) {
        self.head += 1;
        Self::print_current_status(&self, '>');
    }

    fn move_left(&mut self) {
        if self.head == 0 {
            return;
        }

        self.head -= 1;
        Self::print_current_status(&self, '<');
    }

    fn stdin(&mut self) {
        if self.stdin_buf.len() != 0 {
            self.mem[self.head] = self.stdin_buf.remove(0);
            return;
        }

        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Failed to read line");

        self.stdin_buf = c.into_bytes();
        self.mem[self.head] = self.stdin_buf.remove(0);

        Self::print_current_status(&self, ',');
    }

    fn show_head_char(&self) {
        print!("{}", self.mem[self.head] as char);
        Self::print_current_status(&self, '.');
    }

    fn is_current_cell_alive(&self) -> bool {
        self.mem[self.head] > 0
    }

    /* デバッグ用メソッド */
    fn print_current_status(&self, _token: char) {
        let token = self.tokens[self.tokens_cursor];
        let _c = match token {
            Token::INC => '+',
            Token::DEC => '-',
            Token::GTS => '>',
            Token::LTS => '<',
            Token::DOT => '.',
            Token::CMM => ',',
            Token::BOL => '[',
            Token::EOL => ']',
            Token::None => '_',
        };

        // デバッグ時は以下の行をコメントアウトする。
        // println!("{} | {:?} head: {} cell: {} ", self.tokens_cursor, c, self.head, self.mem[self.head]);
    }

    pub fn run(&mut self) {
        while self.tokens_cursor < self.tokens.len() {

            match self.tokens[self.tokens_cursor] {
                Token::INC => self.inc(),
                Token::DEC => self.dec(),
                Token::GTS => self.move_right(),
                Token::LTS => self.move_left(),
                Token::DOT => self.show_head_char(),
                Token::CMM => self.stdin(),
                Token::BOL => {
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
                _ => {}
            };

            self.tokens_cursor += 1;
        }
    }
}
