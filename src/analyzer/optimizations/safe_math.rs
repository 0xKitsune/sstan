use solang_parser::pt::{self, Expression, Loc, SourceUnit};
use std::collections::HashSet;

use crate::analyzer::extractors::{
    compound::SolidityVerisonExtractor,
    primitive::{MemberAccessExtractor, UsingListExtractor},
    Extractor,
};

pub const SAFE_MATH: &str = "SafeMath";
pub const ADD: &str = "add";
pub const DIV: &str = "div";
pub const SUB: &str = "sub";
pub const MUL: &str = "mul";

pub fn safe_math_pre_080_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    safe_math_optimization(source_unit, true)
}

pub fn safe_math_post_080_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    safe_math_optimization(source_unit, false)
}

pub fn safe_math_optimization(
    source_unit: &mut SourceUnit,
    pre_080: bool,
) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let solidity_versions = SolidityVerisonExtractor::extract(source_unit)?;

    for version in solidity_versions {
        if let Some(version) = version {
            if (pre_080 && version.minor < 8) || (!pre_080 && version.minor >= 8) {
                //if using safe math
                if check_if_using_safe_math(&mut source_unit.clone())? {
                    //get all locations that safe math functions are used
                    optimization_locations
                        .extend(parse_contract_for_safe_math_functions(source_unit)?);
                }
            }
        }
    }

    Ok(optimization_locations)
}

fn check_if_using_safe_math(source_unit: &mut SourceUnit) -> eyre::Result<bool> {
    let mut using_safe_math: bool = false;

    let using_list_nodes = UsingListExtractor::extract(source_unit)?;
    for node in using_list_nodes {
        if let pt::UsingList::Library(identifier_path) = node {
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
) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let member_access_nodes = MemberAccessExtractor::extract(source_unit)?;

    for node in member_access_nodes {
        //if the function call identifier is a variable
        if let Expression::MemberAccess(loc, _, identifier) = node {
            //if the identifier name is add, sub, mul or div
            if identifier.name == ADD
                || identifier.name == SUB
                || identifier.name == MUL
                || identifier.name == DIV
            {
                optimization_locations.insert(loc);
            }
        }
    }

    Ok(optimization_locations)
}

#[test]
fn test_analyze_for_safe_math_pre_080() {
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
    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = safe_math_pre_080_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 4);
}

#[test]
fn test_analyze_for_safe_math_post_080() {
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
    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = safe_math_post_080_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 4);
}
