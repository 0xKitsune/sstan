use std::collections::HashSet;

use solang_parser::pt::{self, Identifier, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::extractors::primitive::AssignmentExtractor;
use crate::analyzer::extractors::{compound::StorageVariableExtractor, Extractor};
use crate::analyzer::utils::get_32_byte_storage_variables;

pub fn sstore_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Get all storage variables
    let storage_variables = get_32_byte_storage_variables(source_unit, false, true);

    //Extract the target nodes from the source_unit
    let assignment_nodes = AssignmentExtractor::extract(source_unit)?;

    for node in assignment_nodes {
        //We can use unwrap because Target::Assign is an expression
        //if the expression is an Assign
        if let pt::Expression::Assign(loc, box_expression, _) = node {
            //if the first expr in the assign expr is a variable
            if let pt::Expression::Variable(identifier) = *box_expression {
                if storage_variables.contains_key(&identifier.name) {
                    optimization_locations.insert(loc);
                }
            }
        }
    }
    //Return the identified optimization locations
    Ok(optimization_locations)
}
#[test]
fn test_sstore_optimization() {
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
    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = sstore_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 3);
}
