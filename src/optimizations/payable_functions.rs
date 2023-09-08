use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::{OptimizationOutcome, OptimizationPattern, PayableFunctions};
use solang_parser::pt::{self, CodeLocation, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::primitive::FunctionExtractor;
use crate::extractors::{primitive::ContractDefinitionExtractor, Extractor};
use crate::report::ReportSectionFragment;
use crate::utils::MockSource;

impl OptimizationPattern for PayableFunctions {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let mut contract_definition_nodes = ContractDefinitionExtractor::extract(source_unit)?;

            for contract_definition_node in contract_definition_nodes.iter_mut() {
                let target_nodes = FunctionExtractor::extract(contract_definition_node)?;

                for node in target_nodes {
                    //We can use unwrap because Target::FunctionDefinition is a contract_part

                    //if there is function body
                    if node.body.is_some() && !node.attributes.is_empty() {
                        let mut payable = false;
                        let mut public_or_external = false;

                        for attr in &node.attributes {
                            match attr {
                                // Visibility
                                pt::FunctionAttribute::Visibility(visibility) => match visibility {
                                    pt::Visibility::External(_) => {
                                        public_or_external = true;
                                    }
                                    pt::Visibility::Public(_) => {
                                        public_or_external = true;
                                    }
                                    _ => {}
                                },
                                pt::FunctionAttribute::Mutability(pt::Mutability::Payable(_)) => {
                                    payable = true;
                                }
                                _ => {}
                            }
                        }

                        //if the function is public or external, and it is not marked as payable
                        if public_or_external && !payable {
                            //insert the loc of the function definition into optimization locations
                            // optimization_locations.insert(node.loc);
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                        }
                    }
                }
            }
        }
        Ok(OptimizationOutcome::PayableFunctions(outcome))
    }
}

#[test]
fn test_payable_function_optimization() -> eyre::Result<()> {
    let file_contents = r#"
    

    contract Contract0 {

        function div2(uint256 a, uint256 b) public pure {
            
        }

        function mul2(uint256 a, uint256 b) external view {
            
        }
    }
    "#;

    let mut source = MockSource::new().add_source("payable_functions.sol", file_contents);
    let optimization_locations = PayableFunctions::find(&mut source.source)?;
    assert_eq!(optimization_locations.len(), 2);

    let report: Option<ReportSectionFragment> = optimization_locations.into();
    if let Some(report) = report {
        let mut f = File::options()
            .append(true)
            .open("mocks/optimization_report_sections.md")?;
        writeln!(&mut f, "{}", &String::from(report))?;
    }
    Ok(())
}
