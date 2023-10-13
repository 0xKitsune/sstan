use std::{collections::HashMap, path::PathBuf};

use solang_parser::{
    helpers::CodeLocation,
    pt::{Loc, SourceUnit, VariableAttribute, VariableDefinition, Visibility},
};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{compound::StorageVariableExtractor, Extractor},
};

use super::{ExplicitVisibility, QAPattern, QualityAssuranceOutcome};
impl QAPattern for ExplicitVisibility {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let storage_variables = StorageVariableExtractor::extract(source_unit)?;

            for var in storage_variables {
                let has_visibility_attribute = var.attrs.iter().any(|attr| {
                    if let VariableAttribute::Visibility(_) = attr {
                        true
                    } else {
                        false
                    }
                });

                if !has_visibility_attribute {
                    outcome.push_or_insert(path_buf.clone(), var.loc(), var.to_string());
                }
            }
        }

        Ok(QualityAssuranceOutcome::ExplicitVisibility(outcome))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_explicit_visibility() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract contract0 {

        uint256 private constant CONSTANT_NAME = 1;
        uint256 private constant constantName = 1;
        uint256 constant_name = 1;
        uint256 public IMMUTABLE_NAME = 1;
        uint256 immutable_name = 1;
        uint256 internal immutableName = 1;
    }
 
    "#;

        let mut mock_source = MockSource::new().add_source(
            "explicit_visibility.sol",
            file_contents,
        );
        let qa_locations = ExplicitVisibility::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 2);

        Ok(())
    }
}
