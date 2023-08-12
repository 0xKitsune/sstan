use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::extractors::primitive::FunctionCallExtractor;
use crate::analyzer::extractors::{compound::SolidityVerisonExtractor, Extractor};

pub fn string_error_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let solidity_versions = SolidityVerisonExtractor::extract(source_unit)?;
    for version in solidity_versions.into_iter().flatten() {
        if version.minor >= 8 && version.patch >= 4 {
            //Extract the target nodes from the source_unit
            let target_nodes = FunctionCallExtractor::extract(source_unit)?;

            for node in target_nodes {
                if let pt::Expression::FunctionCall(_, function_identifier, func_call_expressions) =
                    node
                {
                    //if the function call identifier is a variable
                    if let pt::Expression::Variable(identifier) = *function_identifier {
                        //if the identifier name is "require"
                        if identifier.name == *"require" {
                            //If the require statement contains strings
                            if let Some(pt::Expression::StringLiteral(vec_string_literal)) =
                                func_call_expressions.last()
                            {
                                optimization_locations.insert(vec_string_literal[0].loc);
                            }
                        }
                    }
                }
            }
        }
    }
    //Return the identified optimization locations
    Ok(optimization_locations)
}
#[test]
fn test_string_error_optimization() {
    //test when base solidiy version is > than 0.8.4
    let file_contents = r#"
     pragma solidity >=0.8.13;
 
     contract Contract0 {
         function addressInternalBalance() public returns (uint256) {
 
             require(true, "some message");
 
             require(true && a==b, "some message");
             require(true && a==b && b==c, "thing");
 
             return address(this).balance;
         }
     }
     "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = string_error_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 3);

    //test when base solidiy version is < than 0.8.4
    let file_contents_1 = r#"
     pragma solidity <= 0.8.3;
 
     contract Contract0 {
         function addressInternalBalance() public returns (uint256) {
 
             require(true, "some message");
 
             require(true && a==b, "some message");
             require(true && a==b && b==c, "thing");
 
             return address(this).balance;
         }
     }
     "#;

    let mut source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;

    let optimization_locations_1 = string_error_optimization(&mut source_unit_1);

    assert_eq!(optimization_locations_1.unwrap().len(), 0);
}
