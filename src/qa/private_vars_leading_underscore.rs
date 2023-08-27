use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
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

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;

    use crate::report::ReportSectionFragment;
    use crate::utils::MockSource;
    use crate::{
        cleanup_test_source, create_test_source,
        engine::Report,
        qa::{PrivateVariablesLeadingUnderscore, QAPattern},
    };

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
        let mut mock_source = MockSource::new().add_source(file_contents);
        let source = std::mem::take(&mut mock_source.source);
        let qa_locations = PrivateVariablesLeadingUnderscore::find(source)?;
        assert_eq!(qa_locations.len(), 3);

        let report: Option<ReportSectionFragment> = qa_locations.into();
        if let Some(report) = report {
            let mut f = File::options()
                .append(true)
                .open("src/report/mock_report.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }

        Ok(())
    }
}
