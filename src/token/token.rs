#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL = 0,
    EOF = 1,

    // Identifiers and Literals
    IDENT = 2,
    INT = 3,

    // Operators
    PLUS = 4,
    ASSIGN = 5,

    // Delimiters
    COMMA = 6,
    SEMICOLON = 7,

    LPAREN = 8,
    RRAREN = 9,
    LBRACE = 10,
    RBRACE = 11,

    // Keywords
    FUNCTION = 12,
    LET = 13,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: Vec<char>,
}

impl Token {
    pub fn new(ttype: TokenType, literal: Vec<char>) -> Token {
        Token {
            ttype: ttype,
            literal: literal,
        }
    }

    pub fn lookup_ident(ident: &Vec<char>) -> TokenType {
        // Exact matching for now
        if ident.len() == 3 && ident[0] == 'l' && ident[1] == 'e' && ident[2] == 't'{
            TokenType::LET
        }else if ident.len() == 2 && ident[0] == 'f' && ident[1] == 'n'{
            TokenType::FUNCTION
        }else{
            TokenType::IDENT
        }
    }
    pub fn new_from_literal(literal: Vec<char>) -> Token {
        Token { ttype: Token::lookup_ident(&literal), literal: literal }
    }

}
