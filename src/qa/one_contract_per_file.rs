use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{compound::ContractExtractor, Extractor},
};

use super::{OneContractPerFile, QAPattern, QualityAssuranceOutcome};
impl QAPattern for OneContractPerFile {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let contracts = ContractExtractor::extract(source_unit)?;

            if contracts.len() > 1 {
                for contract in contracts.iter() {
                    outcome.push_or_insert(path_buf.clone(), contract.loc, contract.to_string());
                }
            }
        }

        Ok(QualityAssuranceOutcome::OneContractPerFile(outcome))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::MockSource;

    use super::*;
    #[test]
    fn test_one_contract_per_file() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract Contract0 {
       
    }

    contract Contract1 {
       
    }

    contract Contract2 {
       
    }
    "#;

        let mut mock_source =
            MockSource::new().add_source("one_contract_per_file.sol", file_contents);
        let qa_locations = OneContractPerFile::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 3);

        Ok(())
    }
}
