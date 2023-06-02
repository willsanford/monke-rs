
use crate::token::token::*;

trait Node {
   fn token_literal(&mut self) -> Vec<char>; 
}


struct Expression {

}
struct Statement {
    literal: Vec<char>
}

impl Node for Statement{
    fn token_literal(&mut self) -> Vec<char> {
       self.literal 
    }
}

struct Program {
    statements: Vec<Statement> 
}

struct Identifier{
    token: Token, 
    value: Vec<char>
}

impl Node for Identifier {
    fn token_literal(&mut self) -> Vec<char> {
        self.token.literal
    }
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expression
}

impl Node for LetStatement {
    fn token_literal(&mut self) -> Vec<char> {
        self.token.literal
    }
}

impl Node for Program {
    fn token_literal(&mut self) -> Vec<char> {
        if !self.statements.is_empty(){
            self.statements[0].token_literal()
        }else {
            vec![' ']
        }
    }
}