struct Token<'a> {
    next: &'a Token<'a>,
    val: TokenType
}

pub enum TokenType {
    INC, // + Increment
    DEC, // - Decrement
    LTS, // < Less-Than Sign
    GTS, // > Grater-Than Sign
    BOL, // [ Begin Of a Loop
    EOL, // ] End of a Loop
    DOT, // . DOT
    CMM, // , Comma
}
