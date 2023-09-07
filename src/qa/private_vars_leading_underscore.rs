use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::utils::get_32_byte_storage_variables;

use super::{PrivateVariablesLeadingUnderscore, QAPattern, QualityAssuranceOutcome};

impl QAPattern for PrivateVariablesLeadingUnderscore {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use crate::qa::{PrivateVariablesLeadingUnderscore, QAPattern};
    use crate::report::ReportSectionFragment;
    use crate::utils::MockSource;

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
        let mut mock_source = MockSource::new().add_source("private_vars.sol", file_contents);
        let qa_locations = PrivateVariablesLeadingUnderscore::find(&mut mock_source.source)?;
        assert_eq!(qa_locations.len(), 3);

        let report: Option<ReportSectionFragment> = qa_locations.into();
        if let Some(report) = report {
            let mut f = File::options()
                .append(true)
                .open("qa_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }

        Ok(())
    }
}
