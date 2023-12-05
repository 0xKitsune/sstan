use std::collections::HashMap;
use std::path::PathBuf;

use crate::engine::{EngineError, Outcome};
use solang_parser::{self, pt::SourceUnit};

use super::{OptimizationOutcome, OptimizationPattern, OptimizationTemplate};

//Template Optimization Pattern
impl OptimizationPattern for OptimizationTemplate {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut _outcome = Outcome::new();
        #[allow(clippy::for_kv_map)]
        for (_path_buf, _source_unit) in source {}

        Ok(OptimizationOutcome::OptimizationTemplate(_outcome))
    }
}
mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;

    #[test]
    fn test_template_optimization() -> eyre::Result<()> {
        let file_contents = r#"contract Contract0 {}"#;
        let mut source = MockSource::new().add_source("template_optimization.sol", file_contents);

        let optimization_locations = OptimizationTemplate::find(&mut source.source)?;
        assert_eq!(optimization_locations.len(), 0);

        Ok(())
    }
}
