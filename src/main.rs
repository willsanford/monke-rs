mod ast;
mod lexer;
mod repl;
mod token;

fn main() {
    println!("Welcome to the Monke(y) language REPL!");
    repl::start();
}

#[cfg(test)]
mod tests {
    use ast::*;
    use lexer::*;
    use token::*;

    use super::*;

    #[test]
    fn test_let_statement() {
        let input: &str = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
            ";

        let l: Lexer = Lexer::new(input.chars().into_iter().collect());
        let p: Parser = Parser::new(l);
        let program: Program = Program();
    }
}

#[cfg(test)]
mod tests {
    use lexer::*;
    use token::*;

    use super::*;

    #[test]
    fn test_next_token_basic() {
        let input: &str = "=+(){},;";

        let tests: Vec<(TokenType, Vec<char>)> = vec![
            (TokenType::ASSIGN, vec!['=']),
            (TokenType::PLUS, vec!['+']),
            (TokenType::LPAREN, vec!['(']),
            (TokenType::RPAREN, vec![')']),
            (TokenType::LBRACE, vec!['{']),
            (TokenType::RBRACE, vec!['}']),
            (TokenType::COMMA, vec![',']),
            (TokenType::SEMICOLON, vec![';']),
            (TokenType::EOF, vec!['\0']),
        ];

        let mut l: Lexer = Lexer::new(input.chars().collect());

        for (ttype, literal) in tests {
            let tok: Token = l.next_token();

            println!("Next token: {:?} | {:?} ", tok.ttype, tok.literal);
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

        !-/*5;
        5 < 10 > 5;
        
        if (5 < 10){
            return true;
        }else{
            return false;
        }

        10 == 10;
        10 != 9; 
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
            (TokenType::LET, vfs("let")),
            (TokenType::IDENT, vfs("add")),
            (TokenType::ASSIGN, vfs("=")),
            (TokenType::FUNCTION, vfs("fn")),
            (TokenType::LPAREN, vfs("(")),
            (TokenType::IDENT, vfs("x")),
            (TokenType::COMMA, vfs(",")),
            (TokenType::IDENT, vfs("y")),
            (TokenType::RPAREN, vfs(")")),
            (TokenType::LBRACE, vfs("{")),
            (TokenType::IDENT, vfs("x")),
            (TokenType::PLUS, vfs("+")),
            (TokenType::IDENT, vfs("y")),
            (TokenType::SEMICOLON, vfs(";")),
            (TokenType::RBRACE, vfs("}")),
            (TokenType::LET, vfs("let")),
            (TokenType::IDENT, vfs("result")),
            (TokenType::ASSIGN, vfs("=")),
            (TokenType::IDENT, vfs("add")),
            (TokenType::LPAREN, vfs("(")),
            (TokenType::IDENT, vfs("five")),
            (TokenType::COMMA, vfs(",")),
            (TokenType::IDENT, vfs("ten")),
            (TokenType::RPAREN, vfs(")")),
            (TokenType::SEMICOLON, vfs(";")),
            (TokenType::BANG, vfs("!")),
            (TokenType::MINUS, vfs("-")),
            (TokenType::SLASH, vfs("/")),
            (TokenType::ASTERISK, vfs("*")),
            (TokenType::INT, vfs("5")),
            (TokenType::SEMICOLON, vfs(";")),
            (TokenType::INT, vfs("5")),
            (TokenType::LT, vfs("<")),
            (TokenType::INT, vfs("10")),
            (TokenType::GT, vfs(">")),
            (TokenType::INT, vfs("5")),
            (TokenType::SEMICOLON, vfs(";")),
            (TokenType::IF, vfs("if")),
            (TokenType::LPAREN, vfs("(")),
            (TokenType::INT, vfs("5")),
            (TokenType::LT, vfs("<")),
            (TokenType::INT, vfs("10")),
            (TokenType::RPAREN, vfs(")")),
            (TokenType::LBRACE, vfs("{")),
            (TokenType::RETURN, vfs("return")),
            (TokenType::TRUE, vfs("true")),
            (TokenType::SEMICOLON, vfs(";")),
            (TokenType::RBRACE, vfs("}")),
            (TokenType::ELSE, vfs("else")),
            (TokenType::LBRACE, vfs("{")),
            (TokenType::RETURN, vfs("return")),
            (TokenType::FALSE, vfs("false")),
            (TokenType::SEMICOLON, vfs(";")),
            (TokenType::RBRACE, vfs("}")),
            (TokenType::INT, vfs("10")),
            (TokenType::EQ, vfs("==")),
            (TokenType::INT, vfs("10")),
            (TokenType::SEMICOLON, vfs(";")),
            (TokenType::INT, vfs("10")),
            (TokenType::NOT_EQ, vfs("!=")),
            (TokenType::INT, vfs("9")),
            (TokenType::SEMICOLON, vfs(";")),
            (TokenType::EOF, vfs("\0")),
        ];

        let mut l: Lexer = Lexer::new(input.chars().collect());
        for (ttype, literal) in tests {
            let tok: Token = l.next_token();
            println!(
                "Next token: {:?} , {:?} | Expected token: {:?} , {:?}",
                tok.ttype, tok.literal, ttype, literal
            );

            assert_eq!(tok.ttype, ttype);
            assert_eq!(tok.literal, literal);
        }
    }
}
