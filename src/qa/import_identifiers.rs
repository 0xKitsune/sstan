use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{primitive::PlainImportExtractor, Extractor},
};

use super::{ImportIdentifiers, QAPattern, QualityAssuranceOutcome};
impl QAPattern for ImportIdentifiers {
    fn find(
        source: HashMap<PathBuf, &mut SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let mut imports = PlainImportExtractor::extract(source_unit)?;
            for import in imports.iter_mut() {
                if let pt::Import::Plain(_, loc) = import {
                    outcome.push_or_insert(path_buf.clone(), *loc, import.to_string());
                }
            }
        }

        Ok(QualityAssuranceOutcome::ImportIdentifiers(outcome))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::{create_test_source, engine::Report, utils::MockSource, report::ReportSectionFragment};

    use super::*;
    #[test]
    fn test_import_identifiers() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract Contract0 {
        
        function msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgData() private view returns(bytes calldata) {
            return msg.data;
        }

        function msgData() private view returns(bytes calldata) {
            return msg.data;
        }
    }
    "#;

    let mut mock_source = MockSource::new().add_source(file_contents);
    let source = std::mem::take(&mut mock_source.source);
        let qa_locations = ImportIdentifiers::find(source)?;
        assert_eq!(qa_locations.len(), 1);
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
