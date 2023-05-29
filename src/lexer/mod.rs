pub mod lexer;

fn vfs(s: &str) -> Vec<char> {
    s.chars().collect()
}
#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::{Token, TokenType};

    use super::*;

    #[test]
    fn test_next_token_basic() {
        let input: &str = "=+(){},;";

        let tests: Vec<(TokenType, Vec<char>)> = vec![
            (TokenType::ASSIGN, vec!['=']),
            (TokenType::PLUS, vec!['+']),
            (TokenType::LPAREN, vec!['(']),
            (TokenType::RRAREN, vec![')']),
            (TokenType::LBRACE, vec!['{']),
            (TokenType::RBRACE, vec!['}']),
            (TokenType::COMMA, vec![',']),
            (TokenType::SEMICOLON, vec![';']),
            (TokenType::EOF, vec!['\0']),
        ];

        let mut l: Lexer = Lexer::new(input.chars().collect());
        for (ttype, literal) in tests {
            let tok: Token = l.next_token();

            assert_eq!(tok.ttype, ttype);
            assert_eq!(tok.literal, literal);
        }
    }

    #[test]
    fn test_next_token_advanced() {
        let input: &str = "
        let five = 5;
        let ten = 10;

        let add = fn(x, y){
            x + y;
        } 

        let result = add(five, ten);
        ";

        let tests: Vec<(TokenType, Vec<char>)> = vec![
            (TokenType::LET, vfs("let")),
            (TokenType::IDENT, vfs("five")),
            (TokenType::ASSIGN, vfs("=")),
            (TokenType::INT, vfs("5")),
            (TokenType::SEMICOLON, vfs(";")),
            (TokenType::LET, vfs("let")),
            (TokenType::IDENT, vfs("ten")),
            (TokenType::ASSIGN, vfs("=")),
            (TokenType::INT, vfs("10")),
            (TokenType::SEMICOLON, vfs(";")),
        ];

        let mut l: Lexer = Lexer::new(input.chars().collect());
        for (ttype, literal) in tests {
            let tok: Token = l.next_token();

            assert_eq!(tok.ttype, ttype);
            assert_eq!(tok.literal, literal);
        }
    }
}
