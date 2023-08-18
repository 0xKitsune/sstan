use std::collections::HashSet;

use solang_parser::pt::{Expression, Loc, SourceUnit};

use crate::analyzer::extractors::{
    compound::MutableStorageVariableExtractor,
    primitive::{ContractDefinitionExtractor, ForExtractor, VariableExtractor},
    Extractor,
};

pub fn read_storage_in_for_loop_optimization(
    source_unit: &mut SourceUnit,
) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;

    for contract in contracts.iter_mut() {
        let storage_variables = MutableStorageVariableExtractor::extract(contract)?;
        let mut variable_names = HashSet::new();
        for storage_variable in storage_variables {
            if let Some(identifier) = storage_variable.name {
                variable_names.insert(identifier.name);
            }
        }

        let mut for_loops = ForExtractor::extract(contract)?;
        for for_loop in for_loops.iter_mut() {
            let variables = VariableExtractor::extract(for_loop)?;
            for variable in variables {
                if let Expression::Variable(identifier) = variable {
                    if variable_names.contains(&identifier.name) {
                        optimization_locations.insert(identifier.loc);
                    }
                }
            }
        }
    }

    Ok(optimization_locations)
}

#[test]
fn test_read_from_storage_in_for_loop() {
    let file_contents = r#"

    contract Contract0 {
        uint x;
        uint y;
        function shouldCacheInMemory() public {
            for (uint i = 0; i < 10; i++) {
                uint z = y;
            }
        }
        function shouldCacheInMemory() public {
            for (uint i = 0; i < 10; i++) {
                uint z = x;
            }
        }
        function isCachingInMemory() public {
            uint z = x;
            uint a = z;
        }

    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = read_storage_in_for_loop_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 2)
}
