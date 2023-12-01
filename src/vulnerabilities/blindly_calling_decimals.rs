use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::helpers::CodeLocation;
use solang_parser::pt::Expression;
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::primitive::{FunctionExtractor, MemberAccessExtractor};
use crate::extractors::Extractor;

use super::{BlindDecimalsCall, VulnerabilityOutcome, VulnerabilityPattern};

impl VulnerabilityPattern for BlindDecimalsCall {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<VulnerabilityOutcome, EngineError> {
        let mut vulnerability_locations = Outcome::new();

        for (path_buf, source_unit) in source {
            let mut functions = FunctionExtractor::extract(source_unit)?;
            for function in functions.iter_mut() {
                let member_accesses = MemberAccessExtractor::extract(function)?;
                for member_access in member_accesses.iter() {
                    if let Expression::MemberAccess(_, _, ident) = member_access {
                        if ident.name == "decimals" {
                            vulnerability_locations.push_or_insert(
                                path_buf.clone(),
                                member_access.loc(),
                                member_access.to_string(),
                            );
                        }
                    }
                }
            }
        }

        Ok(VulnerabilityOutcome::BlindDecimalsCall(
            vulnerability_locations,
        ))
    }
}
mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[allow(unused)]
    use crate::vulnerabilities::BlindDecimalsCall;
    #[test]
    fn test_blindly_calling_decimals() -> eyre::Result<()> {
        let file_contents = r#"
    
    contract Contract0 {
        
        function blindDecimalsCall(addrress arbitraryToken) public returns (uint8) {
            return IERC20(arbitraryToken).decimals();
        }
        //TODO:
        // function safeDecimalsCall(address arbitraryToken) public returns (uint256) {
        //     arbitraryToken.decimals();
        // }
    }
    "#;
        let mut mock_source =
            MockSource::new().add_source("blind_decimals_call.sol", file_contents);
        let vuln_locations = BlindDecimalsCall::find(&mut mock_source.source)?;
        assert_eq!(vuln_locations.len(), 1);
        Ok(())
    }
}
