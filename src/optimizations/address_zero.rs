use std::{collections::HashMap, path::PathBuf};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{primitive::EqualityExtractor, Extractor}, utils::MockSource,
};
use solang_parser::{pt::{self, SourceUnit}, helpers::CodeLocation};

use super::{AddressZero, OptimizationOutcome, OptimizationPattern};

pub const ZERO: &str = "0";

impl OptimizationPattern for AddressZero {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let equality_nodes = EqualityExtractor::extract(source_unit)?;

            for node in equality_nodes {
                //We can use unwrap because Target::Equal and Target::NotEqual are expressions

                match node.clone() {
                    pt::Expression::NotEqual(_loc, box_expression, box_expression_1) => {
                        if check_for_address_zero(*box_expression)
                            || check_for_address_zero(*box_expression_1)
                        {
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string());
                        }
                    }
                    pt::Expression::Equal(_loc, box_expression, box_expression_1) => {
                        if check_for_address_zero(*box_expression)
                            || check_for_address_zero(*box_expression_1)
                        {
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(OptimizationOutcome::AddressZero(outcome))
    }
}

fn check_for_address_zero(box_expression: pt::Expression) -> bool {
    //create a boolean to determine if address(0) is present
    let mut address_zero: bool = false;

    //if the first expression is address(0)
    if let pt::Expression::FunctionCall(_, func_call_box_expression, vec_expression) =
        box_expression
    {
        if let pt::Expression::Type(_, pt::Type::Address) = *func_call_box_expression {
            if let pt::Expression::NumberLiteral(_, val, _, _) = &vec_expression[0] {
                if val == ZERO {
                    address_zero = true;
                }
            }
        }
    }

    //return true or false for address_zero
    address_zero
}

#[test]
fn test_address_zero_optimization() {
    let file_contents = r#"
    
    contract Contract0 {

        function ownerNotZero(address _addr) public pure {
            require(_addr == address(0), "zero address");
        }

        function ownerNotZero(address _addr) public pure {
            require(_addr != address(0), "zero address");
        }

        function ownerNotZero1(address _addr) public pure {
            require(address(0) == _addr, "zero address");
        }

        function ownerNotZero1(address _addr) public pure {
            require(address(0) != _addr, "zero address");
        }

     }
    "#;

    let mut mock_source = MockSource::new().add_source("address_zero.sol", file_contents);
    let qa_locations = AddressZero::find(&mut mock_source.source).unwrap();
    assert_eq!(qa_locations.len(), 4)
}
