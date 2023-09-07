use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use solang_parser::pt::{CodeLocation, Expression, SourceUnit};

use crate::extractors::{
    compound::MutableStorageVariableExtractor,
    primitive::{ContractDefinitionExtractor, ForExtractor, VariableExtractor},
    Extractor,
};

use super::{OptimizationOutcome, OptimizationPattern, ReadStorageInForLoop};
use crate::engine::{EngineError, Outcome, Pushable};

impl OptimizationPattern for ReadStorageInForLoop {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;

            for contract in contracts.iter_mut() {
                let storage_variables = MutableStorageVariableExtractor::extract(contract)?;
                let mut variable_names = HashSet::new();
                for storage_variable in storage_variables {
                    if let Some(identifier) = storage_variable.name {
                        variable_names.insert(identifier.name);
                    }
                }

                let mut for_loops = ForExtractor::extract(contract)?;
                for for_loop in for_loops.iter_mut() {
                    let variables = VariableExtractor::extract(for_loop)?;
                    for variable in variables {
                        if let Expression::Variable(identifier) = variable {
                            if variable_names.contains(&identifier.name) {
                                outcome.push_or_insert(
                                    path_buf.clone(),
                                    for_loop.loc(),
                                    for_loop.to_string(),
                                )
                            }
                        }
                    }
                }
            }
        }
        Ok(OptimizationOutcome::ReadStorageInForLoop(outcome))
    }
}
mod test {
    use std::{fs::File, io::Write};

    use crate::{
        optimizations::{OptimizationPattern, ReadStorageInForLoop},
        report::ReportSectionFragment,
        utils::MockSource,
    };

    #[test]
    fn test_read_from_storage_in_for_loop() -> eyre::Result<()> {
        let file_contents = r#"

    contract Contract0 {
        uint x;
        uint y;
        function shouldCacheInMemory() public {
            for (uint i = 0; i < 10; i++) {
                uint z = y;
            }
        }
        function shouldCacheInMemory() public {
            for (uint i = 0; i < 10; i++) {
                uint z = x;
            }
        }
        function isCachingInMemory() public {
            uint z = x;
            uint a = z;
        }

    }
    "#;

        let mut source =
            MockSource::new().add_source("read_from_storage_in_for_loop.sol", file_contents);
        let optimization_locations = ReadStorageInForLoop::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 2);
        let report: Option<ReportSectionFragment> = optimization_locations.into();
        if let Some(report) = report {
            let mut f = File::options()
                .append(true)
                .open("optimization_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }
        Ok(())
    }
}
