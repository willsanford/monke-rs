use crate::token::token::{Token, TokenType};

use log::{debug, trace, info};

pub fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

pub fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
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
            self.ch = '\0'
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
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

    fn read_forward(&mut self, pred: fn(char) -> bool) -> Vec<char> {
        let position = self.position;

        while pred(self.peek_char()) {
            self.read_char();
        }

        self.input[position..self.position + 1]
            .into_iter()
            .map(|c| *c)
            .collect()
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::EQ, vec![ch, self.ch])
                } else {
                    Token::new(TokenType::ASSIGN, vec![self.ch])
                }
            }
            ';' => Token::new(TokenType::SEMICOLON, vec![self.ch]),
            '(' => Token::new(TokenType::LPAREN, vec![self.ch]),
            ')' => Token::new(TokenType::RPAREN, vec![self.ch]),
            '{' => Token::new(TokenType::LBRACE, vec![self.ch]),
            '}' => Token::new(TokenType::RBRACE, vec![self.ch]),
            ',' => Token::new(TokenType::COMMA, vec![self.ch]),
            '+' => Token::new(TokenType::PLUS, vec![self.ch]),
            '-' => Token::new(TokenType::MINUS, vec![self.ch]),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::NOT_EQ, vec![ch, self.ch])
                } else {
                    Token::new(TokenType::BANG, vec![self.ch])
                }
            }
            '*' => Token::new(TokenType::ASTERISK, vec![self.ch]),
            '/' => Token::new(TokenType::SLASH, vec![self.ch]),
            '<' => Token::new(TokenType::LT, vec![self.ch]),
            '>' => Token::new(TokenType::GT, vec![self.ch]),
            '\0' => Token::new(TokenType::EOF, vec!['\0']),
            _ => {
                if is_letter(self.ch) {
                    Token::new_from_literal(self.read_forward(is_letter))
                } else if is_digit(self.ch) {
                    Token::new(TokenType::INT, self.read_forward(is_digit))
                } else {
                    Token::new(TokenType::ILLEGAL, vec![self.ch])
                }
            }
        };

        self.read_char();

        tok
    }
}
