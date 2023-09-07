use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self, CodeLocation};
use solang_parser::{self, pt::SourceUnit};

use crate::extractors::{primitive::FunctionCallExtractor, Extractor};

use super::{OptimizationOutcome, OptimizationPattern, SolidityKeccak256};
use crate::engine::{EngineError, Outcome, Pushable};

impl OptimizationPattern for SolidityKeccak256 {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            //Extract the target nodes from the source_unit
            let function_call_nodes = FunctionCallExtractor::extract(source_unit)?;

            //For each target node that was extracted, check for the optimization patterns
            for node in function_call_nodes {
                //Can unwrap because FunctionCall is an expression

                if let pt::Expression::FunctionCall(_, box_expression, _) = node.clone() {
                    if let pt::Expression::Variable(variable) = *box_expression {
                        if variable.name == "keccak256" {
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string());
                        }
                    }
                }
            }
        }
        //Return the identified optimization locations
        Ok(OptimizationOutcome::SolidityKeccak256(outcome))
    }
}
mod test {
    use std::{fs::File, io::Write};

    use crate::{
        optimizations::{OptimizationPattern, SolidityKeccak256},
        report::ReportSectionFragment,
        utils::MockSource,
    };

    #[test]
    fn test_template_optimization() -> eyre::Result<()> {
        let file_contents = r#"
    
contract Contract0 {

    constructor(uint256 a, uint256 b){
        keccak256(abi.encodePacked(a, b));

    }

    function solidityHash(uint256 a, uint256 b) public view {
        //unoptimized
        keccak256(abi.encodePacked(a, b));
    }


    function assemblyHash(uint256 a, uint256 b) public view {
        //optimized
        assembly {
            mstore(0x00, a)
            mstore(0x20, b)
            let hashedVal := keccak256(0x00, 0x40)
        }
    }
}
    "#;

        let mut source = MockSource::new().add_source("solidity_keccak256.sol", file_contents);
        let optimization_locations = SolidityKeccak256::find(&mut source.source)?;

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
