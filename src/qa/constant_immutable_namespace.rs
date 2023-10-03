use std::collections::HashMap;
use std::path::PathBuf;

use regex::Regex;
use solang_parser::pt::Loc;
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Pushable};
use crate::extractors::compound::{
    ConstantStorageVariableExtractor, ImmutableStorageVariableExtractor,
};
use crate::extractors::Extractor;

use super::{ConstantImmutableNamespace, Outcome, QAPattern, QualityAssuranceOutcome};
impl QAPattern for ConstantImmutableNamespace {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let re: Regex = Regex::new(r"\b[A-Z][A-Z0-9_]*\b")?;
            let constant_variables = ConstantStorageVariableExtractor::extract(source_unit)?;
            let immutable_variables = ImmutableStorageVariableExtractor::extract(source_unit)?;

            for var in constant_variables.iter().chain(immutable_variables.iter()) {
                if let Some(name) = var.name.clone() {
                    if !re.is_match(name.name.as_str()) {
                        outcome.push_or_insert(path_buf.clone(), var.loc, var.to_string());
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::ConstantImmutableNamespace(outcome))
    }
}
#[cfg(test)]
mod test {

    use crate::utils::MockSource;

    use super::*;
    #[test]
    fn test_constant_immutable_namespace() -> eyre::Result<()> {
        let file_contents = r#"
    contract Contract {

        address immutable IS_FINE;
        address constant is_bad = address(1);
        address immutable Is_Bad;
        address constant ALSO_IS_FINE = address(1);
        constructor(address _isFine, address _isBad) {
            IS_FINE = _isFine;
            Is_Bad = _isBad;
        }
    
    }
    "#;

        let mut mock_source = MockSource::new().add_source("constant_immutable.sol", file_contents);
        let qa_locations = ConstantImmutableNamespace::find(&mut mock_source.source)?;
        assert_eq!(qa_locations.len(), 2);

        Ok(())
    }
}
