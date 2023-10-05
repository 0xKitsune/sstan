use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self, CodeLocation};
use solang_parser::{self, pt::SourceUnit};

use super::{OptimizationOutcome, OptimizationPattern, Sstore};
use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::primitive::AssignmentExtractor;
use crate::extractors::Extractor;
use crate::utils::get_32_byte_storage_variables;

impl OptimizationPattern for Sstore {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            //Get all storage variables
            let storage_variables = get_32_byte_storage_variables(source_unit, false, true);

            //Extract the target nodes from the source_unit
            let assignment_nodes = AssignmentExtractor::extract(source_unit)?;

            for node in assignment_nodes {
                //We can use unwrap because Target::Assign is an expression
                //if the expression is an Assign
                if let pt::Expression::Assign(_loc, box_expression, _) = node.clone() {
                    //if the first expr in the assign expr is a variable
                    if let pt::Expression::Variable(identifier) = *box_expression {
                        if storage_variables.contains_key(&identifier.name) {
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string());
                        }
                    }
                }
            }
        }
        //Return the identified optimization locations
        Ok(OptimizationOutcome::Sstore(outcome))
    }
}

mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_sstore_optimization() -> eyre::Result<()> {
        let file_contents = r#"
    
    pragma solidity >= 0.8.0;
    contract Contract {
       
        uint256 thing = 100;
        address someAddress = address(0);
        bytes someBytes;
        
    
       
        function testFunction() public {
             thing = 1+2;
             someAddress = msg.sender;
             someBytes = bytes(0);


        }
    }
 
    "#;
        let mut source = MockSource::new().add_source("sstore.sol", file_contents);
        let optimization_locations = Sstore::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 3);

        Ok(())
    }
}
