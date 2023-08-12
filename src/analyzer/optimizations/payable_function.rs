use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::primitive::FunctionExtractor;
use crate::analyzer::extractors::{primitive::ContractDefinitionExtractor, Extractor};

pub fn payable_function_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let mut contract_definition_nodes = ContractDefinitionExtractor::extract(source_unit)?;

    for contract_definition_node in contract_definition_nodes.iter_mut() {
        let target_nodes = FunctionExtractor::extract(contract_definition_node)?;

        for node in target_nodes {
            //We can use unwrap because Target::FunctionDefinition is a contract_part

            //if there is function body
            if node.body.is_some() && !node.attributes.is_empty() {
                let mut payable = false;
                let mut public_or_external = false;

                for attr in node.attributes {
                    match attr {
                        // Visibility
                        pt::FunctionAttribute::Visibility(visibility) => match visibility {
                            pt::Visibility::External(_) => {
                                public_or_external = true;
                            }
                            pt::Visibility::Public(_) => {
                                public_or_external = true;
                            }
                            _ => {}
                        },
                        pt::FunctionAttribute::Mutability(pt::Mutability::Payable(_)) => {
                            payable = true;
                        }
                        _ => {}
                    }
                }

                //if the function is public or external, and it is not marked as payable
                if public_or_external && !payable {
                    //insert the loc of the function definition into optimization locations
                    optimization_locations.insert(node.loc);
                }
            }
        }
    }

    Ok(optimization_locations)
}

#[test]
fn test_payable_function_optimization() {
    let file_contents = r#"
    

    contract Contract0 {

        function div2(uint256 a, uint256 b) public pure {
            
        }

        function mul2(uint256 a, uint256 b) external view {
            
        }
    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = payable_function_optimization(&mut source_unit);
    assert_eq!(optimization_locations.unwrap().len(), 2)
}
