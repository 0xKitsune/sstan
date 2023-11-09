use std::collections::HashSet;

use super::{visitable::Visitable, visitor::Visitor, ExtractionError, Extractor};
use crate::default_extractor;
use solang_parser::pt::*;

default_extractor!(MemberAccessExtractor, Expression);

impl Visitor for MemberAccessExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        if let Expression::MemberAccess(_, _, _) = expr {
            self.targets.push(expr.clone())
        }
        Ok(())
    }
}

default_extractor!(ForExtractor, Statement);

impl Visitor for ForExtractor {
    type Error = ExtractionError;
    fn extract_statement(&mut self, statement: &mut Statement) -> Result<(), Self::Error> {
        if let Statement::For(_, _, _, _, _) = statement {
            self.targets.push(statement.clone())
        }
        Ok(())
    }
}

default_extractor!(PlainImportExtractor, Import);

impl Visitor for PlainImportExtractor {
    type Error = ExtractionError;
    fn extract_import(
        &mut self,
        import: &mut solang_parser::pt::Import,
    ) -> Result<(), Self::Error> {
        if let Import::Plain(_, _) = import {
            self.targets.push(import.clone())
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

impl EqualityExtractor {
    pub fn extract_not_equal(exprs: Vec<Expression>) -> Vec<Expression> {
        let mut extracted = Vec::new();
        for expr in exprs {
            if let Expression::NotEqual(_, _, _) = expr {
                extracted.push(expr)
            }
        }
        extracted
    }

    pub fn extract_equal(exprs: Vec<Expression>) -> Vec<Expression> {
        let mut extracted = Vec::new();
        for expr in exprs {
            if let Expression::Equal(_, _, _) = expr {
                extracted.push(expr)
            }
        }
        extracted
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

default_extractor!(NumberLiteralExtractor, Expression);

impl Visitor for NumberLiteralExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        if let Expression::NumberLiteral(_, _, _, _) = expr {
            self.targets.push(expr.clone())
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

default_extractor!(ErrorExtractor, ErrorDefinition);

impl Visitor for ErrorExtractor {
    type Error = ExtractionError;
    fn extract_error(&mut self, _error: &mut ErrorDefinition) -> Result<(), Self::Error> {
        self.targets.push(_error.clone());
        Ok(())
    }
}

default_extractor!(FunctionCallExtractor, Expression);

impl Visitor for FunctionCallExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        if let Expression::FunctionCall(_, _, _) = expr {
            self.targets.push(expr.clone())
        }
        Ok(())
    }
}

default_extractor!(BlockExtractor, Statement);

impl Visitor for BlockExtractor {
    type Error = ExtractionError;
    fn extract_statement(&mut self, statement: &mut Statement) -> Result<(), Self::Error> {
        if let Statement::Block {
            loc: _,
            unchecked: _,
            statements: _,
        } = statement
        {
            self.targets.push(statement.clone())
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
impl ContractDefinitionExtractor {
    pub fn extract_names(contracts: Vec<ContractDefinition>) -> HashSet<String> {
        let mut names = HashSet::new();
        for contract in contracts.iter() {
            if let Some(name) = &contract.name {
                names.insert(name.to_string());
            }
        }

        names
    }
}
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

default_extractor!(UrnaryOpteratorExtractor, Expression);

impl Visitor for UrnaryOpteratorExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::Not(_, _)
            | Expression::BitwiseNot(_, _)
            | Expression::UnaryPlus(_, _)
            | Expression::Negate(_, _)
            | Expression::Power(_, _, _)
            | Expression::Multiply(_, _, _)
            | Expression::Divide(_, _, _)
            | Expression::Modulo(_, _, _)
            | Expression::Add(_, _, _)
            | Expression::Subtract(_, _, _)
            | Expression::ShiftLeft(_, _, _)
            | Expression::ShiftRight(_, _, _)
            | Expression::BitwiseAnd(_, _, _)
            | Expression::BitwiseXor(_, _, _)
            | Expression::BitwiseOr(_, _, _)
            | Expression::And(_, _, _)
            | Expression::Or(_, _, _) => self.targets.push(expr.clone()),
            _ => {}
        }
        Ok(())
    }
}

default_extractor!(DeleteExtractor, Expression);

impl Visitor for DeleteExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        match expr {
            Expression::Delete(..) => self.targets.push(expr.clone()),
            _ => {}
        }
        Ok(())
    }
}

default_extractor!(VariableExtractor, Expression);

impl Visitor for VariableExtractor {
    type Error = ExtractionError;
    fn extract_expr(&mut self, _loc: Loc, expr: &mut Expression) -> Result<(), Self::Error> {
        if let Expression::Variable(_) = expr {
            self.targets.push(expr.clone())
        }
        Ok(())
    }
}

impl VariableExtractor {
    pub fn extract_names(expressions: Vec<Expression>) -> HashSet<String> {
        let mut names = HashSet::new();
        for expr in expressions.iter() {
            if let Expression::Variable(var) = expr {
                names.insert(var.name.to_string());
            }
        }

        names
    }
}

default_extractor!(ParameterExtractor, Parameter);

impl Visitor for ParameterExtractor {
    type Error = ExtractionError;
    fn extract_parameter(&mut self, parameter: &mut Parameter) -> Result<(), Self::Error> {
        self.targets.push(parameter.clone());
        Ok(())
    }
}

impl ParameterExtractor {
    pub fn extract_names(parameters: Vec<Parameter>) -> HashSet<String> {
        let mut names = HashSet::new();
        for parameter in parameters.iter() {
            if let Some(name) = &parameter.name {
                names.insert(name.to_string());
            }
        }

        names
    }
}

default_extractor!(EventExtractor, EventDefinition);

impl Visitor for EventExtractor {
    type Error = ExtractionError;
    fn extract_event(&mut self, _event: &mut EventDefinition) -> Result<(), ExtractionError> {
        self.targets.push(_event.clone());
        Ok(())
    }
}

default_extractor!(YulFunctionCallExtractor, YulFunctionCall);

impl Visitor for YulFunctionCallExtractor {
    type Error = ExtractionError;
    fn extract_yul_function_call(
        &mut self,
        _stmt: &mut YulFunctionCall,
    ) -> Result<(), Self::Error> {
        self.targets.push(_stmt.clone());
        Ok(())
    }
}

default_extractor!(VariableDefinitionExtractor, VariableDefinition);

impl Visitor for VariableDefinitionExtractor {
    type Error = ExtractionError;
    fn extract_var_definition(&mut self, _var: &mut VariableDefinition) -> Result<(), Self::Error> {
        self.targets.push(_var.clone());
        Ok(())
    }
}

default_extractor!(ArraySubscriptExtractor, Expression);

impl Visitor for ArraySubscriptExtractor {
    type Error = ExtractionError;
    fn extract_array_subscript(
        &mut self,
        _loc: Loc,
        _expr_0: &mut Box<Expression>,
        _expr_1: &mut Option<Box<Expression>>,
    ) -> Result<(), Self::Error> {
        self.targets.push(Expression::ArraySubscript(
            _loc,
            _expr_0.clone(),
            _expr_1.clone(),
        ));
        Ok(())
    }
}
