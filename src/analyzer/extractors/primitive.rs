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
