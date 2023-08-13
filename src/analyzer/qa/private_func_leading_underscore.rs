use std::collections::HashSet;

use solang_parser::pt::{self, FunctionTy, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::compound::ContractPartFunctionExtractor;
use crate::analyzer::extractors::Extractor;

pub fn private_func_leading_underscore(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each qa target identified
    let mut qa_locations: HashSet<Loc> = HashSet::new();
    //Extract the target nodes from the source_unit
    let target_nodes = ContractPartFunctionExtractor::extract(source_unit)?;

    for node in target_nodes {
        if FunctionTy::Function != node.ty {
            continue;
        }

        for attr in node.attributes {
            if let pt::FunctionAttribute::Visibility(v) = attr {
                let fn_def_data = node.name.clone();
                if let Some(fn_data) = fn_def_data {
                    match v {
                        pt::Visibility::Public(_) | pt::Visibility::External(_) => {
                            if fn_data.name.starts_with('_') {
                                qa_locations.insert(fn_data.loc);
                            }
                        }
                        // Private or Internal functions
                        _ => {
                            if !fn_data.name.starts_with('_') {
                                qa_locations.insert(fn_data.loc);
                            }
                        }
                    }
                }
            }
        }
    }

    //Return the identified qa locations
    Ok(qa_locations)
}

#[test]
fn test_private_func_leading_underscore() {
    let file_contents = r#"
    
    contract Contract0 {
        
        function msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgData() private view returns(bytes calldata) {
            return msg.data;
        }

        function msgData() private view returns(bytes calldata) {
            return msg.data;
        }
    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let qa_locations = private_func_leading_underscore(&mut source_unit);
    assert_eq!(qa_locations.unwrap().len(), 2)
}
