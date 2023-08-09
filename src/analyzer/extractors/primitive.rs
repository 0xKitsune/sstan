use std::{collections::HashSet, error::Error};

use solang_parser::pt::{self, Expression};

use super::{visit::Visitor, visitable::Visitable, ExtractionError, Extractor, Target};

pub struct MemberAccessExtractor {
    targets: Vec<pt::Expression>,
}

impl MemberAccessExtractor {
    pub fn new() -> Self {
        Self { targets: vec![] }
    }
}

impl Visitor for MemberAccessExtractor {
    type Error = ExtractionError;

    fn visit_expr(&mut self, _loc: pt::Loc, expr: &mut pt::Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::MemberAccess(_, _, _) => {
                self.targets.push(expr.clone());
            }

            _ => {}
        }

        Ok(())
    }
}

impl<V: Visitable> Extractor<V, pt::Expression> for MemberAccessExtractor {
    fn extract(v: &mut V) -> Result<Vec<pt::Expression>, ExtractionError> {
        let mut member_access_extractor = Self::new();
        v.visit(&mut member_access_extractor)?;
        Ok(member_access_extractor.targets)
    }
}
