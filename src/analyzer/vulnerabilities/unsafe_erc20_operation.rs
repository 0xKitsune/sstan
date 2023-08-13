use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::primitive::MemberAccessExtractor;
use crate::analyzer::extractors::Extractor;

pub fn unsafe_erc20_operation_vulnerability(
    source_unit: &mut SourceUnit,
) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each vulnerability target identified
    let mut vulnerability_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let member_access_nodes = MemberAccessExtractor::extract(source_unit)?;

    //For each target node that was extracted, check for the vulnerability patterns

    for node in member_access_nodes {
        //We can use unwrap because Target::MemberAccess is an expression

        if let pt::Expression::MemberAccess(loc, _, identifier) = node {
            if identifier.name == "transfer"
                || identifier.name == "transferFrom"
                || identifier.name == "approve"
            {
                vulnerability_locations.insert(loc);
            }
        }
    }

    //Return the identified vulnerability locations
    Ok(vulnerability_locations)
}

#[test]
fn test_unsafe_erc20_operation_vulnerability() {
    let file_contents = r#"
    
    contract Contract0 {
        IERC20 e;

        constructor(){
            e = IERC20(0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984);
        }

        function unsafe_erc20_operations() public {
            e.approve(address(0), 200);
            e.transfer(address(0), 100);
            e.transferFrom(address(0), 100);
        }

    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let vulnerability_locations = unsafe_erc20_operation_vulnerability(&mut source_unit);
    assert_eq!(vulnerability_locations.unwrap().len(), 3)
}
