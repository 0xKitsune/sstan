use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::compound::ContractPartFunctionExtractor;
use crate::analyzer::extractors::Extractor;

// Constructor must be placed before any other function
pub fn constructor_order(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each qa target identified
    let mut qa_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let target_nodes = ContractPartFunctionExtractor::extract(source_unit)?;

    let mut fn_counter: u8 = 0; // up to 256 function definitions before reaching the constructor function

    //For each target node that was extracted, check for the qa patterns
    for node in target_nodes {
        match node.ty {
            pt::FunctionTy::Constructor => {
                if fn_counter > 0 {
                    qa_locations.insert(node.loc);
                    break;
                }
            }
            // Modifiers must be placed before constructor
            pt::FunctionTy::Modifier => continue,
            _ => {
                fn_counter += 1;
            }
        }
    }

    //Return the identified qa locations
    Ok(qa_locations)
}

#[test]
fn test_constructor_order_qa() {
    let test_contracts = vec![
        r#"
    contract Contract0 {
        address public owner;
        function test() public {
            owner = address(0);
        }
        constructor() {
            owner = address(1);
        }
    }
    "#,
        r#"
    contract Contract0 {
        address public owner;
        receive() external payable {}
        constructor() {
            owner = address(1);
        }
    }
    "#,
        r#"
    contract Contract0 {
        address public owner;
        modifier onlyOwner {
            require(
            msg.sender == owner,
            "Only owner can call this function."
            );
            _;
        }
        constructor() {
            owner = address(1);
        }
    }
    "#,
        r#"
    contract Contract0 {
        address public owner;
        function test() public {
            owner = address(0);
        }
    }
    "#,
    ];

    let assertions = vec![1, 1, 0, 0];

    assert_eq!(test_contracts.len(), assertions.len());

    if !assertions.is_empty() {
        for i in 0..assertions.len() - 1 {
            let mut source_unit = solang_parser::parse(test_contracts[i], 0).unwrap().0;

            let qa_locations = constructor_order(&mut source_unit);
            assert_eq!(qa_locations.unwrap().len(), assertions[i]);
        }
    }
}
