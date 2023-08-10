use std::{collections::HashSet, error::Error};

use solang_parser::pt::*;

use super::{visitable::Visitable, visitor::Visitor, ExtractionError, Extractor, Target};

pub struct MemberAccessExtractor {
    targets: Vec<Expression>,
}

impl MemberAccessExtractor {
    pub fn new() -> Self {
        Self { targets: vec![] }
    }
}

impl Visitor for MemberAccessExtractor {
    type Error = ExtractionError;

    fn visit_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::MemberAccess(_, _, _) => {
                self.targets.push(expr.clone());
            }

            _ => {}
        }

        Ok(())
    }
}

impl<V: Visitable> Extractor<V, Expression> for MemberAccessExtractor {
    fn extract(v: &mut V) -> Result<Vec<Expression>, ExtractionError> {
        let mut member_access_extractor = Self::new();
        v.visit(&mut member_access_extractor)?;
        Ok(member_access_extractor.targets)
    }
}

pub struct ForExtractor {
    targets: Vec<Statement>,
}

impl ForExtractor {
    pub fn new() -> Self {
        Self { targets: vec![] }
    }
}

impl Visitor for ForExtractor {
    type Error = ExtractionError;

    fn visit_statement(&mut self, statement: &mut Statement) -> Result<(), Self::Error> {
        match statement {
            Statement::For(_, _, _, _, _) => {
                self.targets.push(statement.clone());
            }

            _ => {}
        }

        Ok(())
    }
}

impl<V: Visitable> Extractor<V, Statement> for ForExtractor {
    fn extract(v: &mut V) -> Result<Vec<Statement>, ExtractionError> {
        let mut for_extractor = Self::new();
        v.visit(&mut for_extractor)?;
        Ok(for_extractor.targets)
    }
}

pub struct EqualityExtractor {
    targets: Vec<Expression>,
}

impl EqualityExtractor {
    pub fn new() -> Self {
        Self { targets: vec![] }
    }
}

impl Visitor for EqualityExtractor {
    type Error = ExtractionError;

    fn visit_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::Equal(_, _, _)
            | Expression::NotEqual(_, _, _)
            | Expression::LessEqual(_, _, _)
            | Expression::MoreEqual(_, _, _)
            | Expression::Less(_, _, _)
            | Expression::More(_, _, _) => {
                self.targets.push(expr.clone());
            }

            _ => {}
        }

        Ok(())
    }
}

impl<V: Visitable> Extractor<V, Expression> for EqualityExtractor {
    fn extract(v: &mut V) -> Result<Vec<Expression>, ExtractionError> {
        let mut equality_extractor = Self::new();
        v.visit(&mut equality_extractor)?;
        Ok(equality_extractor.targets)
    }
}

pub struct AssignmentExtractor {
    targets: Vec<Expression>,
}

impl AssignmentExtractor {
    pub fn new() -> Self {
        Self { targets: vec![] }
    }
}

impl Visitor for AssignmentExtractor {
    type Error = ExtractionError;

    fn visit_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
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
            | Expression::AssignModulo(_, _, _) => {
                self.targets.push(expr.clone());
            }

            _ => {}
        }

        Ok(())
    }
}

impl<V: Visitable> Extractor<V, Expression> for AssignmentExtractor {
    fn extract(v: &mut V) -> Result<Vec<Expression>, ExtractionError> {
        let mut equality_extractor = Self::new();
        v.visit(&mut equality_extractor)?;
        Ok(equality_extractor.targets)
    }
}

pub struct IncrementorExtractor {
    targets: Vec<Expression>,
}

impl IncrementorExtractor {
    pub fn new() -> Self {
        Self { targets: vec![] }
    }
}

impl Visitor for IncrementorExtractor {
    type Error = ExtractionError;

    fn visit_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::PreDecrement(_, _)
            | Expression::PostDecrement(_, _)
            | Expression::PreIncrement(_, _)
            | Expression::PostIncrement(_, _) => {
                self.targets.push(expr.clone());
            }

            _ => {}
        }

        Ok(())
    }
}

impl<V: Visitable> Extractor<V, Expression> for IncrementorExtractor {
    fn extract(v: &mut V) -> Result<Vec<Expression>, ExtractionError> {
        let mut equality_extractor = Self::new();
        v.visit(&mut equality_extractor)?;
        Ok(equality_extractor.targets)
    }
}

pub struct BlockExtractor {
    targets: Vec<Statement>,
}

impl BlockExtractor {
    pub fn new() -> Self {
        Self { targets: vec![] }
    }
}

impl Visitor for BlockExtractor {
    type Error = ExtractionError;

    fn visit_statement(&mut self, statement: &mut Statement) -> Result<(), Self::Error> {
        match statement {
            Statement::Block {
                loc: _,
                unchecked: _,
                statements: _,
            } => self.targets.push(*statement),
            _ => {}
        }

        Ok(())
    }
}

impl<V: Visitable> Extractor<V, Statement> for BlockExtractor {
    fn extract(v: &mut V) -> Result<Vec<Statement>, ExtractionError> {
        let mut equality_extractor = Self::new();
        v.visit(&mut equality_extractor)?;
        Ok(equality_extractor.targets)
    }
}
