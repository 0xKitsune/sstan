use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{self, Expression, Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        compound::ContractExtractor,
        primitive::{EventExtractor, FunctionCallExtractor, PlainImportExtractor},
        Extractor,
    },
    utils::{is_camel_case, is_pascal_case},
};

use super::{
    EventNamePascalCase, ImportIdentifiers, OneContractPerFile, QAPattern, QualityAssuranceOutcome,
    RemoveConsole,
};
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

        Ok(QualityAssuranceOutcome::ImportIdentifiers(outcome))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::{report::ReportSectionFragment, utils::MockSource};

    use super::*;
    #[test]
    fn test_import_identifiers() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract Contract0 {
       event Event1();
       event event2();
       event eventThree();
       event EventFour_x();

    }

    "#;

        let mut mock_source =
            MockSource::new().add_source("event_name_pascal_case.sol", file_contents);
        let qa_locations = ImportIdentifiers::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 3);
        let report: Option<ReportSectionFragment> = qa_locations.into();
        if let Some(report) = report {
            let mut f = File::options().append(true).open("qa_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }
        Ok(())
    }
}
