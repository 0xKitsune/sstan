use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};

use crate::extractors::primitive::MemberAccessExtractor;
use crate::extractors::Extractor;

use super::{UnsafeErc20Operation, VulnerabilityOutcome, VulnerabilityPattern};

impl VulnerabilityPattern for UnsafeErc20Operation {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<VulnerabilityOutcome, EngineError> {
        let mut vulnerability_locations = Outcome::new();

        for (path_buf, source_unit) in source {
            //Extract the target nodes from the source_unit
            let member_access_nodes = MemberAccessExtractor::extract(source_unit)?;

            //For each target node that was extracted, check for the vulnerability patterns

            for node in member_access_nodes {
                //We can use unwrap because Target::MemberAccess is an expression

                if let pt::Expression::MemberAccess(loc, _, identifier) = &node {
                    if identifier.name == "transfer"
                        || identifier.name == "transferFrom"
                        || identifier.name == "approve"
                    {
                        vulnerability_locations.push_or_insert(
                            path_buf.clone(),
                            *loc,
                            node.to_string(),
                        );
                    }
                }
            }
        }

        Ok(VulnerabilityOutcome::UnsafeErc20Operation(
            vulnerability_locations,
        ))
    }
}
mod test {
    #[allow(unused)]
    use crate::utils::MockSource;
    #[allow(unused)]
    use super::*;
    #[test]
    fn test_unsafe_erc20_operation() -> eyre::Result<()> {
        let file_contents = r#"
    
    contract Contract0 {
        IERC20 e;

        constructor(){
            e = IERC20(0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984);
        }

        function unsafe_erc20_operations() public {
            e.approve(address(0), 200);
            e.transfer(address(0), 100);
            e.transferFrom(address(0), 100);
        }

    }
    "#;
        let mut mock_source =
            MockSource::new().add_source("unsafe_erc20_operation.sol", file_contents);
        let vuln_locations = UnsafeErc20Operation::find(&mut mock_source.source)?;
        assert_eq!(vuln_locations.len(), 3);

        Ok(())
    }
}
