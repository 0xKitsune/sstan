use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::create_test_source;
use crate::engine::{EngineError, Pushable, Report};
use crate::extractors::compound::ContractPartFunctionExtractor;
use crate::extractors::Extractor;
use crate::qa::ConstructorOrder;

use super::{Outcome, QAPattern, QualityAssuranceOutcome};

impl QAPattern for ConstructorOrder {
    fn find(
        source: HashMap<PathBuf, &mut SourceUnit>,
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
                            outcome.push_or_insert(path_buf, node.loc, node.to_string());
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

    let source = create_test_source!(file_contents);
    let qa_locations = ConstructorOrder::find(source)?;
    dbg!(&qa_locations);
    assert_eq!(qa_locations.len(), 1);
    Ok(())
}
