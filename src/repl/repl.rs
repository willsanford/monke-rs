use std::io;

use crate::{token::token::*, lexer::lexer::*};

pub fn start(){

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut l: Lexer = Lexer::new(buffer.chars().into_iter().collect());
        
        loop {
            let tok: Token = l.next_token();
            println!("Token Type: {:?} | Token Literal: {}", tok.ttype, vts(tok.literal));
            if tok.ttype == TokenType::EOF {
                break;
            }
        }

    }


}