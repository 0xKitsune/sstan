use std::collections::{HashMap, HashSet};

use solang_parser::pt::{Expression, Loc, SourceUnit};

use crate::analyzer::extractors::{
    compound::MutableStorageVariableExtractor,
    primitive::{ContractDefinitionExtractor, FunctionExtractor, VariableExtractor},
    Extractor,
};

pub fn cache_storage_in_memory_optimization(
    source_unit: &mut SourceUnit,
) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();
    let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;
    for contract in contracts.iter_mut() {
        let storage_variables = MutableStorageVariableExtractor::extract(contract)?;
        //Create a hashset of the names of the storage variables
        let mut storage_variable_names = HashSet::new();
        for storage_variable in storage_variables {
            if let Some(identifier) = storage_variable.name {
                storage_variable_names.insert(identifier.name);
            }
        }
        //Get all functions in the contract
        let mut functions = FunctionExtractor::extract(contract)?;
        //Iterate through the functions
        for function in functions.iter_mut() {
            let mut num_storage_references: HashMap<String, u32> = HashMap::new();
            let all_variables = VariableExtractor::extract(function)?;
            for var in all_variables {
                if let Expression::Variable(identifier) = var {
                    if storage_variable_names.contains(&identifier.name) {
                        let current_count =
                            if let Some(count) = num_storage_references.get(&identifier.name) {
                                *count
                            } else {
                                0
                            };
                        num_storage_references.insert(identifier.name, current_count + 1);
                        if current_count + 1 > 1 {
                            optimization_locations.insert(function.loc);
                        }
                    }
                }
            }
        }
    }
    Ok(optimization_locations)
}

#[test]
fn test_public_function_optimization() {
    let file_contents = r#"

    contract Contract0 {
        uint x;
        uint y;
        function shouldCacheInMemory() public {
            uint z = y;
            uint e = y;
        }
        function shouldCacheInMemory() public {
            uint z = x;
            uint e = x;
        }
        function isCachingInMemory() public {
            uint z = x;
            uint a = z;
        }

    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = cache_storage_in_memory_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 2)
}
