use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::extractors::primitive::MemberAccessExtractor;
use crate::analyzer::extractors::Extractor;

pub const BALANCE: &str = "balance";

//Use selfbalance() instead of address(this).balance()
pub fn address_balance_optimization(source_unit: &mut SourceUnit) -> HashSet<Loc> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let member_access_nodes =
        MemberAccessExtractor::extract(source_unit).expect("TODO: handle this error");

    for node in member_access_nodes {
        //We can use unwrap because Target::MemberAccess is an expression

        if let pt::Expression::MemberAccess(loc, box_expression, identifier) = node {
            if let pt::Expression::FunctionCall(_, box_expression, _) = *box_expression {
                if let pt::Expression::Type(_, pt::Type::Address) = *box_expression {
                    //if address(0x...).balance or address(this).balance
                    if identifier.name == *BALANCE {
                        optimization_locations.insert(loc);
                    }
                }
            }
        }
    }

    optimization_locations
}

#[test]
fn test_address_balance_optimization() {
    let file_contents = r#"
    
contract Contract0 {
    function addressInternalBalance(){
        uint256 bal = address(this).balance;
        bal++;
    }

    function addressExternalBalance(address addr) public {
        uint256 bal = address(addr).balance;
        bal++;
    }
}

    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = address_balance_optimization(&mut source_unit);

    assert_eq!(optimization_locations.len(), 2)
}
