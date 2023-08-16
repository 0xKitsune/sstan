use std::collections::HashSet;

use solang_parser::pt::{Expression, Loc, SourceUnit};

use crate::analyzer::extractors::{compound::StorageVariableExtractor, Extractor};

pub fn addresses_hardcoded(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut qa_locations: HashSet<Loc> = HashSet::new();
    let variables = StorageVariableExtractor::extract(source_unit)?;
    for variable in variables.iter() {
        if let Some(initializer) = variable.initializer.clone() {
            if let Expression::HexNumberLiteral(_, _, _) = initializer {
                qa_locations.insert(variable.loc);
            }
        }
    }
    Ok(qa_locations)
}

#[test]
fn test_hardcoded_address() {
    let file_contents_1 = r#"
    contract Contract0 {
        address public owner;
        address hardcoded = 0xCF93bC53DA6D3543ec2B39EB9Fb3eb1472502afA;
        constructor(address _owner) {
            owner = _owner;
        
        }
    }
    "#;

    let mut source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;
    let qa_locations_1 = addresses_hardcoded(&mut source_unit_1);
    assert_eq!(qa_locations_1.unwrap().len(), 1);
}
