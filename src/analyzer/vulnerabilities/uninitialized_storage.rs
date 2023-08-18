use std::collections::{HashMap, HashSet};

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::analyzer::extractors::{
    compound::{ContractExtractor, MutableStorageVariableExtractor},
    primitive::{FunctionExtractor, VariableExtractor},
    Extractor,
};

pub fn uninitialized_storage_variable(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut vulnerability_locations: HashSet<Loc> = HashSet::new();
    let mut contracts = ContractExtractor::extract(source_unit)?;
    for contract in contracts.iter_mut() {
        let storage_variables = MutableStorageVariableExtractor::extract(contract)?;
        let mut storage_variable_names: HashMap<String, Loc> = HashMap::new();
        //Accumulate a hashmap of storage variables and their locations
        for storage_variable in storage_variables {
            if let Some(ident) = storage_variable.name {
                storage_variable_names.insert(ident.name, storage_variable.loc);
            }
        }

        let mut functions = FunctionExtractor::extract(contract)?;
        let mut variable_names = HashSet::new();
        //Get all variable names in every function.
        for function in functions.iter_mut() {
            let variables = VariableExtractor::extract(function)?;

            for variable in variables {
                if let pt::Expression::Variable(ident) = variable {
                    variable_names.insert(ident.name);
                }
            }
        }
        //For each storage variable make sure it is used in the contract.
        for storage_variable_name in storage_variable_names.keys() {
            if !variable_names.contains(storage_variable_name) {
                vulnerability_locations
                    .insert(*storage_variable_names.get(storage_variable_name).unwrap());
            }
        }
    }

    Ok(vulnerability_locations)
}
#[test]
fn test_incorrect_shift_math() {
    let file_contents = r#"
    
    contract Contract0 {
        address owner;
        address owner2;
        address owner3;

        constructor() public {
            owner = msg.sender;
        }
        
    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let vulnerability_locations = uninitialized_storage_variable(&mut source_unit);
    assert_eq!(vulnerability_locations.unwrap().len(), 2)
}