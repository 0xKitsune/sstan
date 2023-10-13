use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{primitive::EventExtractor, Extractor},
    utils::is_pascal_case,
};

use super::{EventNamePascalCase, QAPattern, QualityAssuranceOutcome};
impl QAPattern for EventNamePascalCase {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let event_definitions = EventExtractor::extract(source_unit)?;
            for event in event_definitions {
                if let Some(name) = &event.name {
                    if !is_pascal_case(&name.name) {
                        outcome.push_or_insert(path_buf.clone(), event.loc, event.to_string());
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::EventNamePascalCase(outcome))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::MockSource;

    #[test]
    fn test_event_name_pascal_case() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract Contract0 {
       event Event();
       event event2();
       event eventThree();
       event EventFour_x();

    }

    "#;

        let mut mock_source =
            MockSource::new().add_source("event_name_pascal_case.sol", file_contents);
        let qa_locations = EventNamePascalCase::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 3);

        Ok(())
    }
}
