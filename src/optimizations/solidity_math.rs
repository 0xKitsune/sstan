use std::collections::{HashMap};
use std::path::PathBuf;

use solang_parser::pt::{self, CodeLocation};
use solang_parser::{self, pt::SourceUnit};

use crate::extractors::primitive::UrnaryOpteratorExtractor;
use crate::extractors::Extractor;

use super::{OptimizationOutcome, OptimizationPattern, SolidityMath};
use crate::engine::{EngineError, Outcome, Pushable};

impl OptimizationPattern for SolidityMath {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            //Extract the target nodes from the source_unit
            let urnary_operator_nodes = UrnaryOpteratorExtractor::extract(source_unit)?;

            //For each target node that was extracted, check for the optimization patterns
            for node in urnary_operator_nodes {
                //Can unwrap because all targets are expressions
                match node {
                    pt::Expression::Add(_loc, _, _)
                    | pt::Expression::Subtract(_loc, _, _)
                    | pt::Expression::Multiply(_loc, _, _)
                    | pt::Expression::Divide(_loc, _, _) => {
                        outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                    }

                    _ => {}
                }
            }
        }
        //Return the identified optimization locations
        Ok(OptimizationOutcome::SolidityMath(outcome))
    }
}
mod test {
    use crate::{
        optimizations::{OptimizationPattern, SolidityMath},
        utils::MockSource,
    };

    #[test]
    fn test_analyze_for_math_optimization() -> eyre::Result<()> {
        let file_contents = r#"
    

    contract Contract0 {

        //addition in Solidity
        function addTest(uint256 a, uint256 b) public pure {
            uint256 c = a + b;
        }

        //addition in assembly
        function addAssemblyTest(uint256 a, uint256 b) public pure {
            assembly {
                let c := add(a, b)
    
                if lt(c, a) {
                    mstore(0x00, "overflow")
                    revert(0x00, 0x20)
                }
            }
        }

        //subtraction in Solidity
        function subTest(uint256 a, uint256 b) public pure {
            uint256 c = a - b;
        }
    }
    
    contract Contract3 {
        //subtraction in assembly
        function subAssemblyTest(uint256 a, uint256 b) public pure {
            assembly {
                let c := sub(a, b)
    
                if gt(c, a) {
                    mstore(0x00, "underflow")
                    revert(0x00, 0x20)
                }
            }
        }

        //multiplication in Solidity
        function mulTest(uint256 a, uint256 b) public pure {
            uint256 c = a * b;
        }
        //multiplication in assembly
        function mulAssemblyTest(uint256 a, uint256 b) public pure {
            assembly {
                let c := mul(a, b)
    
                if lt(c, a) {
                    mstore(0x00, "overflow")
                    revert(0x00, 0x20)
                }
            }
        }

        //division in Solidity
        function divTest(uint256 a, uint256 b) public pure {
            uint256 c = a * b;
        }
        
        function divAssemblyTest(uint256 a, uint256 b) public pure {
            assembly {
                let c := div(a, b)
    
                if gt(c, a) {
                    mstore(0x00, "underflow")
                    revert(0x00, 0x20)
                }
            }
        }
    }

    "#;

        let mut source = MockSource::new().add_source("solidity_math.sol", file_contents);
        let optimization_locations = SolidityMath::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 4);

        Ok(())
    }
}
