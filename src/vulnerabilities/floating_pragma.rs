use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::PathBuf;

use solang_parser::pt::{self, Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::primitive::PragmaDirectiveExtractor;
use crate::extractors::{
    primitive::{AssignmentExtractor, UrnaryOpteratorExtractor},
    Extractor,
};
use crate::report::ReportSectionFragment;
use crate::utils::MockSource;
use std::io::Write;

use super::{FloatingPragma, VulnerabilityOutcome, VulnerabilityPattern};

impl VulnerabilityPattern for FloatingPragma {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<VulnerabilityOutcome, EngineError> {
        let mut vulnerability_locations = Outcome::new();

        for (path_buf, source_unit) in source {
            //Extract the target nodes from the source_unit
            let target_nodes = PragmaDirectiveExtractor::extract(source_unit)?;

            //For each target node that was extracted, check for the vulnerability patterns
            for node in target_nodes {
                //We can use unwrap because Target::PragmaDirective is a source unit part
                if let pt::SourceUnitPart::PragmaDirective(loc, _, Some(pragma)) = &node {
                    if pragma.string.contains('^') {
                        vulnerability_locations.push_or_insert(
                            path_buf.clone(),
                            *loc,
                            node.to_string(),
                        );
                    }
                }
            }
        }

        Ok(VulnerabilityOutcome::DivideBeforeMultiply(
            vulnerability_locations,
        ))
    }
}

#[test]
fn test_floating_pragma_vulnerability() -> eyre::Result<()> {
    let file_contents = r#"

    pragma solidity ^0.8.16;

    contract Contract0 {

    }
    "#;

    let mut mock_source = MockSource::new().add_source("floating_pragma.sol", file_contents);
    let vuln_locations = FloatingPragma::find(&mut mock_source.source)?;
    assert_eq!(vuln_locations.len(), 1);

    let report: Option<ReportSectionFragment> = vuln_locations.into();
    if let Some(report) = report {
        let mut f = File::options()
            .append(true)
            .open("mocks/vulnerability_report_sections.md")?;
        writeln!(&mut f, "{}", &String::from(report))?;
    }

    Ok(())
}
