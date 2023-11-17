use std::{collections::HashMap, path::PathBuf};

use solang_parser::{
    helpers::CodeLocation,
    pt::{Loc, SourceUnit},
};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{compound::RequireExtractor, primitive::ErrorExtractor, Extractor},
};

use super::{InconsistentRequireError, QAPattern, QualityAssuranceOutcome};
impl QAPattern for InconsistentRequireError {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let errors = ErrorExtractor::extract(source_unit)?;
            let requires = RequireExtractor::extract(source_unit)?;
            //Using both requires/revertsx
            if !requires.is_empty() && !errors.is_empty() {
                //Show all errors
                if requires.len() >= errors.len() {
                    for error in errors {
                        outcome.push_or_insert(path_buf.clone(), error.loc, error.to_string());
                    }
                } else {
                    //Show all requires
                    for require in requires {
                        outcome.push_or_insert(
                            path_buf.clone(),
                            require.loc(),
                            require.to_string(),
                        );
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::InconsistentRequireError(outcome))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_inconsistent_require_error() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract contract0 {

        error SomeError();
        error SomeOtherError();

        function foo() public {
            require(true, "SomeError");
            revert SomeError();
        }

        function bar() {
            require(true, "SomeError");
            require(true, "SomeOtherError");
            revert SomeOtherError();
        }
    }
 
    "#;

        let mut mock_source =
            MockSource::new().add_source("consistent_require_revert.sol", file_contents);
        let qa_locations = InconsistentRequireError::find(&mut mock_source.source)?;
        //Should show all errors, since there are more requires
        assert_eq!(qa_locations.len(), 2);

        Ok(())
    }
}
