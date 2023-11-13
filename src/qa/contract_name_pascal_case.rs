use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{primitive::ContractDefinitionExtractor, Extractor},
    utils::is_pascal_case,
};

use super::{ContractNamePascalCase, QAPattern, QualityAssuranceOutcome};
impl QAPattern for ContractNamePascalCase {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let contracts = ContractDefinitionExtractor::extract(source_unit)?;
            for contract in contracts {
                if let Some(name) = &contract.name {
                    if !is_pascal_case(&name.name) {
                        outcome.push_or_insert(
                            path_buf.clone(),
                            contract.loc,
                            contract.to_string(),
                        );
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::ContractNamePascalCase(outcome))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_contract_pascal_case() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract contract0 {}
    contract Contract {}
    contract Contract_2{}
    "#;

        let mut mock_source =
            MockSource::new().add_source("contract_name_pascal_case.sol", file_contents);
        let qa_locations = ContractNamePascalCase::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 2);

        Ok(())
    }
}
