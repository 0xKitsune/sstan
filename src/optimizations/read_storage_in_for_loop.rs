use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use solang_parser::pt::{CodeLocation, Expression, SourceUnit};

use crate::extractors::{
    compound::MutableStorageVariableExtractor,
    primitive::{ContractDefinitionExtractor, ForExtractor, VariableExtractor, ArraySubscriptExtractor},
    Extractor,
};

use super::{OptimizationOutcome, OptimizationPattern, ReadStorageInForLoop};
use crate::engine::{EngineError, Outcome, Pushable};

impl OptimizationPattern for ReadStorageInForLoop {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;

            for contract in contracts.iter_mut() {
                let storage_variables = MutableStorageVariableExtractor::extract(contract)?;
                let mut variable_names = HashSet::new();
                for storage_variable in storage_variables {
                    if let Some(identifier) = storage_variable.name {
                        variable_names.insert(identifier.name);
                    }
                }
                //Extract all ArraySubscript variable names
                let mut array_subscripts = ArraySubscriptExtractor::extract(contract)?;
                let mut array_variable_names = HashSet::new();
                for array_subscript in array_subscripts.iter_mut() {
                    let array_variables = VariableExtractor::extract(array_subscript)?;
                    //Insert all Array subscript var names into the HashSet
                    for array_variable in array_variables {
                        if let Expression::Variable(identifier) = array_variable {
                            array_variable_names.insert(identifier.name);
                        }
                    }
                }
                let mut for_loops = ForExtractor::extract(contract)?;
                for for_loop in for_loops.iter_mut() {
                    let variables = VariableExtractor::extract(for_loop)?;
                    for variable in variables {
                        if let Expression::Variable(identifier) = variable {
                            if variable_names.contains(&identifier.name) {
                                //Check if the variable is in an array subscript
                                if !array_variable_names.contains(&identifier.name) {
                                    outcome.push_or_insert(
                                        path_buf.clone(),
                                        for_loop.loc(),
                                        format!("{}", for_loop),
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(OptimizationOutcome::ReadStorageInForLoop(outcome))
    }
}
mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;

    #[test]
    fn test_read_from_storage_in_for_loop() -> eyre::Result<()> {
        let file_contents = r#"

    contract Contract0 {
        uint x;
        uint y;
        struct ArrayStruct {
            uint256[] arr;
        }
        function shouldCacheInMemory() public {
            for (uint i = 0; i < 10; i++) {
                uint z = y;
            }
        }

        function isReferencingArray(ArrayStruct memory arr) public {
            for (uint i = 0; i < arr.length; i++) {
                uint z = arr[i];
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

        let mut source =
            MockSource::new().add_source("read_from_storage_in_for_loop.sol", file_contents);
        let optimization_locations = ReadStorageInForLoop::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 2);

        Ok(())
    }
}
