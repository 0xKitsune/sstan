use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::PathBuf;

use solang_parser::pt::{self, Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable, Snippet};
use crate::extractors::compound::{
    ContractExtractor, MutableStorageVariableExtractor, YulShiftExtractor,
};
use crate::extractors::primitive::{FunctionExtractor, VariableExtractor};
use crate::extractors::{
    primitive::{AssignmentExtractor, UrnaryOpteratorExtractor},
    Extractor,
};
use crate::report::ReportSectionFragment;
use crate::utils::MockSource;
use std::io::Write;

use super::{
    DivideBeforeMultiply, IncorrectShiftMath, UninitializedStorageVariable, VulnerabilityOutcome,
    VulnerabilityPattern,
};

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

        Ok(VulnerabilityOutcome::DivideBeforeMultiply(
            vulnerability_locations,
        ))
    }
}

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
    let mut mock_source = MockSource::new().add_source("incorrect_shift_math.sol", file_contents);
    let vuln_locations = DivideBeforeMultiply::find(&mut mock_source.source)?;
    assert_eq!(vuln_locations.len(), 2);

    let report: Option<ReportSectionFragment> = vuln_locations.into();
    if let Some(report) = report {
        let mut f = File::options()
            .append(true)
            .open("src/report/mocks/vulnerability_report_sections.md")?;
        writeln!(&mut f, "{}", &String::from(report))?;
    }

    Ok(())
}
