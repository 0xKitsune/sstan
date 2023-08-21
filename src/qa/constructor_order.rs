use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::Report;
use crate::extractors::compound::ContractPartFunctionExtractor;
use crate::extractors::Extractor;
use crate::qa;
use crate::qa::ConstructorOrder;

use super::{Outcome, QAPattern, QualityAssuranceOutcome};

impl QAPattern for ConstructorOrder {
    fn find(source: HashMap<PathBuf, &mut SourceUnit>) -> QualityAssuranceOutcome {
        let mut outcome = Outcome::new();

        for (path_buf, source_unit) in source {
            let target_nodes = ContractPartFunctionExtractor::extract(source_unit).unwrap();

            let mut fn_counter: u8 = 0; // up to 256 function definitions before reaching the constructor function

            //For each target node that was extracted, check for the qa patterns
            for node in target_nodes {
                match node.ty {
                    pt::FunctionTy::Constructor => {
                        if fn_counter > 0 {
                            outcome.insert(path_buf, vec![(node.loc, node.to_string())]);
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

        QualityAssuranceOutcome::ConstructorOrder(outcome)
    }
}

#[test]
fn test_constructor_order_qa() {
    let test_contracts = vec![
        r#"
    contract Contract0 {
        address public owner;
        function test() public {
            owner = address(0);
        }
        constructor() {
            owner = address(1);
        }
    }
    "#,
        r#"
    contract Contract0 {
        address public owner;
        receive() external payable {}
        constructor() {
            owner = address(1);
        }
    }
    "#,
        r#"
    contract Contract0 {
        address public owner;
        modifier onlyOwner {
            require(
            msg.sender == owner,
            "Only owner can call this function."
            );
            _;
        }
        constructor() {
            owner = address(1);
        }
    }
    "#,
        r#"
    contract Contract0 {
        address public owner;
        function test() public {
            owner = address(0);
        }
    }
    "#,
    ];

    let assertions = vec![1, 1, 0, 0];

    assert_eq!(test_contracts.len(), assertions.len());

    if !assertions.is_empty() {
        for i in 0..assertions.len() - 1 {
            let mut source = HashMap::new();
            let mut source_unit = solang_parser::parse(test_contracts[i], 0).unwrap().0;
            source.insert(PathBuf::new(), &mut source_unit);

            let qa_locations = ConstructorOrder::find(source);

            dbg!("{:?}", &qa_locations);
            assert_eq!(qa_locations.len(), assertions[i]);

            let report: Report = qa_locations.into();

            println!("{}", report)
        }
    }
}
