use regex::Regex;
use solang_parser::pt::{Loc, SourceUnit};

use std::{collections::HashMap, path::PathBuf};

use crate::{
    create_test_source,
    engine::{EngineError, Outcome, Pushable},
    extractors::{compound::MutableStorageVariableExtractor, Extractor},
};

use super::{QAPattern, QualityAssuranceOutcome, StorageVariableNamespace};
impl QAPattern for StorageVariableNamespace {
    fn find(
        source: HashMap<PathBuf, &mut SourceUnit>,
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
    use crate::engine::Report;
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

        let mock_source = MockSource::new().add_source(file_contents_1);
        let qa_locations = StorageVariableNamespace::find(mock_source.source).unwrap();
        assert_eq!(qa_locations.len(), 2);
        let report: Report = qa_locations.into();
        let mut f = File::options()
            .append(true)
            .open("src/qa/test_report/mock_report.md")?;
        writeln!(&mut f, "{}", report)?;
        Ok(())
    }
}
