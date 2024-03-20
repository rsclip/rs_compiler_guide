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
    /// Same as peeking
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos())
    }

    /// Get current token
    /// Raises an error if at EOF
    fn current_or_eof(&self) -> Result<&Token> {
        self.current()
            .filter(|token| token.kind != TokenKind::Eof)
            .ok_or_else(|| anyhow!(self.eof_error()))
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

    fn ident(&mut self) -> Result<Ident> {
        println!("Parsing ident (no-end)");

        match self.current_or_eof()?.kind {
            TokenKind::Ident(ref ident) => {
                let ident = ident.clone();
                self.advance();
                Ok(Ident { ident })
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
            TokenKind::Return => self.return_statement(),
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

        // may be else, may be not
        let else_block = if self.current_or_eof()?.kind == TokenKind::Else {
            self.advance();
            Some(self.block()?)
        } else {
            None
        };

        let stmt = Statement::Flow(FlowStatement {
            condition,
            if_block,
            else_block,
        });

        println!("Parsed flow statement: {:#?}", stmt);

        Ok(stmt)
    }

    fn return_statement(&mut self) -> Result<Statement> {
        println!("Parsing return statement");

        // "return"
        self.expect(TokenKind::Return)?;

        // expression or not
        let expression = if self.current_or_eof()?.kind != TokenKind::Semicolon {
            Some(Box::new(self.expression()?))
        } else {
            None
        };

        // ";"
        self.expect(TokenKind::Semicolon)?;

        let stmt = Statement::Return(expression);

        println!("Parsed return statement: {:#?}", stmt);

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
        let mut expr = self.primary()?;

        while let Some(op) = self.current_or_eof()?.kind.as_binary_operator() {
            let op_precedence = op.precedence();

            // Check if the operator precedence is higher than the previous operator
            if let Some(prev_op) = expr.binary_operator() {
                if op_precedence <= prev_op.precedence() {
                    break; // Break the loop if the current operator has lower precedence
                }
            }

            // Consume the operator token
            self.advance();

            // Parse the right-hand side expression
            let rhs = self.primary()?;

            // Handle unary operators
            while let Some(unary_op) = self.current_or_eof()?.kind.as_unary_operator() {
                // Consume the operator token
                self.advance();

                // Parse the right-hand side expression
                let rhs = self.primary()?;

                let unary_expression = match unary_op {
                    UnaryOperator::Negate => UnaryExpression::Negation(Box::new(rhs)),
                    UnaryOperator::Not => UnaryExpression::Not(Box::new(rhs)),
                };

                expr = Expression::Unary(unary_expression);
            }

            // Parse binary expression
            expr = Expression::Binary(BinaryExpression {
                lhs: Box::new(expr),
                op: op,
                rhs: Box::new(rhs),
            });
        }

        println!("Parsed expression: {:#?}", expr);

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression> {
        println!("Parsing primary expression (no-end)");

        match self.current_or_eof()?.kind {
            TokenKind::IntLiteral(_) | TokenKind::BoolLiteral(_) => {
                self.literal().map(Expression::Primary)
            }
            TokenKind::Ident(_) => {
                let ident = self.ident()?;

                // if the next token is a "(", then it's a function call
                if self.current_or_eof()?.kind == TokenKind::LParen {
                    self.advance();
                    let args = self.expression_list()?;
                    self.expect(TokenKind::RParen)?;

                    Ok(Expression::Primary(PrimaryExpression::FunctionCall(
                        ident, args,
                    )))
                } else {
                    Ok(Expression::Primary(PrimaryExpression::Ident(ident)))
                }
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.expect(TokenKind::RParen)?;
                Ok(Expression::Primary(PrimaryExpression::Parenthesized(
                    Box::new(expr),
                )))
            }
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![
                    TokenKind::IntLiteral(0),
                    TokenKind::BoolLiteral(false),
                    TokenKind::Ident("".to_string()),
                    TokenKind::LParen,
                ],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }

    fn literal(&mut self) -> Result<PrimaryExpression> {
        println!("Parsing literal (no-end)");

        match self.current_or_eof()?.kind {
            TokenKind::IntLiteral(value) => {
                self.advance();
                Ok(PrimaryExpression::Literal(Literal::Int(value)))
            }
            TokenKind::BoolLiteral(value) => {
                self.advance();
                Ok(PrimaryExpression::Literal(Literal::Bool(value)))
            }
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![TokenKind::IntLiteral(0), TokenKind::BoolLiteral(false),],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }

    fn expression_list(&mut self) -> Result<Vec<Expression>> {
        println!("Parsing expression list");

        let mut args = Vec::new();
        while self.current_or_eof()?.kind != TokenKind::RParen {
            let arg = self.expression()?;
            args.push(arg);

            if self.current_or_eof()?.kind == TokenKind::Comma {
                self.advance();
            }
        }

        Ok(args)
    }

    // ====================
    // Parse the token stream
    // ====================

    /// Parse the token stream into an AST
    pub fn parse(&mut self, file_id: usize) -> Result<AST> {
        let program = self.program()?;
        Ok(AST { program, file_id })
    }
}
