use std::collections::HashSet;

use solang_parser::pt::{Loc, SourceUnit};

use crate::analyzer::extractors::{
    compound::ContractExtractor,
    primitive::{FunctionExtractor, VariableExtractor},
    Extractor,
};

pub fn unused_returns(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut qa_locations: HashSet<Loc> = HashSet::new();
    //Extract all contracts from the source unit.
    let mut contracts = ContractExtractor::extract(source_unit)?;
    //For each contract, extract all functions.
    for contract in contracts.iter_mut() {
        let mut functions = FunctionExtractor::extract(contract)?;
        for function in functions.iter_mut() {
            let mut return_names = HashSet::new();
            //Extract all the return variable names into a HashSet.
            let returns = function.returns.clone();
            for return_type in returns.iter() {
                if let Some(parameter) = &return_type.1 {
                    if let Some(name) = &parameter.name {
                        return_names.insert(name.to_string());
                    }
                }
            }

            //Extract all variable names in the function body, and make sure each of the return names exists.
            if let Some(body) = &mut function.body {
                let body_variables =
                    VariableExtractor::extract_names(VariableExtractor::extract(body)?);
                for return_name in return_names.iter() {
                    if !body_variables.contains(return_name) {
                        qa_locations.insert(function.loc.clone());
                    }
                }
            }
        }
    }

    Ok(qa_locations)
}

#[test]
fn test_import_identifiers() {
    let file_contents_1 = r#"
    contract Contract0 {
        address public owner;
        
        function foo() public returns (uint256 x) {
            uint256 y = 0;
            return y;
        }
    }
    "#;

    let file_contents_2 = r#"
    contract Contract0 {
        address public owner;
        
        function foo() public returns (uint256 x) {
            x = 0;
        }
    }
    "#;

    let mut source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;
    let mut source_unit_2 = solang_parser::parse(file_contents_2, 0).unwrap().0;
    let qa_locations_1 = unused_returns(&mut source_unit_1);
    let qa_locations_2 = unused_returns(&mut source_unit_2);
    assert_eq!(qa_locations_1.unwrap().len(), 1);
    assert_eq!(qa_locations_2.unwrap().len(), 0);
}
