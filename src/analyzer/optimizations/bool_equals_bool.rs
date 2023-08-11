use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::ast::{self, Target};
use crate::analyzer::extractors::{primitive::EqualityExtractor, Extractor};

pub fn bool_equals_bool_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let equality_nodes = EqualityExtractor::extract(source_unit).unwrap();

    //For each target node that was extracted, check for the optimization patterns
    for node in equality_nodes {
        match node {
            pt::Expression::NotEqual(loc, box_expression, box_expression_1) => {
                if check_for_bool_equals_bool(*box_expression, *box_expression_1) {
                    optimization_locations.insert(loc);
                }
            }

            pt::Expression::Equal(loc, box_expression, box_expression_1) => {
                if check_for_bool_equals_bool(*box_expression, *box_expression_1) {
                    optimization_locations.insert(loc);
                }
            }

            _ => {}
        }
    }

    //Return the identified optimization locations
    Ok(optimization_locations)
}

fn check_for_bool_equals_bool(
    box_expression: pt::Expression,
    box_expression_1: pt::Expression,
) -> bool {
    //create a boolean to determine if address(0) is present
    let mut bool_equals_bool: bool = false;

    //if the first expression is true or false
    if let pt::Expression::BoolLiteral(_, _) = box_expression {
        bool_equals_bool = true;
    }
    //if the second expression is true or false
    if let pt::Expression::BoolLiteral(_, _) = box_expression_1 {
        bool_equals_bool = true;
    }

    //return true or false for bool equals bool
    bool_equals_bool
}

#[test]
fn test_analyze_for_if_bool_equals_bool_optimization() {
    let file_contents = r#"
    

    contract Contract0 {

        function boolEqualsBool0(bool check) public pure {
            if (check == true){
                return;
            }
        }


        function boolEqualsBool1(bool check) public pure {
            if (check == false){
                return;
            }
        }

        function boolEqualsBool2(bool check) public pure {
            if (false == check){
                return;
            }
        }

        function boolEqualsBool3(bool check) public pure {
            if (true == check){
                return;
            }
        }

        function boolEqualsBool4(bool check) public pure {
            if (check != true){
                return;
            }
        }


        function boolEqualsBool5(bool check) public pure {
            if (check != false){
                return;
            }
        }

        function boolEqualsBool6(bool check) public pure {
            if (false != check){
                return;
            }
        }

        function boolEqualsBool7(bool check) public pure {
            if (true != check){
                return;
            }
        }

    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = bool_equals_bool_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 8)
}
