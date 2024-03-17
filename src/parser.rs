//! Parses a token stream into an AST

use crate::ast::*;
use crate::errors::LangError;
use crate::token::{Token, TokenKind};
use anyhow::{anyhow, Result};
use std::cell::RefCell;

/// Main parser struct
/// Parses a program into an AST
pub struct Parser {
    tokens: Vec<Token>,
    pos: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: RefCell::new(0),
        }
    }

    /// Get position
    fn pos(&self) -> usize {
        *self.pos.borrow()
    }

    /// Get current token
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos())
    }

    /// Get current token
    /// Raises an error if at EOF
    fn current_or_eof(&self) -> Result<&Token> {
        self.current().ok_or_else(|| anyhow!(self.eof_error()))
    }

    /// Assert not at EOF or empty token at current position
    fn assert_not_eof(&self) -> Result<()> {
        if self.current().is_none() {
            Err(anyhow!(self.eof_error()))
        } else {
            Ok(())
        }
    }

    /// Get the last token and return an EOF error
    fn eof_error(&self) -> LangError {
        let token = self.tokens.last().unwrap();
        LangError::UnexpectedEOF(token.span.clone())
    }

    /// Advance the parser by one token
    fn advance(&self) {
        *self.pos.borrow_mut() += 1;
    }

    // ====================
    // Parsing helpers
    // ====================

    /// Expect a token and advance
    /// Does not work with EOF
    fn expect(&mut self, kind: TokenKind) -> Result<&Token> {
        let current: &Token = self.current_or_eof()?;
        self.advance();

        if current.kind == kind {
            Ok(current)
        } else {
            Err(anyhow!(LangError::ExpectedToken {
                expected: kind,
                found: current.kind.clone(),
                span: current.span.clone(),
            }))
        }
    }

    // ====================
    // Parsing functions
    // ====================

    /// Parse a program
    fn program(&mut self) -> Result<Program> {
        let mut items = Vec::new();
        while self.current().is_some() {
            items.push(self.item()?);
        }

        Ok(Program { items })
    }

    /// Parse a single item
    fn item(&mut self) -> Result<Item> {
        self.assert_not_eof()?;
        let current = self.current_or_eof()?;

        let item = match current.kind {
            TokenKind::Fn => Item::FunctionDecl(self.function()?),
            _ => {
                return Err(anyhow!(LangError::ExpectedAnyToken {
                    expected: vec![TokenKind::Fn,],
                    found: current.kind.clone(),
                    span: current.span.clone(),
                }))
            }
        };

        Ok(item)
    }

    fn function(&mut self) -> Result<FunctionDecl> {
        println!("Parsing function");
        // "fn"
        self.expect(TokenKind::Fn)?;

        // IDENTIFIER
        let ident = self.ident()?;

        // "("
        self.expect(TokenKind::LParen)?;

        // parameter list
        let mut params: Vec<Parameter> = Vec::new();
        while self.current_or_eof()?.kind != TokenKind::RParen {
            let param = self.parameter()?;
            params.push(param);

            if self.current_or_eof()?.kind == TokenKind::Comma {
                self.advance();
            }
        }

        // ")"
        self.expect(TokenKind::RParen)?;

        // "->"
        self.expect(TokenKind::Arrow)?;

        // type
        let ty = self.type_()?;

        // block
        let block = self.block()?;

        let func = FunctionDecl {
            ident,
            parameters: params,
            ty,
            block,
        };

        println!("Parsed function: {:#?}", func);

        Ok(func)
    }

    fn ident(&mut self) -> Result<String> {
        println!("Parsing ident (no-end)");

        match self.current_or_eof()?.kind {
            TokenKind::Ident(ref ident) => {
                let ident = ident.clone();
                self.advance();
                Ok(ident)
            }
            _ => Err(anyhow!(LangError::ExpectedToken {
                expected: TokenKind::Ident("".to_string()),
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }

    fn parameter(&mut self) -> Result<Parameter> {
        println!("Parsing parameter");

        // IDENTIFIER
        let ident = self.ident()?;

        // ":"
        self.expect(TokenKind::Colon)?;

        // type
        let ty = self.type_()?;

        let param = Parameter { ident, ty };

        println!("Parsed parameter: {:#?}", param);

        Ok(param)
    }

    fn type_(&mut self) -> Result<Type> {
        println!("Parsing type (no-end)");

        match self.current_or_eof()?.kind {
            TokenKind::Int => {
                self.advance();
                Ok(Type::Primitive(PrimitiveType::Int))
            }
            TokenKind::Bool => {
                self.advance();
                Ok(Type::Primitive(PrimitiveType::Bool))
            }
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![TokenKind::Int, TokenKind::Bool,],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }

    fn block(&mut self) -> Result<Block> {
        println!("Parsing block");

        // "{"
        self.expect(TokenKind::LBrace)?;

        let mut statements = Vec::new();
        while self.current_or_eof()?.kind != TokenKind::RBrace {
            let statement = self.statement()?;
            statements.push(statement);
        }

        // "}"
        self.expect(TokenKind::RBrace)?;

        let block = Block { statements };

        println!("Parsed block: {:#?}", block);

        Ok(block)
    }

    fn statement(&mut self) -> Result<Statement> {
        println!("Parsing statement (no-end)");

        match self.current_or_eof()?.kind {
            TokenKind::Let => self.variable_decl(),
            TokenKind::If => self.flow_statement(),
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![TokenKind::Let,],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }

    fn variable_decl(&mut self) -> Result<Statement> {
        println!("Parsing variable decl");

        // "let"
        self.expect(TokenKind::Let)?;

        // IDENTIFIER
        let ident = self.ident()?;

        // ":"
        self.expect(TokenKind::Colon)?;

        // type
        let ty = self.type_()?;

        // "="
        self.expect(TokenKind::Equals)?;

        // expression
        let expression = self.expression()?;

        let var_decl = Statement::VariableDecl(VariableDecl {
            ident,
            ty,
            expression,
        });

        // ";"
        self.expect(TokenKind::Semicolon)?;

        println!("Parsed variable decl: {:#?}", var_decl);

        Ok(var_decl)
    }

    fn flow_statement(&mut self) -> Result<Statement> {
        println!("Parsing flow statement");

        // "if"
        self.expect(TokenKind::If)?;

        // condition
        let condition = self.expression()?;

        // block
        let if_block = self.block()?;

        // "else"
        self.expect(TokenKind::Else)?;

        // block
        let else_block = self.block()?;

        let stmt = Statement::Flow(FlowStatement {
            condition,
            if_block,
            else_block: Some(else_block),
        });

        println!("Parsed flow statement: {:#?}", stmt);

        Ok(stmt)
    }

    // ====================
    // Expression parsing
    // Should support:
    // - Primary expressions (literals, identifiers, function calls)
    // - Unary expressions (unary operators)
    // - Binary expressions (binary operators)
    // ====================

    fn expression(&mut self) -> Result<Expression> {
        println!("Parsing expression");
        let expr = self.primary_expression();

        println!("Parsed expression: {:#?}", expr);

        expr
    }

    fn primary_expression(&mut self) -> Result<Expression> {
        println!("Parsing primary expression (no-end)");

        match self.current_or_eof()?.kind {
            TokenKind::IntLiteral(ref value) => {
                let value = value.clone();
                self.advance();
                Ok(Expression::Primary(PrimaryExpression::Literal(
                    Literal::Int(value),
                )))
            }
            TokenKind::BoolLiteral(ref value) => {
                let value = value.clone();
                self.advance();
                Ok(Expression::Primary(PrimaryExpression::Literal(
                    Literal::Bool(value),
                )))
            }
            TokenKind::Ident(ref ident) => {
                let ident = ident.clone();
                self.advance();
                Ok(Expression::Primary(PrimaryExpression::Ident(ident)))
            }
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![
                    TokenKind::IntLiteral(0),
                    TokenKind::BoolLiteral(true),
                    TokenKind::Ident("".to_string()),
                ],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }

    // ====================
    // Parse the token stream
    // ====================

    /// Parse the token stream into an AST
    pub fn parse(&mut self, file_id: usize) -> Result<AST> {
        let program = self.program()?;
        Ok(AST {
            program,
            file_id,
        })
    }
}
