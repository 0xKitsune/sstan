use solang_parser::pt::{self, CodeLocation, Expression, Loc};
use solang_parser::{self, pt::SourceUnit};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use super::{IncrementDecrement, OptimizationOutcome, OptimizationPattern};
use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::{
    primitive::{BlockExtractor, IncrementorExtractor},
    Extractor,
};

impl OptimizationPattern for IncrementDecrement {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            //Get all increment/decrement expressions in unchecked blocks so that the analyzer does not mark these as optimization targets
            let block_nodes = BlockExtractor::extract(source_unit)?;
            let mut unchecked_locations: HashSet<Loc> = HashSet::new();
            for node in block_nodes {
                if let pt::Statement::Block {
                    loc: _,
                    unchecked,
                    mut statements,
                } = node
                {
                    if unchecked {
                        for statement in statements.iter_mut() {
                            unchecked_locations
                                .extend(extract_pre_increment_pre_decrement(statement));
                        }
                    }
                }
            }

            //Get all increment / decrement locations mapped to the node
            let nodes = extract_increment_decrement(source_unit);

            for node in nodes {
                if !unchecked_locations.contains(&node.0) {
                    outcome.push_or_insert(path_buf.clone(), node.1.loc(), node.1.to_string());
                }
            }
        }

        Ok(OptimizationOutcome::IncrementDecrement(outcome))
    }
}

pub fn extract_increment_decrement(node: &mut SourceUnit) -> HashMap<Loc, Expression> {
    let mut locations: HashMap<Loc, Expression> = HashMap::new();

    let target_nodes = IncrementorExtractor::extract(node).unwrap();

    for node in target_nodes {
        match node {
            Expression::PreIncrement(loc, _) => {
                locations.insert(loc, node);
            }
            Expression::PreDecrement(loc, _) => {
                locations.insert(loc, node);
            }
            Expression::PostIncrement(loc, _) => {
                locations.insert(loc, node);
            }
            Expression::PostDecrement(loc, _) => {
                locations.insert(loc, node);
            }

            _ => {}
        }
    }
    locations
}

pub fn extract_pre_increment_pre_decrement(node: &mut pt::Statement) -> HashSet<Loc> {
    let mut locations: HashSet<Loc> = HashSet::new();

    let target_nodes = IncrementorExtractor::extract(node).unwrap();

    for node in target_nodes {
        match node {
            Expression::PreIncrement(loc, _) => {
                locations.insert(loc);
            }
            Expression::PreDecrement(loc, _) => {
                locations.insert(loc);
            }

            _ => {}
        }
    }
    locations
}
mod test {
    use std::{fs::File, io::Write};

    use crate::{
        optimizations::{IncrementDecrement, OptimizationPattern},
        report::ReportSectionFragment,
        utils::MockSource,
    };

    #[test]
    fn test_increment_optimization() -> eyre::Result<()> {
        let file_contents = r#"
  
    contract Contract0 {
        function iPlusPlus(){
            uint256  i = 0;
            i++;
        }
    
        function plusPlusI() public {
            uint256  i = 0;
            ++i;
    
        for (uint256 j = 0; j < numNfts; ) {
                bytes32 hash = keccak256(abi.encode(ORDER_ITEM_HASH, nfts[i].collection, _tokensHash(nfts[i].tokens)));
                hashes[i] = hash;
                unchecked {
                  ++j;
                }
              }
    
            unchecked{
                i++;
            }
        }
        
    }
    
    "#;

        let mut source = MockSource::new().add_source("increment_decrement.sol", file_contents);
        let optimization_locations = IncrementDecrement::find(&mut source.source)?;
        assert_eq!(optimization_locations.len(), 3);
        let report: Option<ReportSectionFragment> = optimization_locations.into();
        if let Some(report) = report {
            let mut f = File::options()
                .append(true)
                .open("optimization_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }
        Ok(())
    }
}
