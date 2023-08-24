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

    use crate::{create_test_source, engine::Report};

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

        let source = create_test_source!(file_contents);
        let qa_locations = ImportIdentifiers::find(source)?;
        assert_eq!(qa_locations.len(), 1);
        let report: Report = qa_locations.into();
        let mut f = File::options()
            .append(true)
            .open("src/qa/test_report/mock_report.md")?;
        writeln!(&mut f, "{}", report)?;

        Ok(())
    }
}
