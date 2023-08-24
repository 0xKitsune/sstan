use std::{collections::{HashSet, HashMap}, path::PathBuf};

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::{engine::{EngineError, Outcome, Pushable}, extractors::{primitive::PlainImportExtractor, Extractor}};

use super::{ QAPattern, QualityAssuranceOutcome, ImportIdentifiers};
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
    use crate::{
        cleanup_test_source, create_test_source,
        engine::Report,
        qa::{PrivateVariablesLeadingUnderscore, QAPattern, ImportIdentifiers},
    };
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

        let report:Report = qa_locations.into();
        println!("{:#?}", report);
        Ok(())
    }
}
