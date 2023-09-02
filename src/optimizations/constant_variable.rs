use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self, CodeLocation};
use solang_parser::{self, pt::SourceUnit};

use super::{ConstantVariable, OptimizationOutcome, OptimizationPattern};
use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::primitive::IncrementorExtractor;
use crate::extractors::{primitive::AssignmentExtractor, Extractor};
use crate::utils::get_32_byte_storage_variables;

impl OptimizationPattern for ConstantVariable {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let mut storage_variables = get_32_byte_storage_variables(source_unit, true, false);

            let assignment_nodes = AssignmentExtractor::extract(source_unit)?;
            let incrementor_nodes = IncrementorExtractor::extract(source_unit)?;
            let nodes = assignment_nodes
                .into_iter()
                .chain(incrementor_nodes.into_iter())
                .collect::<Vec<pt::Expression>>();

            for node in nodes {
                match node {
                    pt::Expression::Assign(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::PreIncrement(_, box_expression) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::PostIncrement(_, box_expression) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::PreDecrement(_, box_expression) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::PostDecrement(_, box_expression) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }

                    pt::Expression::AssignAdd(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignAnd(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignDivide(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignModulo(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignMultiply(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignOr(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignShiftLeft(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignShiftRight(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignSubtract(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignXor(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if storage_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                storage_variables.remove(&identifier.name);
                            }
                        }
                    }
                    _ => {}
                }
            }

            //if the variable is not been reassigned, add it to the optimization locations
            for variable in storage_variables {
                outcome.push_or_insert(path_buf.clone(), variable.1.loc(), variable.1.to_string());
            }
        }

        Ok(OptimizationOutcome::ConstantVariable(outcome))
    }
}
mod test {
    use crate::{
        optimizations::{ConstantVariable, OptimizationPattern},
        utils::MockSource,
    };

    #[test]
    fn test_constant_variable_optimization() -> eyre::Result<()> {
        let file_contents = r#"
    
    pragma solidity >= 0.8.0;
    contract Contract {


        uint256 firstUint256 = 0;
        uint256 secondUint256 = 100;
        uint256 immutable thirdUint256 = 100;
        uint256 fourthUint256 = 100;
        uint256 constant fifthUint256 = 1000000;

       
        function testFunction() public {
            firstUint256 = 10;
            secondUint256 = someVal;
        }
    }
 
    "#;

        let mut source = MockSource::new().add_source("constant_variable.sol", file_contents);
        let optimization_locations = ConstantVariable::find(&mut source.source)?;
        assert_eq!(optimization_locations.len(), 2);

        Ok(())
    }
}
