use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::{primitive::FunctionCallExtractor, Extractor};

//Use assembly to hash instead of keccak256
pub fn solidity_keccak256_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let function_call_nodes = FunctionCallExtractor::extract(source_unit)?;

    //For each target node that was extracted, check for the optimization patterns
    for node in function_call_nodes {
        //Can unwrap because FunctionCall is an expression

        if let pt::Expression::FunctionCall(_, box_expression, _) = node {
            if let pt::Expression::Variable(variable) = *box_expression {
                if variable.name == "keccak256" {
                    optimization_locations.insert(variable.loc);
                }
            }
        }
    }

    //Return the identified optimization locations
    Ok(optimization_locations)
}

#[test]
fn test_template_optimization() {
    let file_contents = r#"
    
contract Contract0 {

    constructor(uint256 a, uint256 b){
        keccak256(abi.encodePacked(a, b));

    }

    function solidityHash(uint256 a, uint256 b) public view {
        //unoptimized
        keccak256(abi.encodePacked(a, b));
    }


    function assemblyHash(uint256 a, uint256 b) public view {
        //optimized
        assembly {
            mstore(0x00, a)
            mstore(0x20, b)
            let hashedVal := keccak256(0x00, 0x40)
        }
    }
}
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = solidity_keccak256_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 2)
}
