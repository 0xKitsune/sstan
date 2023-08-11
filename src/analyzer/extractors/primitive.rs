use std::{collections::HashSet, error::Error};

use solang_parser::pt::*;

use super::{visitable::Visitable, visitor::Visitor, ExtractionError, Extractor, Target};

macro_rules! extractor {
    ($extractor_name:ident, $target_type:ty) => {
        pub struct $extractor_name {
            targets: Vec<$target_type>,
        }

        impl $extractor_name {
            pub fn new() -> Self {
                Self { targets: vec![] }
            }
        }

        impl<V: Visitable> Extractor<V, $target_type> for $extractor_name {
            fn extract(v: &mut V) -> Result<Vec<$target_type>, ExtractionError> {
                let mut extractor_instance = Self::new();
                v.visit(&mut extractor_instance)?;
                Ok(extractor_instance.targets)
            }
        }
    };
}

extractor!(MemberAccessExtractor, Expression);

impl Visitor for MemberAccessExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::MemberAccess(_, _, _) => self.targets.push(expr.clone()),
            _ => {}
        }
        Ok(())
    }
}

extractor!(ForExtractor, Statement);

impl Visitor for ForExtractor {
    type Error = ExtractionError;
    fn extract_statement(&mut self, statement: &mut Statement) -> Result<(), Self::Error> {
        match statement {
            Statement::For(_, _, _, _, _) => self.targets.push(statement.clone()),
            _ => {}
        }
        Ok(())
    }
}

pub struct DefaultVisitor {}

impl Visitor for DefaultVisitor {
    type Error = ExtractionError;
}

extractor!(EqualityExtractor, Expression);

impl Visitor for EqualityExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::Equal(_, _, _)
            | Expression::NotEqual(_, _, _)
            | Expression::LessEqual(_, _, _)
            | Expression::MoreEqual(_, _, _)
            | Expression::Less(_, _, _)
            | Expression::More(_, _, _) => self.targets.push(expr.clone()),
            _ => {}
        }
        Ok(())
    }
}

extractor!(AssignmentExtractor, Expression);

impl Visitor for AssignmentExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::Assign(_, _, _)
            | Expression::AssignOr(_, _, _)
            | Expression::AssignAnd(_, _, _)
            | Expression::AssignXor(_, _, _)
            | Expression::AssignShiftLeft(_, _, _)
            | Expression::AssignShiftRight(_, _, _)
            | Expression::AssignAdd(_, _, _)
            | Expression::AssignSubtract(_, _, _)
            | Expression::AssignMultiply(_, _, _)
            | Expression::AssignDivide(_, _, _)
            | Expression::AssignModulo(_, _, _) => self.targets.push(expr.clone()),
            _ => {}
        }
        Ok(())
    }
}

extractor!(IncrementorExtractor, Expression);

impl Visitor for IncrementorExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::PreDecrement(_, _)
            | Expression::PostDecrement(_, _)
            | Expression::PreIncrement(_, _)
            | Expression::PostIncrement(_, _) => self.targets.push(expr.clone()),
            _ => {}
        }
        Ok(())
    }
}

extractor!(FunctionCallExtractor, Expression);

impl Visitor for FunctionCallExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::FunctionCall(_, _, _) => self.targets.push(expr.clone()),
            _ => {}
        }
        Ok(())
    }
}

extractor!(BlockExtractor, Statement);

impl Visitor for BlockExtractor {
    type Error = ExtractionError;
    fn extract_statement(&mut self, statement: &mut Statement) -> Result<(), Self::Error> {
        match statement {
            Statement::Block {
                loc: _,
                unchecked: _,
                statements: _,
            } => self.targets.push(statement.clone()),
            _ => {}
        }
        Ok(())
    }
}

extractor!(FunctionExtractor, FunctionDefinition);

impl Visitor for FunctionExtractor {
    type Error = ExtractionError;
    fn extract_function(&mut self, function: &mut FunctionDefinition) -> Result<(), Self::Error> {
        self.targets.push(function.clone());
        Ok(())
    }
}
