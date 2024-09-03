
use crate::lexer::{Token, Tokentype};
use crate::ast::{ASTNode, NumberNode,VariableNode,VariableDec,BlockNode,ConditionalNode,BinaryOpNode};
use std::rc::Rc;


pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Rc<dyn ASTNode> {
        let mut block_node = BlockNode::new();
        while !self.is_at_end() {
            block_node.add_stat(self.parse_stat());
        }
        Rc::new(block_node)
    }

    fn parse_stat(&mut self) -> Rc<dyn ASTNode> {
        if self.match_types(&[Tokentype::INT]) {
            self.parse_var_dec()
        } else if self.match_types(&[Tokentype::IF]) {
            self.parse_cond()
        } else if self.match_types(&[Tokentype::IDENTIFIER]) {
            self.parse_var_assign()
        } else {
            self.parse_exp_stat()
        }
    }

    fn parse_var_assign(&mut self) -> Rc<dyn ASTNode> {
        let var_name = self.previous().value.clone();

        // Semantic check: Ensure variable is declared
        if !VariableNode::is_declared(&var_name) {
            panic!("Semantic Error: Variable '{}' not declared.", var_name);
        }

        self.consume(Tokentype::EQ, "Expected '=' after variable name.");
        let val = self.parse_exp();
        self.consume(Tokentype::SEMICOLON, "Expected ';' after expression.");
        Rc::new(VariableDec::new(var_name, val))
    }

    fn parse_var_dec(&mut self) -> Rc<dyn ASTNode> {

        self.consume(Tokentype::IDENTIFIER, "Expected variable name.");
        let var_name = self.previous().value.clone();
        println!("Curr Position: {}", self.pos);
        // Semantic check: Variable redeclaration check
        if VariableNode::is_declared(&var_name) {
            panic!("Semantic Error: Variable '{}' already declared.", var_name);
        }

        let val = if self.match_types(&[Tokentype::EQ]) {
            self.parse_exp()
        } else {
            self.consume(Tokentype::SEMICOLON, "Expected ';' after variable declaration.");
            Rc::new(VariableNode::new(var_name.clone()))
        };

        println!("Curr Position: {}", self.pos);
        self.consume(Tokentype::SEMICOLON, "Expected ';' after expression.");
        Rc::new(VariableDec::new(var_name, val))
    }

    fn parse_cond(&mut self) -> Rc<dyn ASTNode> {
        self.consume(Tokentype::LPAREN, "Expected '(' after 'if'.");
        let cond = self.parse_exp();
        self.consume(Tokentype::RPAREN, "Expected ')' after condition.");
        let then_branch = self.parse_block();
        let else_branch = if self.match_types(&[Tokentype::ELSE]) {
            Some(self.parse_block())
        } else {
            None
        };

        Rc::new(ConditionalNode::new(cond, then_branch, else_branch))
    }

    fn parse_block(&mut self) -> Rc<dyn ASTNode> {
        if self.match_types(&[Tokentype::LBRACE]) {
            let mut block_node = BlockNode::new();
            while !self.check(Tokentype::RBRACE) && !self.is_at_end() {
                block_node.add_stat(self.parse_stat());
            }
            self.consume(Tokentype::RBRACE, "Expected '}' after block.");
            Rc::new(block_node)
        } else {
            self.parse_stat()
        }
    }

    fn parse_exp_stat(&mut self) -> Rc<dyn ASTNode> {
        let expr = self.parse_exp();
        self.consume(Tokentype::SEMICOLON, "Expected ';' after expression.");
        expr
    }

    fn parse_exp(&mut self) -> Rc<dyn ASTNode> {
        let mut left = self.parse_prim(); 

        println!("Curr Position outside while: {}", self.pos);


        while self.match_types(&[Tokentype::PLUS, Tokentype::MINUS, Tokentype::EQ, Tokentype::EQUAL]) {
            let op = self.previous().value.clone();
            let right = self.parse_prim();
            println!("Curr Position inside while: {}", self.pos);
            left = Rc::new(  BinaryOpNode::new(left, op, right));
        }

        left
    }

    fn  parse_prim(&mut self) -> Rc<dyn ASTNode> {
        if self.match_types(&[Tokentype::NUMBER]) {
            println!("Number {}", self.previous().value);
            Rc::new(NumberNode::new(self.previous().value.parse().unwrap()))
        } else if self.match_types(&[Tokentype::IDENTIFIER]) {
            Rc::new(VariableNode::new(self.previous().value.clone()))
        } else if self.match_types(&[Tokentype::LPAREN]) {
            let expr = self.parse_exp();
            self.consume(Tokentype::RPAREN, "Expected ')' after expression.");
            expr
        } else {
            panic!("Unexpected token '{}'", self.tokens[self.pos].value);
        }
    }


    fn match_types(&mut self, types: &[Tokentype]) -> bool {
        if self.is_at_end() {
            return false;
        }
        if types.contains(&self.tokens[self.pos].typ) {
            self.advance();
            return true;
        }
        false
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len() || self.tokens[self.pos].typ == Tokentype::END
    }

    fn check(&self, typ: Tokentype) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.tokens[self.pos].typ == typ
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.pos - 1]
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.pos += 1;
        }
    }

    fn consume(&mut self, typ: Tokentype, error_message: &str) {
        if self.check(typ) {
            self.advance();
        } else {
            panic!("{} Found '{}'", error_message, self.tokens[self.pos].value);
        }
    }
}

