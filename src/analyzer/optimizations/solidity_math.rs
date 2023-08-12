use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::primitive::UrnaryOpteratorExtractor;
use crate::analyzer::extractors::Extractor;

pub fn solidity_math_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let urnary_operator_nodes = UrnaryOpteratorExtractor::extract(source_unit)?;

    //For each target node that was extracted, check for the optimization patterns
    for node in urnary_operator_nodes {
        //Can unwrap because all targets are expressions
        match node {
            pt::Expression::Add(loc, _, _) => {
                optimization_locations.insert(loc);
            }
            pt::Expression::Subtract(loc, _, _) => {
                optimization_locations.insert(loc);
            }
            pt::Expression::Multiply(loc, _, _) => {
                optimization_locations.insert(loc);
            }
            pt::Expression::Divide(loc, _, _) => {
                optimization_locations.insert(loc);
            }

            _ => {}
        }
    }

    //Return the identified optimization locations
    Ok(optimization_locations)
}

#[test]
fn test_analyze_for_math_optimization() {
    let file_contents = r#"
    

    contract Contract0 {

        //addition in Solidity
        function addTest(uint256 a, uint256 b) public pure {
            uint256 c = a + b;
        }

        //addition in assembly
        function addAssemblyTest(uint256 a, uint256 b) public pure {
            assembly {
                let c := add(a, b)
    
                if lt(c, a) {
                    mstore(0x00, "overflow")
                    revert(0x00, 0x20)
                }
            }
        }

        //subtraction in Solidity
        function subTest(uint256 a, uint256 b) public pure {
            uint256 c = a - b;
        }
    }
    
    contract Contract3 {
        //subtraction in assembly
        function subAssemblyTest(uint256 a, uint256 b) public pure {
            assembly {
                let c := sub(a, b)
    
                if gt(c, a) {
                    mstore(0x00, "underflow")
                    revert(0x00, 0x20)
                }
            }
        }

        //multiplication in Solidity
        function mulTest(uint256 a, uint256 b) public pure {
            uint256 c = a * b;
        }
        //multiplication in assembly
        function mulAssemblyTest(uint256 a, uint256 b) public pure {
            assembly {
                let c := mul(a, b)
    
                if lt(c, a) {
                    mstore(0x00, "overflow")
                    revert(0x00, 0x20)
                }
            }
        }

        //division in Solidity
        function divTest(uint256 a, uint256 b) public pure {
            uint256 c = a * b;
        }
        
        function divAssemblyTest(uint256 a, uint256 b) public pure {
            assembly {
                let c := div(a, b)
    
                if gt(c, a) {
                    mstore(0x00, "underflow")
                    revert(0x00, 0x20)
                }
            }
        }
    }

    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = solidity_math_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 4)
}
