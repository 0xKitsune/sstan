use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use super::{OptimalComparison, OptimizationOutcome, OptimizationPattern};
use solang_parser::pt::{self, CodeLocation, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::{primitive::EqualityExtractor, Extractor};

impl OptimizationPattern for OptimalComparison {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            //Extract the target nodes from the source_unit
            let equality_nodes = EqualityExtractor::extract(source_unit)?;

            //For each target node that was extracted, check for the optimization patterns
            for node in equality_nodes {
                match node.clone() {
                    // >= operator
                    pt::Expression::MoreEqual(_loc, _box_expression_0, _box_expression_1) => {
                        outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                    }

                    // <= operator
                    pt::Expression::LessEqual(_loc, _box_expression_0, _box_expression_1) => {
                        outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                    }

                    _ => {}
                }
            }
        }
        //Return the identified optimization locations
        Ok(OptimizationOutcome::OptimalComparison(outcome))
    }
}
mod test {
    use crate::{
        optimizations::{OptimalComparison, OptimizationPattern},
        utils::MockSource,
    };

    #[test]
    fn test_optimal_comparison_optimization() -> eyre::Result<()> {
        let file_contents = r#"
    
contract Contract0 {
    function greaterThanOrEqualTo(uint256 a, uint256 b) public pure {
        return a >= b;
    }

    function lessThanOrEqualTo(uint256 a, uint256 b) public pure {
        return a <= b;
    }
}
    "#;

        let mut source = MockSource::new().add_source("optimal_comparison.sol", file_contents);
        let optimization_locations = OptimalComparison::find(&mut source.source)?;
        assert_eq!(optimization_locations.len(), 2);
        Ok(())
    }
}
