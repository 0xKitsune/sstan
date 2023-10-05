use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{CodeLocation, Expression, Loc, SourceUnit, UsingList};

use super::{OptimizationOutcome, OptimizationPattern, SafeMathPost080, SafeMathPre080};
use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        compound::SolidityVerisonExtractor,
        primitive::{MemberAccessExtractor, UsingListExtractor},
        Extractor,
    },
};

pub const SAFE_MATH: &str = "SafeMath";
pub const ADD: &str = "add";
pub const DIV: &str = "div";
pub const SUB: &str = "sub";
pub const MUL: &str = "mul";

impl OptimizationPattern for SafeMathPost080 {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let solidity_versions = SolidityVerisonExtractor::extract(source_unit)?;

            for version in solidity_versions.into_iter().flatten() {
                if version.minor >= 8 {
                    //if using safe math
                    if check_if_using_safe_math(&mut source_unit.clone())? {
                        //get all locations that safe math functions are used

                        parse_contract_for_safe_math_functions(source_unit)?
                            .iter()
                            .for_each(|(loc, snippet)| {
                                outcome.push_or_insert(path_buf.clone(), *loc, snippet.clone());
                            });
                    }
                }
            }
        }
        Ok(OptimizationOutcome::SafeMathPost080(outcome))
    }
}

impl OptimizationPattern for SafeMathPre080 {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let solidity_versions = SolidityVerisonExtractor::extract(source_unit)?;

            for version in solidity_versions.into_iter().flatten() {
                if version.minor < 8 {
                    //if using safe math
                    if check_if_using_safe_math(&mut source_unit.clone())? {
                        //get all locations that safe math functions are used
                        parse_contract_for_safe_math_functions(source_unit)?
                            .iter()
                            .for_each(|(loc, snippet)| {
                                outcome.push_or_insert(path_buf.clone(), *loc, snippet.clone());
                            });
                    }
                }
            }
        }
        Ok(OptimizationOutcome::SafeMathPost080(outcome))
    }
}

fn check_if_using_safe_math(source_unit: &mut SourceUnit) -> Result<bool, EngineError> {
    let mut using_safe_math: bool = false;

    let using_list_nodes = UsingListExtractor::extract(source_unit)?;
    for node in using_list_nodes {
        if let UsingList::Library(identifier_path) = node {
            for identifier in identifier_path.identifiers {
                if identifier.name == SAFE_MATH {
                    using_safe_math = true;
                }
            }
        }
    }

    Ok(using_safe_math)
}

fn parse_contract_for_safe_math_functions(
    source_unit: &mut SourceUnit,
) -> Result<Vec<(Loc, String)>, EngineError> {
    let mut outcomes = vec![];
    let member_access_nodes = MemberAccessExtractor::extract(source_unit)?;
    for node in member_access_nodes {
        //if the function call identifier is a variable
        if let Expression::MemberAccess(_loc, _, identifier) = node.clone() {
            //if the identifier name is add, sub, mul or div
            if identifier.name == ADD
                || identifier.name == SUB
                || identifier.name == MUL
                || identifier.name == DIV
            {
                outcomes.push((node.loc(), node.to_string()))
            }
        }
    }

    Ok(outcomes)
}

mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_analyze_for_safe_math_pre_080() -> eyre::Result<()> {
        let file_contents = r#"
    
    pragma solidity >= 0.7.0;
    contract Contract {
        /// *** Libraries ***
        using SafeMath for uint256;
        using SafeMath for uint16;
    
       
        function testFunction(){
            uint256 something = 190092340923434;
            uint256 somethingElse = 1;

            uint256 thing = something.add(somethingElse);
            uint256 thing1 = something.sub(somethingElse);
            uint256 thing2 = something.mul(somethingElse);
            uint256 thing3 = something.div(somethingElse);

        }
    }
 
    "#;
        let mut source = MockSource::new().add_source("safe_math_pre_080.sol", file_contents);
        let optimization_locations = SafeMathPre080::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 4);

        Ok(())
    }

    #[test]
    fn test_analyze_for_safe_math_post_080() -> eyre::Result<()> {
        let file_contents = r#"
    
    pragma solidity >= 0.8.13;
    contract Contract {
        /// *** Libraries ***
        using SafeMath for uint256;
        using SafeMath for uint16;
    
       
        function testFunction(){
            uint256 something = 190092340923434;
            uint256 somethingElse = 1;

            uint256 thing = something.add(somethingElse);
            uint256 thing1 = something.sub(somethingElse);
            uint256 thing2 = something.mul(somethingElse);
            uint256 thing3 = something.div(somethingElse);

        }
    }
 
    "#;
        let mut source = MockSource::new().add_source("safe_math_post_080.sol", file_contents);
        let optimization_locations = SafeMathPost080::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 4);

        Ok(())
    }
}
