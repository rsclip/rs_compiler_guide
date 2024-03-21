use crate::errors::SemanticError;
use crate::semantic_analysis::{Analysis, SymbolTable};
use crate::{ast::*, token::Span};
use anyhow::{anyhow, Error};
use log::{debug, warn};

#[derive(Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    VariableDecl(VariableDecl),
    Flow(FlowStatement),
    Return(Option<Box<Expression>>),
}

#[derive(Debug)]
pub struct VariableDecl {
    pub ident: Ident,
    pub ty: Type,
    pub expression: Expression,
    pub span: Span,
}

#[derive(Debug)]
pub struct FlowStatement {
    pub condition: Expression,
    pub if_block: Block,
    pub else_block: Option<Block>,
    pub span: Span,
}

impl PrettyPrint for Statement {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Statement::Expression(e) => e.pretty_print(indent),
            Statement::VariableDecl(v) => v.pretty_print(indent),
            Statement::Flow(f) => f.pretty_print(indent),
            Statement::Return(e) => format!(
                "{:indent$}Return\n{}",
                "",
                e.as_ref()
                    .map(|e| e.pretty_print(indent + 1))
                    .unwrap_or_else(|| "".to_string()),
                indent = indent * 4
            ),
        }
    }
}

impl PrettyPrint for VariableDecl {
    // format: "VariableDecl ident: ty =\n" + expression
    fn pretty_print(&self, indent: usize) -> String {
        format!(
            "{:indent$}VariableDecl {}:{} =\n{}",
            "",
            self.ident.ident,
            self.ty.pretty_print(0),
            self.expression.pretty_print(indent + 1),
            indent = indent * 4
        )
    }
}

impl PrettyPrint for FlowStatement {
    // format: "FlowStatement\ncond=\n" + condition + "if=\n" + if_block + "else=\n" + else_block
    fn pretty_print(&self, indent: usize) -> String {
        let mut s = format!("{:indent$}FlowStatement\n", "", indent = indent * 4);
        s.push_str(&self.condition.pretty_print(indent + 1));
        s.push_str(&self.if_block.pretty_print(indent + 1));
        if let Some(else_block) = &self.else_block {
            s.push_str(&else_block.pretty_print(indent + 1));
        }
        s
    }
}

impl ASTSpan for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::Expression(e) => e.span(),
            Statement::VariableDecl(v) => v.span.clone(),
            Statement::Flow(f) => f.span.clone(),
            Statement::Return(e) => e.as_ref().map(|e| e.span()).unwrap_or_default(),
        }
    }
}

impl Analysis for Statement {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        match self {
            Statement::Expression(e) => e.analyze(table),
            Statement::VariableDecl(v) => v.analyze(table),
            Statement::Flow(f) => f.analyze(table),
            Statement::Return(e) => e.as_ref().map_or_else(|| vec![], |e| e.analyze(table)),
        }
    }
}

impl Analysis for VariableDecl {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        debug!("Analyzing variable declaration: {:?}", self.ident.ident);
        let mut errors = Vec::new();

        match table.add_var(&self) {
            Ok(_) => (),
            Err(e) => errors.push(e),
        };

        debug!("Checking expression type: {:?}", self.expression);
        // check if expression type matches variable type
        match self.expression.get_type(table) {
            Ok(ty) => {
                if ty != self.ty {
                    warn!("Variable type does not match expression type: {:?}", self.ident.ident);
                    errors.push(anyhow!(SemanticError::TypesDoNotMatch {
                        expected_type: self.ty.clone(),
                        expected_span: self.ty.span(),
                        found_type: ty,
                        found_span: self.expression.span(),
                    }));
                }
            }
            Err(e) => {
                warn!("Error getting expression type: {:?}", self.ident.ident);
                errors.push(e);
                return errors;
            }
        };

        debug!("Variable declaration analysis errors: {:?}", errors);

        errors
    }
}

impl Analysis for FlowStatement {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        debug!("Analyzing flow statement: {:?}", self);
        let mut errors = Vec::new();

        // check if condition is a boolean
        debug!("Checking condition type: {:?}", self.condition);
        match self.condition.get_type(table) {

            Ok(ty) => {
                let bool_type = Type::Primitive(PrimitiveType {
                    kind: PrimitiveKind::Bool,
                    span: Span::default(),
                });

                if ty != bool_type {
                    errors.push(anyhow!(SemanticError::NonBooleanCondition {
                        found_type: ty,
                        found_span: self.condition.span(),
                    }));
                }
            }
            Err(e) => {
                warn!("Error getting condition type: {:?}", self.condition);
                errors.push(e);
                return errors;
            }
        };

        debug!("Analyzing if block: {:?}", self.if_block);
        errors.extend(self.if_block.analyze(table));

        if let Some(else_block) = &self.else_block {
            debug!("Analyzing else block: {:?}", else_block);
            errors.extend(else_block.analyze(table));
        }

        debug!("Flow statement analysis errors: {:?}", errors);

        errors
    }
}

