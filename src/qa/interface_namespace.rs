use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{Loc, SourceUnit};

use crate::qa::InterfaceNamespace;
use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{compound::InterfaceExtractor, Extractor},
};

use super::{QAPattern, QualityAssuranceOutcome};
impl QAPattern for InterfaceNamespace {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();
        for (path_buf, source_unit) in source {
            let interfaces = InterfaceExtractor::extract(source_unit)?;
            for interface in interfaces.iter() {
                if let Some(identifier) = &interface.name {
                    if !identifier.name.starts_with('I') {
                        outcome.push_or_insert(
                            path_buf.clone(),
                            interface.loc,
                            format!("{} {} {{}}", interface.ty, identifier),
                        );
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::InterfaceNamespace(outcome))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use crate::report::ReportSectionFragment;
    use crate::utils::MockSource;
    use std::fs::File;
    use std::io::Write;
    #[test]
    fn test_interface_namespace() -> eyre::Result<()> {
        let file_contents = r#"
    interface IContract {}

    interface Contract0 {
        function foo() external returns (uint256 x);
    }
    "#;
        let mut mock_source =
            MockSource::new().add_source("interface_namespace.sol", file_contents);
        let qa_locations = InterfaceNamespace::find(&mut mock_source.source)?;
        assert_eq!(qa_locations.len(), 1);

        Ok(())
    }
}
