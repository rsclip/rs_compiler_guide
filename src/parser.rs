//! Parses a token stream into an AST

use crate::ast::*;
use crate::errors::LangError;
use crate::token::{Span, Token, TokenKind};
use anyhow::{anyhow, Result};
use std::cell::RefCell;
use log::debug;

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
        debug!("Parsing function");
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

        let span = Span::combine(&ident.span, &block.span);

        let func = FunctionDecl {
            ident,
            parameters: params,
            ty,
            block,
            span,
        };

        debug!("Parsed function: {:#?}", func);

        Ok(func)
    }

    fn ident(&mut self) -> Result<Ident> {
        debug!("Parsing ident (no-end)");

        let current = self.current_or_eof()?;

        match current.kind {
            TokenKind::Ident(ref ident) => {
                let ident = ident.clone();
                self.advance();
                Ok(Ident {
                    ident,
                    span: current.span.clone(),
                })
            }
            _ => Err(anyhow!(LangError::ExpectedToken {
                expected: TokenKind::Ident("".to_string()),
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }

    fn parameter(&mut self) -> Result<Parameter> {
        debug!("Parsing parameter");

        // IDENTIFIER
        let ident = self.ident()?;

        // ":"
        self.expect(TokenKind::Colon)?;

        // type
        let ty = self.type_()?;

        let span = Span::combine(&ident.span, &ty.span());

        let param = Parameter { ident, ty, span };

        debug!("Parsed parameter: {:#?}", param);

        Ok(param)
    }

    fn type_(&mut self) -> Result<Type> {
        debug!("Parsing type (no-end)");

        match self.current_or_eof()?.kind {
            TokenKind::Int => {
                self.advance();
                Ok(Type::Primitive(PrimitiveType {
                    kind: PrimitiveKind::Int,
                    span: self.current_or_eof()?.span.clone(),
                }))
            }
            TokenKind::Bool => {
                self.advance();
                Ok(Type::Primitive(PrimitiveType {
                    kind: PrimitiveKind::Bool,
                    span: self.current_or_eof()?.span.clone(),
                }))
            }
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![TokenKind::Int, TokenKind::Bool,],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }

    fn block(&mut self) -> Result<Block> {
        debug!("Parsing block");

        let start_span = self.current_or_eof()?.span.clone();

        // "{"
        self.expect(TokenKind::LBrace)?;

        let mut statements = Vec::new();
        while self.current_or_eof()?.kind != TokenKind::RBrace {
            let statement = self.statement()?;
            statements.push(statement);
        }

        let end_span = self.current_or_eof()?.span.clone();

        // "}"
        self.expect(TokenKind::RBrace)?;

        let span = Span::combine(&start_span, &end_span);

        let block = Block { statements, span };

        debug!("Parsed block: {:#?}", block);

        Ok(block)
    }

    fn statement(&mut self) -> Result<Statement> {
        debug!("Parsing statement (no-end)");

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
        debug!("Parsing variable decl");

        // "let"
        let start_span = self.expect(TokenKind::Let)?.span.clone();

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

        let span = Span::combine(&start_span, &expression.span());

        let var_decl = Statement::VariableDecl(VariableDecl {
            ident,
            ty,
            expression,
            span,
        });

        // ";"
        self.expect(TokenKind::Semicolon)?;

        debug!("Parsed variable decl: {:#?}", var_decl);

        Ok(var_decl)
    }

    fn flow_statement(&mut self) -> Result<Statement> {
        debug!("Parsing flow statement");

        // "if"
        let start_span = self.expect(TokenKind::If)?.span.clone();

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

        let span = if let Some(else_block) = &else_block {
            Span::combine(&start_span, &else_block.span)
        } else {
            Span::combine(&start_span, &if_block.span)
        };

        let stmt = Statement::Flow(FlowStatement {
            condition,
            if_block,
            else_block,
            span,
        });

        debug!("Parsed flow statement: {:#?}", stmt);

        Ok(stmt)
    }

    fn return_statement(&mut self) -> Result<Statement> {
        debug!("Parsing return statement");

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

        debug!("Parsed return statement: {:#?}", stmt);

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
        debug!("Parsing expression");
        let mut expr = self.primary()?;

        while let Some(op) = self.current_or_eof()?.as_bin_op() {
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
            while let Some(unary_op) = self.current_or_eof()?.as_un_op() {
                // Consume the operator token
                self.advance();

                // Parse the right-hand side expression
                let rhs = self.primary()?;

                // let unary_expression = match unary_op {
                //     UnaryOperator::Negate => UnaryExpression::Negation(Box::new(rhs)),
                //     UnaryOperator::Not => UnaryExpression::Not(Box::new(rhs)),
                // };

                let unary_span = Span::combine(&unary_op.span, &rhs.span());

                let unary_expression = UnaryExpression {
                    kind: match unary_op.kind {
                        UnaryOperatorKind::Negate => UnaryExpressionKind::Negation(Box::new(rhs)),
                        UnaryOperatorKind::Not => UnaryExpressionKind::Not(Box::new(rhs)),
                    },
                    span: unary_span,
                };

                expr = Expression::Unary(unary_expression)
            }

            let span = Span::combine(&expr.span(), &rhs.span());

            // Parse binary expression
            expr = Expression::Binary(BinaryExpression {
                lhs: Box::new(expr),
                op: op,
                rhs: Box::new(rhs),
                span,
            });
        }

        debug!("Parsed expression: {:#?}", expr);

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression> {
        debug!("Parsing primary expression (no-end)");

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
        debug!("Parsing literal (no-end)");

        let current = self.current_or_eof()?;

        match current.kind {
            TokenKind::IntLiteral(value) => {
                self.advance();
                Ok(PrimaryExpression::Literal(Literal {
                    kind: LiteralKind::Int(value),
                    span: current.span.clone(),
                }))
            }
            TokenKind::BoolLiteral(value) => {
                self.advance();
                Ok(PrimaryExpression::Literal(Literal {
                    kind: LiteralKind::Bool(value),
                    span: current.span.clone(),
                }))
            }
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![TokenKind::IntLiteral(0), TokenKind::BoolLiteral(false),],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }

    fn expression_list(&mut self) -> Result<Vec<Expression>> {
        debug!("Parsing expression list");

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
    pub fn parse(&mut self, file_id: String) -> Result<AST> {
        let program = self.program()?;
        Ok(AST { program, file_id })
    }
}
