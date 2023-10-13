use std::{collections::HashMap, path::PathBuf};

use solang_parser::{
    helpers::CodeLocation,
    pt::{Expression, Loc, SourceUnit},
};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{primitive::MemberAccessExtractor, Extractor},
};

use super::{QAPattern, QualityAssuranceOutcome, RemoveConsole};
impl QAPattern for RemoveConsole {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let member_accesses = MemberAccessExtractor::extract(source_unit)?;
            for member_access in member_accesses {
                //TODO: update this to not use clone
                if let Expression::MemberAccess(_, expression, _) = &member_access {
                    if let Expression::Variable(identifier) = *expression.clone() {
                        if identifier.name == "console" {
                            outcome.push_or_insert(
                                path_buf.clone(),
                                member_access.loc(),
                                member_access.to_string(),
                            );
                        }
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::RemoveConsole(outcome))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::MockSource;

    use super::*;
    #[test]
    fn test_remove_console() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract Contract0 {
       function someFunction(){

        console.log("hello world");
        console.log("some other string");
       }
    }

    "#;

        let mut mock_source = MockSource::new().add_source("console_log.sol", file_contents);
        let qa_locations = RemoveConsole::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 2);

        Ok(())
    }
}
