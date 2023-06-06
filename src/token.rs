#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers and Literals
    IDENT,
    INT,

    // Operators
    EQ,
    NOT_EQ,
    PLUS,
    ASSIGN,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE, // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
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
        // TODO :: Exact matching for now. This can definitely be done better
        if ident.len() == 3 && ident[0] == 'l' && ident[1] == 'e' && ident[2] == 't' {
            TokenType::LET
        } else if ident.len() == 2 && ident[0] == 'f' && ident[1] == 'n' {
            TokenType::FUNCTION
        } else if ident.len() == 2 && ident[0] == 'i' && ident[1] == 'f' {
            TokenType::IF
        } else if ident.len() == 4
            && ident[0] == 't'
            && ident[1] == 'r'
            && ident[2] == 'u'
            && ident[3] == 'e'
        {
            TokenType::TRUE
        } else if ident.len() == 4
            && ident[0] == 'e'
            && ident[1] == 'l'
            && ident[2] == 's'
            && ident[3] == 'e'
        {
            TokenType::ELSE
        } else if ident.len() == 5
            && ident[0] == 'f'
            && ident[1] == 'a'
            && ident[2] == 'l'
            && ident[3] == 's'
            && ident[4] == 'e'
        {
            TokenType::FALSE
        } else if ident.len() == 6
            && ident[0] == 'r'
            && ident[1] == 'e'
            && ident[2] == 't'
            && ident[3] == 'u'
            && ident[4] == 'r'
            && ident[5] == 'n'
        {
            TokenType::RETURN
        } else {
            TokenType::IDENT
        }
    }
    pub fn new_from_literal(literal: Vec<char>) -> Token {
        Token {
            ttype: Token::lookup_ident(&literal),
            literal: literal,
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token {
            ttype: TokenType::ILLEGAL,
            literal: vec!['0'],
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            ttype: self.ttype.clone(),
            literal: self.literal.clone(),
        }
    }
}
