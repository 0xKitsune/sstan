use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::extractors::primitive::{ForExtractor, MemberAccessExtractor};
use crate::analyzer::extractors::Extractor;

pub const LENGTH: &str = "length";

pub fn cache_array_length_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit

    let for_nodes = ForExtractor::extract(source_unit)?;

    //For each target node that was extracted, check for the optimization patterns
    for node in for_nodes {
        if let pt::Statement::For(_, _, Some(box_expression), _, _) = node {
            //get all of the .length in the for loop definition
            let member_access_nodes = MemberAccessExtractor::extract(&mut box_expression)?;

            for node in member_access_nodes {
                //Can unwrap because Target::MemberAccess will always be an expression
                if let pt::Expression::MemberAccess(loc, _, identifier) = node {
                    if identifier.name == LENGTH {
                        optimization_locations.insert(loc);
                    }
                }
            }
        }
    }

    //Return the identified optimization locations
    optimization_locations
}

#[test]
fn test_cache_array_length_optimization() {
    let file_contents = r#"

    contract Contract1 {


        //loop with i++
        function memoryArray(uint256[] memory arr) public {
            uint256 j;
            for (uint256 i; i < arr.length; i++) {
                j = arr[i] + 10;
            }
        }
    
        //loop with i++
        function calldataArray(uint256[] calldata arr) public {
            uint256 j;
            for (uint256 i; i < 100; i++) {
                j = arr[i] + arr.length;
            }
        }
    
        //loop with i++
        function memoryArray(uint256[] memory arr) public {
            uint256 j;
            for (uint256 i;  arr.length<1000; i++) {
                arr[i] = 10;
            }
        }
    
        }    
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = cache_array_length_optimization(source_unit);

    assert_eq!(optimization_locations.len(), 2)
}
