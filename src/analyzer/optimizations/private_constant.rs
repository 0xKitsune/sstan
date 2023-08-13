use std::collections::HashSet;

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::analyzer::extractors::{compound::ConstantStorageVariableExtractor, Extractor};

pub fn private_constant_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let constant_target_nodes = ConstantStorageVariableExtractor::extract(source_unit).unwrap();
    for node in constant_target_nodes {
        let mut has_visibility_attribute = false;

        for attr in node.attrs {
            match attr {
                pt::VariableAttribute::Visibility(pt::Visibility::External(_))
                | pt::VariableAttribute::Visibility(pt::Visibility::Public(_))
                | pt::VariableAttribute::Visibility(pt::Visibility::Internal(_)) => {
                    optimization_locations.insert(node.loc);
                    has_visibility_attribute = true;
                }
                pt::VariableAttribute::Visibility(pt::Visibility::Private(_)) => {
                    has_visibility_attribute = true;
                }
                _ => {}
            }
        }
        if !has_visibility_attribute {
            optimization_locations.insert(node.loc);
        }
    }

    Ok(optimization_locations)
}

#[test]
fn test_private_constant_optimization() {
    let file_contents = r#"
    
contract Contract0 {

    uint256 constant public x = 100;
    uint256 constant private y = 100;
    uint256 constant z = 100;


    function addPublicConstant(uint256 a) external pure returns (uint256) {
        return a + x;
    }


    function addPrivateConstant(uint256 a) external pure returns (uint256) {
        return a +x;
    }
}

    "#;
    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;
    let optimization_locations = private_constant_optimization(&mut source_unit);
    assert_eq!(optimization_locations.unwrap().len(), 2)
}
