use std::default;

use super::{visitable::Visitable, visitor::Visitor, ExtractionError, Extractor, Target};
use crate::default_extractor;
use solang_parser::pt::*;

default_extractor!(MemberAccessExtractor, Expression);

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

default_extractor!(ForExtractor, Statement);

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

default_extractor!(EqualityExtractor, Expression);

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

default_extractor!(AssignmentExtractor, Expression);

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

default_extractor!(IncrementorExtractor, Expression);

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

default_extractor!(FunctionCallExtractor, Expression);

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

default_extractor!(BlockExtractor, Statement);

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

default_extractor!(FunctionExtractor, FunctionDefinition);

impl Visitor for FunctionExtractor {
    type Error = ExtractionError;
    fn extract_function(&mut self, function: &mut FunctionDefinition) -> Result<(), Self::Error> {
        self.targets.push(function.clone());
        Ok(())
    }
}

default_extractor!(ContractDefinitionExtractor, ContractDefinition);

impl Visitor for ContractDefinitionExtractor {
    type Error = ExtractionError;
    fn extract_contract(&mut self, contract: &mut ContractDefinition) -> Result<(), Self::Error> {
        self.targets.push(contract.clone());
        Ok(())
    }
}

default_extractor!(PragmaDirectiveExtractor, SourceUnitPart);

impl Visitor for PragmaDirectiveExtractor {
    type Error = ExtractionError;
    fn extract_source_unit_part(
        &mut self,
        source_unit_part: &mut SourceUnitPart,
    ) -> Result<(), Self::Error> {
        if let SourceUnitPart::PragmaDirective(_, _, _) = source_unit_part {
            self.targets.push(source_unit_part.clone());
        }
        Ok(())
    }
}

default_extractor!(StructDefinitionExtractor, StructDefinition);

impl Visitor for StructDefinitionExtractor {
    type Error = ExtractionError;
    fn extract_struct(&mut self, struct_def: &mut StructDefinition) -> Result<(), Self::Error> {
        self.targets.push(struct_def.clone());
        Ok(())
    }
}

default_extractor!(UsingListExtractor, UsingList);

impl Visitor for UsingListExtractor {
    type Error = ExtractionError;
    fn extract_using_list(&mut self, using_list: &mut UsingList) -> Result<(), Self::Error> {
        self.targets.push(using_list.clone());
        Ok(())
    }
}
