use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable, Report};
use crate::utils::get_32_byte_storage_variables;

use super::{PrivateVariablesLeadingUnderscore, QAPattern, QualityAssuranceOutcome};

impl QAPattern for PrivateVariablesLeadingUnderscore {
    fn find(
        source: HashMap<PathBuf, &mut SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome = Outcome::new();

        for (path_buf, source_unit) in source {
            let storage_variables = get_32_byte_storage_variables(source_unit, true, false);

            for (variable_name, variable_def) in storage_variables {
                if let pt::ContractPart::VariableDefinition(variable_def) = variable_def {
                    for attr in &variable_def.attrs {
                        if let pt::VariableAttribute::Visibility(v) = attr {
                            match v {
                                pt::Visibility::Private(_) | pt::Visibility::Internal(_) => {
                                    if !variable_name.starts_with('_') {
                                        outcome.push_or_insert(
                                            path_buf.clone(),
                                            variable_def.loc,
                                            variable_def.clone().to_string(),
                                        )
                                    }
                                }
                                // Public variables
                                _ => {
                                    if variable_name.starts_with('_') {
                                        outcome.push_or_insert(
                                            path_buf.clone(),
                                            variable_def.loc,
                                            variable_def.clone().to_string(),
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::PrivateVariablesLeadingUnderscore(
            outcome,
        ))
    }
}

#[test]
fn test_private_vars_leading_underscore() -> eyre::Result<()> {
    let file_contents = r#"
    
    contract Contract0 {
        address public addr1;
        address public _addr2;
        address private _addr3;
        address private addr4;
        address internal _addr5;
        address internal addr6;
    }
    "#;

    let mut source = HashMap::new();
    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;
    source.insert(PathBuf::new(), &mut source_unit);

    let qa_locations = PrivateVariablesLeadingUnderscore::find(source)?;
    assert_eq!(qa_locations.len(), 3);

    let report: Report = qa_locations.into();

    println!("{report:?}");

    Ok(())
}
