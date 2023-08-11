use solang_parser::pt::{self, Expression, Loc};
use solang_parser::{self, pt::SourceUnit};
use std::collections::HashSet;

use crate::analyzer::extractors::primitive::{BlockExtractor, IncrementorExtractor};
use crate::analyzer::extractors::Extractor;
pub fn increment_decrement_optimization(
    source_unit: &mut SourceUnit,
) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Get all increment/decrement expressions in unchecked blocks so that the analyzer does not mark these as optimization targets
    let block_nodes = BlockExtractor::extract(source_unit)?;
    let mut unchecked_locations: HashSet<Loc> = HashSet::new();
    for node in block_nodes {
        if let pt::Statement::Block {
            loc: _,
            unchecked,
            mut statements,
        } = node
        {
            if unchecked {
                for statement in statements.iter_mut() {
                    unchecked_locations.extend(extract_pre_increment_pre_decrement(statement));
                }
            }
        }
    }

    //Get all increment / decrement locations
    let locations = extract_increment_decrement(source_unit);

    for loc in locations {
        if !unchecked_locations.contains(&loc) {
            optimization_locations.insert(loc);
        }
    }

    Ok(optimization_locations)
}

pub fn extract_increment_decrement(node: &mut SourceUnit) -> HashSet<Loc> {
    let mut locations: HashSet<Loc> = HashSet::new();

    let target_nodes = IncrementorExtractor::extract(node).unwrap();

    for node in target_nodes {
        match node {
            Expression::PreIncrement(loc, _) => {
                locations.insert(loc);
            }
            Expression::PreDecrement(loc, _) => {
                locations.insert(loc);
            }
            Expression::PostIncrement(loc, _) => {
                locations.insert(loc);
            }
            Expression::PostDecrement(loc, _) => {
                locations.insert(loc);
            }

            _ => {}
        }
    }
    locations
}

pub fn extract_pre_increment_pre_decrement(node: &mut pt::Statement) -> HashSet<Loc> {
    let mut locations: HashSet<Loc> = HashSet::new();

    let target_nodes = IncrementorExtractor::extract(node).unwrap();

    for node in target_nodes {
        match node {
            Expression::PreIncrement(loc, _) => {
                locations.insert(loc);
            }
            Expression::PreDecrement(loc, _) => {
                locations.insert(loc);
            }

            _ => {}
        }
    }
    locations
}

#[test]
fn test_increment_optimization() {
    let file_contents = r#"
  
    contract Contract0 {
        function iPlusPlus(){
            uint256  i = 0;
            i++;
        }
    
        function plusPlusI() public {
            uint256  i = 0;
            ++i;
    
        for (uint256 j = 0; j < numNfts; ) {
                bytes32 hash = keccak256(abi.encode(ORDER_ITEM_HASH, nfts[i].collection, _tokensHash(nfts[i].tokens)));
                hashes[i] = hash;
                unchecked {
                  ++j;
                }
              }
    
            unchecked{
                i++;
            }
        }
        
    }
    
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = increment_decrement_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 3)
}
