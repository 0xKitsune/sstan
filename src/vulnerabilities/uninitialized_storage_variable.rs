use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable, Snippet};
use crate::extractors::compound::{ContractExtractor, MutableStorageVariableExtractor};
use crate::extractors::primitive::{FunctionExtractor, VariableExtractor};
use crate::extractors::Extractor;

use super::{UninitializedStorageVariable, VulnerabilityOutcome, VulnerabilityPattern};

impl VulnerabilityPattern for UninitializedStorageVariable {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<VulnerabilityOutcome, EngineError> {
        let mut vulnerability_locations = Outcome::new();

        for (path_buf, source_unit) in source {
            let mut contracts = ContractExtractor::extract(source_unit)?;
            for contract in contracts.iter_mut() {
                let storage_variables = MutableStorageVariableExtractor::extract(contract)?;
                let mut storage_variable_names: HashMap<String, (Loc, Snippet)> = HashMap::new();
                //Accumulate a hashmap of storage variables and their locations
                for storage_variable in storage_variables {
                    if let Some(ident) = &storage_variable.name {
                        storage_variable_names.insert(
                            ident.name.to_string(),
                            (storage_variable.loc, storage_variable.to_string()),
                        );
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
                        let loc = storage_variable_names.get(storage_variable_name).unwrap().0;
                        let snippet = storage_variable_names
                            .get(storage_variable_name)
                            .unwrap() // we can unwrap here since we verified above
                            .1
                            .clone();

                        vulnerability_locations.push_or_insert(path_buf.clone(), loc, snippet);
                    }
                }
            }
        }

        Ok(VulnerabilityOutcome::UninitializedStorageVariable(
            vulnerability_locations,
        ))
    }
}
mod test {
    use crate::utils::MockSource;

    use super::*;
    #[test]
    fn test_uninitialized_storage_variable() -> eyre::Result<()> {
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
        let mut mock_source =
            MockSource::new().add_source("uninitialized_storage_variable.sol", file_contents);
        let vuln_locations = UninitializedStorageVariable::find(&mut mock_source.source)?;
        assert_eq!(vuln_locations.len(), 2);

        Ok(())
    }
}
