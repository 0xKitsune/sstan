use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::helpers::CodeLocation;
use solang_parser::pt::{self};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::primitive::MemberAccessExtractor;
use crate::extractors::Extractor;

use super::{AddressBalance, OptimizationOutcome, OptimizationPattern};

pub const BALANCE: &str = "balance";

//Use selfbalance() instead of address(this).balance()
impl OptimizationPattern for AddressBalance {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let member_access_nodes = MemberAccessExtractor::extract(source_unit)?;

            for node in member_access_nodes {
                //We can use unwrap because Target::MemberAccess is an expression

                if let pt::Expression::MemberAccess(_loc, box_expression, identifier) = node.clone()
                {
                    if let pt::Expression::FunctionCall(_, box_expression, _) = *box_expression {
                        if let pt::Expression::Type(_, pt::Type::Address) = *box_expression {
                            //if address(0x...).balance or address(this).balance
                            if identifier.name == *BALANCE {
                                outcome.push_or_insert(
                                    path_buf.clone(),
                                    node.loc(),
                                    node.to_string(),
                                )
                            }
                        }
                    }
                }
            }
        }

        Ok(OptimizationOutcome::AddressBalance(outcome))
    }
}
mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;

    #[test]
    fn test_address_balance_optimization() -> eyre::Result<()> {
        let file_contents = r#"
    
contract Contract0 {
    function addressInternalBalance(){
        uint256 bal = address(this).balance;
        bal++;
    }

    function addressExternalBalance(address addr) public {
        uint256 bal = address(addr).balance;
        bal++;
    }
}

    "#;
        let mut source = MockSource::new().add_source("address_balance.sol", file_contents);

        let optimization_locations = AddressBalance::find(&mut source.source)?;
        assert_eq!(optimization_locations.len(), 2);

        Ok(())
    }
}
