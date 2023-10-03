use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self, CodeLocation};
use solang_parser::{self, pt::SourceUnit};

use super::{EngineError, OptimizationOutcome, Outcome};
use crate::engine::Pushable;
use crate::extractors::{
    primitive::{ForExtractor, MemberAccessExtractor},
    Extractor,
};

use super::{CacheArrayLength, OptimizationPattern};

pub const LENGTH: &str = "length";

impl OptimizationPattern for CacheArrayLength {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            //Extract the target nodes from the source_unit

            let mut for_nodes = ForExtractor::extract(source_unit)?;

            //For each target node that was extracted, check for the optimization patterns
            for node in for_nodes.iter_mut() {
                if let pt::Statement::For(_, _, Some(ref mut box_expression), _, _) = node {
                    //get all of the .length in the for loop definition
                    let member_access_nodes = MemberAccessExtractor::extract(box_expression)?;

                    for node in member_access_nodes {
                        //Can unwrap because Target::MemberAccess will always be an expression
                        if let pt::Expression::MemberAccess(_loc, _, identifier) = node.clone() {
                            if identifier.name == LENGTH {
                                outcome.push_or_insert(
                                    path_buf.clone(),
                                    node.loc(),
                                    node.to_string(),
                                );
                            }
                        }
                    }
                }
            }
        }

        //Return the identified optimization locations
        Ok(OptimizationOutcome::CacheArrayLength(outcome))
    }
}
mod test {

    use crate::utils::MockSource;

    use super::*;

    #[test]
    fn test_cache_array_length_optimization() -> eyre::Result<()> {
        let file_contents = r#"

    contract Contract1 {


        //loop with i++
        function memoryArray(uint256[] memory arr) public {
            uint256 j;
            for (uint256 i; i < arr.length; i++) {
                j = arr[i] + 10;
            }
        }
    
        //loop with i++
        function calldataArray(uint256[] calldata arr) public {
            uint256 j;
            for (uint256 i; i < 100; i++) {
                j = arr[i] + arr.length;
            }
        }
    
        //loop with i++
        function memoryArray(uint256[] memory arr) public {
            uint256 j;
            for (uint256 i;  arr.length<1000; i++) {
                arr[i] = 10;
            }
        }
    
        }    
    "#;

        let mut source = MockSource::new().add_source("cache_array_length.sol", file_contents);
        let optimization_locations = CacheArrayLength::find(&mut source.source)?;
        assert_eq!(optimization_locations.len(), 2);

        Ok(())
    }
}
