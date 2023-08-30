use super::{ConstructorOrder, Outcome, QAPattern, QualityAssuranceOutcome};

use crate::engine::{EngineError, Pushable};
use crate::extractors::compound::ContractPartFunctionExtractor;
use crate::extractors::Extractor;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};
use std::collections::HashMap;

use std::path::PathBuf;

impl QAPattern for ConstructorOrder {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let target_nodes = ContractPartFunctionExtractor::extract(source_unit)?;

            let mut fn_counter: u8 = 0; // up to 256 function definitions before reaching the constructor function

            //For each target node that was extracted, check for the qa patterns
            for node in target_nodes {
                match node.ty {
                    pt::FunctionTy::Constructor => {
                        if fn_counter > 0 {
                            outcome.push_or_insert(
                                path_buf.to_path_buf(),
                                node.loc,
                                node.to_string(),
                            );
                            break;
                        }
                    }
                    // Modifiers must be placed before constructor
                    pt::FunctionTy::Modifier => continue,
                    _ => {
                        fn_counter += 1;
                    }
                }
            }
        }
        Ok(QualityAssuranceOutcome::ConstructorOrder(outcome))
    }
}
#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::{report::ReportSectionFragment, utils::MockSource};

    use super::*;
    #[test]
    fn test_constructor_order_qa() -> eyre::Result<()> {
        let file_contents = r#"
    contract Contract1 {
        address public owner;
        function test() public {
            owner = address(0);
        }
        constructor() {
            owner = address(1);
        }
    }
  
    contract Contract2 {
        address public owner;
        receive() external payable {}
        constructor() {
            owner = address(1);
        }
    }
   
    contract Contract3 {
        address public owner;
        modifier onlyOwner {
            require(
            msg.sender == owner,
            "Only owner can call this function."
            );
            _;
        }
        constructor() {
            owner = address(0);
        }
    }
    
    contract Contract4 {
        address public owner;
        function test() public {
            owner = address(0);
        }
    }
    "#;

        let mock_source = MockSource::new().add_source("constructor_order_qa.sol", file_contents);
        let qa_locations = ConstructorOrder::find(mock_source.source)?;
        assert_eq!(qa_locations.len(), 1);
        let report: Option<ReportSectionFragment> = qa_locations.into();
        if let Some(report) = report {
            let mut f = File::options()
                .append(true)
                .open("src/report/mocks/qa_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }

        Ok(())
    }
}
