use regex::Regex;
use solang_parser::pt::{Loc, SourceUnit};

use std::{collections::HashMap, path::PathBuf};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{compound::MutableStorageVariableExtractor, Extractor},
};

use super::{QAPattern, QualityAssuranceOutcome, StorageVariableNamespace};
impl QAPattern for StorageVariableNamespace {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let re: Regex = Regex::new(r"\b[A-Z][A-Z0-9_]*\b")?;

            let variables = MutableStorageVariableExtractor::extract(source_unit)?;
            for variable in variables.iter() {
                if let Some(name) = &variable.name {
                    if re.is_match(name.name.as_str()) {
                        outcome.push_or_insert(
                            path_buf.clone(),
                            variable.loc,
                            variable.to_string(),
                        );
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::StorageVariableNamespace(outcome))
    }
}
#[cfg(test)]
mod test {

    use crate::report::ReportSectionFragment;
    use crate::utils::MockSource;

    use super::*;
    use std::fs::File;
    use std::io::Write;
    #[test]
    fn test_storage_variable_namespace() -> eyre::Result<()> {
        let file_contents_1 = r#"
    contract Contract {

        address IS_NOT_FINE;
        address isFine;
        address alsoIsFine;
        address ALSO_IS_BAD;
        constructor() {

        }
        
    }
    "#;

        let mut mock_source =
            MockSource::new().add_source("storage_var_namespace.sol", file_contents_1);
        let qa_locations = StorageVariableNamespace::find(&mut mock_source.source)?;
        assert_eq!(qa_locations.len(), 2);
        let report: Option<ReportSectionFragment> = qa_locations.into();
        if let Some(report) = report {
            let mut f = File::options().append(true).open("mocks/qa_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }
        Ok(())
    }
}
