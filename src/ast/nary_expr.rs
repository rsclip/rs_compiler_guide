use crate::errors::SemanticError;
use crate::semantic_analysis::{Analysis, SymbolTable};
use crate::{ast::*, token::Span};
use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
pub struct UnaryExpression {
    pub kind: UnaryExpressionKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum UnaryExpressionKind {
    Negation(Box<Expression>),
    Not(Box<Expression>),
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub op: BinaryOperator,
    pub rhs: Box<Expression>,
    pub span: Span,
}

impl PrettyPrint for UnaryExpression {
    fn pretty_print(&self, indent: usize) -> String {
        match &self.kind {
            UnaryExpressionKind::Negation(e) => format!(
                "{:indent$}Negation\n{}\n",
                "",
                e.pretty_print(indent + 1),
                indent = indent * 4
            ),
            UnaryExpressionKind::Not(e) => format!(
                "{:indent$}Not\n{}\n",
                "",
                e.pretty_print(indent + 1),
                indent = indent * 4
            ),
        }
    }
}

impl PrettyPrint for BinaryExpression {
    // format: "BinaryExpression\nop=\n" + op + "lhs=\n" + lhs + "rhs=\n" + rhs
    fn pretty_print(&self, indent: usize) -> String {
        let mut s = format!("{:indent$}BinaryExpression\n", "", indent = indent * 4);
        s.push_str(&format!(
            "{:indent$}op={}\n",
            "",
            self.op.pretty_print(indent + 1),
            indent = indent * 4
        ));
        s.push_str(&self.lhs.pretty_print(indent + 1));
        s.push_str(&self.rhs.pretty_print(indent + 1));
        s
    }
}

impl UnaryExpression {
    fn analyze_negation(&self, table: &SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();
        
        let expr: &Box<Expression> = match &self.kind {
            UnaryExpressionKind::Negation(e) => e,
            _ => unreachable!(),
        };

        // check if unsupported type
        let expr_type = match expr.get_type(table) {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };

        match expr_type {
            Type::Primitive(ref prim_ty) => {
                match prim_ty.kind {
                    PrimitiveKind::Int | PrimitiveKind::Float => (),
                    _ => errors.push(anyhow!(SemanticError::UnsupportedUnaryOperation {
                        operator: "Negation".to_string(),
                        operand_type: expr_type.clone(),
                        span: self.span.clone(),
                    })),
                }
            }
        }

        errors
    }

    fn analyze_not(&self, table: &SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();
        
        let expr: &Box<Expression> = match &self.kind {
            UnaryExpressionKind::Not(e) => e,
            _ => unreachable!(),
        };

        // check if unsupported type
        let expr_type = match expr.get_type(table) {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };

        match expr_type {
            Type::Primitive(ref prim_ty) => {
                match prim_ty.kind {
                    PrimitiveKind::Bool => (),
                    _ => errors.push(anyhow!(SemanticError::UnsupportedUnaryOperation {
                        operator: "Not".to_string(),
                        operand_type: expr_type.clone(),
                        span: self.span.clone(),
                    })),
                }
            }
        }

        errors
    }

    pub fn get_type(&self, table: &SymbolTable) -> Result<Type> {
        let expr = match &self.kind {
            UnaryExpressionKind::Negation(e) => e,
            UnaryExpressionKind::Not(e) => e,
        };

        expr.get_type(table)
    }
}

impl Analysis for UnaryExpression {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        // no need to analyze the expression, the expression itself is analyzed
        match &self.kind {
            UnaryExpressionKind::Negation(_) => self.analyze_negation(table),
            UnaryExpressionKind::Not(_) => self.analyze_not(table),
        }
    }
}

impl Analysis for BinaryExpression {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        // validate both sides are same type
        let mut errors = Vec::new();

        let lhs_type = match self.lhs.get_type(table) {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };

        let rhs_type = match self.rhs.get_type(table) {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };

        if lhs_type != rhs_type {
            errors.push(anyhow!(SemanticError::TypesDoNotMatch {
                expected_type: lhs_type.clone(),
                expected_span: self.lhs.span(),
                found_type: rhs_type.clone(),
                found_span: self.rhs.span(),
            }));
        };

        // check if either side is not a valid type
        if !self.is_valid_type(&lhs_type) {
            errors.push(anyhow!(SemanticError::UnsupportedBinaryOperation {
                operator: self.op.kind.to_string(),
                operand_type: lhs_type,
                span: self.span.clone(),
            }));
        }

        if !self.is_valid_type(&rhs_type) {
            errors.push(anyhow!(SemanticError::UnsupportedBinaryOperation {
                operator: self.op.kind.to_string(),
                operand_type: rhs_type,
                span: self.span.clone(),
            }));
        }

        errors
    }
}

impl BinaryExpression {
    pub fn get_type(&self, table: &SymbolTable) -> Result<Type> {
        let lhs_type = self.lhs.get_type(table)?;
        let rhs_type = self.rhs.get_type(table)?;

        if lhs_type != rhs_type {
            return Err(anyhow!(SemanticError::TypesDoNotMatch {
                expected_type: lhs_type,
                expected_span: self.lhs.span(),
                found_type: rhs_type,
                found_span: self.rhs.span(),
            }));
        }

        Ok(lhs_type)
    }

    fn is_valid_type(&self, ty: &Type) -> bool {
        match ty {
            Type::Primitive(ref prim_ty) => {
                match prim_ty.kind {
                    PrimitiveKind::Int | PrimitiveKind::Float | PrimitiveKind::Bool => true,
                    _ => false,
                }
            }
        }
    }
}

impl std::fmt::Display for BinaryOperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperatorKind::Add => write!(f, "+"),
            BinaryOperatorKind::Subtract => write!(f, "-"),
            BinaryOperatorKind::Multiply => write!(f, "*"),
            BinaryOperatorKind::Divide => write!(f, "/"),
            BinaryOperatorKind::Modulus => write!(f, "%"),
            BinaryOperatorKind::And => write!(f, "&&"),
            BinaryOperatorKind::Or => write!(f, "||"),
            BinaryOperatorKind::Equal => write!(f, "=="),
            BinaryOperatorKind::NotEqual => write!(f, "!="),
            BinaryOperatorKind::LessThan => write!(f, "<"),
            BinaryOperatorKind::LessThanOrEqual => write!(f, "<="),
            BinaryOperatorKind::GreaterThan => write!(f, ">"),
            BinaryOperatorKind::GreaterThanOrEqual => write!(f, ">="),
        }
    }
}