use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use solang_parser::pt::{Loc, SourceUnit};

use crate::create_test_source;
use crate::qa::InterfaceNamespace;
use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{compound::InterfaceExtractor, Extractor},
};

use super::{QAPattern, QualityAssuranceOutcome};
impl QAPattern for InterfaceNamespace {
    fn find(
        source: HashMap<PathBuf, &mut SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();
        for (path_buf, source_unit) in source {
            let interfaces = InterfaceExtractor::extract(source_unit)?;
            for interface in interfaces.iter() {
                if let Some(identifier) = &interface.name {
                    if !identifier.name.starts_with("I") {
                        outcome.push_or_insert(
                            path_buf.clone(),
                            identifier.loc,
                            identifier.to_string(),
                        );
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::InterfaceNamespace(outcome))
    }
}

#[test]
fn test_interface_namespace() -> eyre::Result<()> {
    let file_contents_1 = r#"
    interface IContract {}

    interface Contract0 {
        function foo() external returns (uint256 x);
    }
    "#;
    let source = create_test_source!(file_contents_1);
    let qa_locations_1 = InterfaceNamespace::find(source)?;
    assert_eq!(qa_locations_1.len(), 1);
    Ok(())
}
