use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use solang_parser::pt::{CodeLocation, Expression, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        compound::{MutableStorageVariableExtractor, WriteFunctionExtractor},
        primitive::{ContractDefinitionExtractor, DeleteExtractor, VariableExtractor},
        Extractor,
    },
};

use super::{CacheStorageInMemory, OptimizationOutcome, OptimizationPattern};

impl OptimizationPattern for CacheStorageInMemory {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
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
                let mut functions = WriteFunctionExtractor::extract(contract)?;

                //Iterate through the functions
                for function in functions.iter_mut() {
                    let mut num_storage_references: HashMap<String, u32> = HashMap::new();
                    let all_variables = VariableExtractor::extract(function)?;
                    //Get all the delete expressions in the function
                    let mut delete_expressions = DeleteExtractor::extract(function)?;
                    let mut deleted_storage_variables: HashSet<String> = HashSet::new();
                    //Accumulate any deleted storage variable names
                    for delete_expression in delete_expressions.iter_mut() {
                        let variables = VariableExtractor::extract(delete_expression)?;
                        for var in variables.iter() {
                            if let Expression::Variable(identifier) = var.clone() {
                                deleted_storage_variables.insert(identifier.name);
                            }
                        }
                    }
                    for var in all_variables {
                        if let Expression::Variable(identifier) = var.clone() {
                            if storage_variable_names.contains(&identifier.name) {
                                let current_count = if let Some(count) =
                                    num_storage_references.get(&identifier.name)
                                {
                                    *count
                                } else {
                                    0
                                };
                                num_storage_references
                                    .insert(identifier.name.clone(), current_count + 1);
                                if (current_count + 1 > 1
                                    && !deleted_storage_variables.contains(&identifier.name))
                                    || (deleted_storage_variables.contains(&identifier.name)
                                        && current_count + 1 > 2)
                                {
                                    outcome.push_or_insert(
                                        path_buf.clone(),
                                        var.loc(),
                                        var.to_string(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(OptimizationOutcome::CacheStorageInMemory(outcome))
    }
}
mod test {

    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;

    #[test]
    fn test_public_function_optimization() -> eyre::Result<()> {
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

        let mut source = MockSource::new().add_source("cache_storage_in_memory.sol", file_contents);
        let optimization_locations = CacheStorageInMemory::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 2);

        Ok(())
    }
}
