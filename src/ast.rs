use crate::token::*;
use crate::lexer::*;


pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression)
}

pub struct Program{
    statements: Vec<Statement>,
}
pub enum Statement{
    Let(Let),
    Return(Return),
    Expr(Expression)
}
pub enum Expression{
    Identifier(Identifier)
}

pub struct Identifier{
    token: TokenType,
    value: Vec<char>

}

pub struct Let{
    token: TokenType,
    name: Identifier
}

pub struct Return{}

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            cur_token: Token::default(),
            peek_token: Token::default(),
        };

        p.next_token();
        p.next_token();

        p
    }

    fn parse_let_statement(&mut self) -> Option<Let> {
        unimplemented!()
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.ttype {
            TokenType::LET => None,
            _ => None,
        }
    }

    fn parse_program(&mut self) -> Result<Program, &'static str> {
        let mut program: Program = Program {
            statements: Vec::new(),
        };

        while self.cur_token.ttype != TokenType::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            } else {
                return Err("Bad read");
            }
            self.next_token();
        }

        return Ok(program);
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }
}



/*



trait Node {
    fn token_literal(&self) -> Vec<char>;
}
trait Expression: Node {
    fn token_literal(&self) -> Vec<char>;
}

trait Statement: Node {
    fn token_literal(&self) -> Vec<char>;
}

struct Identifier {
    token: TokenType,
    value: Vec<char>,
}

impl Node for Identifier {
    fn token_literal(&self) -> Vec<char> {
        self.value.clone()
    }
}

struct LetStatement<NameType, ValueType>
where
    NameType: Expression,
    ValueType: Expression,
{
    literal: Vec<char>,
    token: Token,
    name: NameType,
    value: ValueType,
}

impl<NameType, ValueType> Node for LetStatement<NameType, ValueType>
where
    NameType: Expression,
    ValueType: Expression,
{
    fn token_literal(&self) -> Vec<char> {
        self.literal.clone()
    }
}

pub struct Program<StatementType>
where
    StatementType: Statement,
{
    statements: Vec<StatementType>,
}

impl<StatementType> Program<StatementType>
where
    StatementType: Statement,
{
    fn new(l: Lexer) -> Program<StatementType> {
        unimplemented!()
    }
}

impl Node for Program {
    fn token_literal(&self) -> Vec<char> {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            vec![' ']
        }
    }
}

*/
