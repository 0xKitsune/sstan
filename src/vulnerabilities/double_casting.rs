use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::helpers::CodeLocation;
use solang_parser::pt::{Expression, Type};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::primitive::FunctionCallExtractor;
use crate::extractors::Extractor;

use super::{DoubleCasting, VulnerabilityOutcome, VulnerabilityPattern};

impl VulnerabilityPattern for DoubleCasting {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<VulnerabilityOutcome, EngineError> {
        let mut vulnerability_locations = Outcome::new();

        for (path_buf, source_unit) in source {
            let function_calls = FunctionCallExtractor::extract(source_unit)?;

            for function_call in function_calls {
                if let Expression::FunctionCall(_, ty, expr_args) = function_call.clone() {
                    if let Expression::Type(_, primitive) = *ty {
                        match primitive {
                            Type::Uint(capacity) | Type::Int(capacity) => {
                                //Check the args are a function call on an Int/Uint
                                for arg in expr_args {
                                    if let Expression::FunctionCall(_, ty, _) = arg {
                                        if let Expression::Type(_, primitive) = *ty {
                                            match primitive {
                                                Type::Uint(inner_capacity)
                                                | Type::Int(inner_capacity) => {
                                                    //If the capacity of the inner cast is greater than capacity we might have truncation from the double cast
                                                    if inner_capacity > capacity {
                                                        vulnerability_locations.push_or_insert(
                                                            path_buf.clone(),
                                                            function_call.loc(),
                                                            function_call.to_string(),
                                                        );
                                                        break;
                                                    }
                                                    break;
                                                }
                                                _ => continue,
                                            }
                                        }
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }
        }

        Ok(VulnerabilityOutcome::DoubleCasting(vulnerability_locations))
    }
}
mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_double_cast() -> eyre::Result<()> {
        let file_contents = r#"
    
    contract Contract0 {
        uint256 constant x = uint128(uint256(1));
        uint256 constant y = int128(int256(1));
        function isOkay() public returns (uint256) {
            return uint256(uint128(1));
        }

        function alsoBad() public returns (int128 x) {
            x = int128(int256(1));
        }

        function okay() public returns (uint128) {
            uint256 x = 1;
            return uint128(x);
        }

        function alsoOkay() public returns (uint256 x) {
            x = 1;
            return uint256(x);
        }
    }
    "#;
        let mut mock_source = MockSource::new().add_source("double_cast.sol", file_contents);
        let vuln_locations = DoubleCasting::find(&mut mock_source.source)?;
        assert_eq!(vuln_locations.len(), 3);
        Ok(())
    }
}
