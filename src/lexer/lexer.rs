use crate::token::token::{Token, TokenType};

use log::{debug, info};

pub fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch >= 'Z' || ch == '_'
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0'
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn new(input: Vec<char>) -> Lexer {
        let mut l: Lexer = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_identifier(&mut self) ->  Vec<char> {
        let position = self.position;

        while is_letter(self.ch){
            self.read_char();
        }

        let mut ident: Vec<char> = Vec::new();
        ident.clone_from_slice(&self.input[position..self.position]);
        return ident;
    }

    pub fn skip_whitespace(&mut self){
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    pub fn next_token(&mut self) -> Token {
        info!("Reading next char: {}", self.ch);

        self.skip_whitespace();

        let tok: Token = match self.ch {
            '=' => Token::new(TokenType::ASSIGN, vec![self.ch]),
            ';' => Token::new(TokenType::SEMICOLON, vec![self.ch]),
            '(' => Token::new(TokenType::LPAREN, vec![self.ch]),
            ')' => Token::new(TokenType::RRAREN, vec![self.ch]),
            '{' => Token::new(TokenType::LBRACE, vec![self.ch]),
            '}' => Token::new(TokenType::RBRACE, vec![self.ch]),
            ',' => Token::new(TokenType::COMMA, vec![self.ch]),
            '+' => Token::new(TokenType::PLUS, vec![self.ch]),
            '\0' => Token::new(TokenType::EOF, vec!['\0']),
            _ => {
                if is_letter(self.ch){
                    Token::new_from_literal(
                        self.read_identifier()
                    ) 
                }else{
                    Token::new(TokenType::ILLEGAL, vec![self.ch])
                }
            } 
        };

        self.read_char();

        tok
    }
}
