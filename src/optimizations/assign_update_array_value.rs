use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::helpers::CodeLocation;
use solang_parser::pt::Expression;
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::primitive::AssignmentExtractor;
use crate::extractors::Extractor;

use super::{AssignUpdateArrayValue, OptimizationOutcome, OptimizationPattern};

impl OptimizationPattern for AssignUpdateArrayValue {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            //Extract the target nodes from the source_unit
            let assignment_nodes = AssignmentExtractor::extract(source_unit).unwrap();
            //For each target node that was extracted, check for the optimization patterns
            for node in assignment_nodes {
                //We can use unwrap because Target::Assign is an expression
                if let Expression::Assign(_loc, box_expression, box_expression_1) = node.clone() {
                    if let Expression::ArraySubscript(
                        _,
                        array_subscript_box_expression,
                        option_array_subscript_box_expression_1,
                    ) = *box_expression
                    {
                        //get the variable name of the array
                        if let Expression::Variable(identifier) = *array_subscript_box_expression {
                            let array_identifier = identifier.name;

                            if let Some(array_subscript_box_expression) =
                                option_array_subscript_box_expression_1
                            {
                                if let Expression::NumberLiteral(_, number, _, _) =
                                    *array_subscript_box_expression
                                {
                                    let index_accessed = number;

                                    match *box_expression_1 {
                                        Expression::Add(_, _box_expression, _box_expression_1)
                                        | Expression::Subtract(
                                            _,
                                            _box_expression,
                                            _box_expression_1,
                                        )
                                        | Expression::Divide(
                                            _,
                                            _box_expression,
                                            _box_expression_1,
                                        )
                                        | Expression::Multiply(
                                            _,
                                            _box_expression,
                                            _box_expression_1,
                                        )
                                        | Expression::Modulo(
                                            _,
                                            _box_expression,
                                            _box_expression_1,
                                        )
                                        | Expression::ShiftLeft(
                                            _,
                                            _box_expression,
                                            _box_expression_1,
                                        )
                                        | Expression::ShiftRight(
                                            _,
                                            _box_expression,
                                            _box_expression_1,
                                        )
                                        | Expression::BitwiseAnd(
                                            _,
                                            _box_expression,
                                            _box_expression_1,
                                        )
                                        | Expression::BitwiseOr(
                                            _,
                                            _box_expression,
                                            _box_expression_1,
                                        )
                                        | Expression::BitwiseXor(
                                            _,
                                            _box_expression,
                                            _box_expression_1,
                                        ) => {
                                            if let Expression::ArraySubscript(
                                                _,
                                                array_subscript_box_expression,
                                                option_array_subscript_box_expression_1,
                                            ) = *_box_expression
                                            {
                                                //get the variable name of the array
                                                if let Expression::Variable(identifier) =
                                                    *array_subscript_box_expression
                                                {
                                                    let _array_identifier = identifier.name;
                                                    if _array_identifier == array_identifier {
                                                        if let Some(
                                                            array_subscript_box_expression_1,
                                                        ) =
                                                            option_array_subscript_box_expression_1
                                                        {
                                                            if let Expression::NumberLiteral(
                                                                _,
                                                                number,
                                                                _,
                                                                _,
                                                            ) = *array_subscript_box_expression_1
                                                            {
                                                                let _index_accessed = number;

                                                                if _index_accessed == index_accessed
                                                                {
                                                                    outcome.push_or_insert(
                                                                        path_buf.clone(),
                                                                        node.clone().loc(),
                                                                        node.to_string(),
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else if let Expression::ArraySubscript(
                                                    _,
                                                    array_subscript_box_expression,
                                                    option_array_subscript_box_expression_1,
                                                ) = *_box_expression_1
                                                {
                                                    //get the variable name of the array
                                                    if let Expression::Variable(identifier) =
                                                        *array_subscript_box_expression
                                                    {
                                                        let _array_identifier = identifier.name;
                                                        if array_identifier == _array_identifier {
                                                            if let Some(
                                                            array_subscript_box_expression_1,
                                                        ) =
                                                            option_array_subscript_box_expression_1
                                                        {
                                                            if let Expression::NumberLiteral(
                                                                _,
                                                                number,
                                                                _,
                                                                _,
                                                            ) = *array_subscript_box_expression_1
                                                            {
                                                                let _index_accessed = number;

                                                                if _index_accessed == index_accessed
                                                                {
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
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        //Return the identified optimization locations
        Ok(OptimizationOutcome::AssignUpdateArrayValue(outcome))
    }
}
mod test {
    use crate::{
        optimizations::{AssignUpdateArrayValue, OptimizationPattern},
        report::ReportSectionFragment,
        utils::MockSource,
    };
    use std::{fs::File, io::Write};

    #[test]
    fn test_assign_update_array_optimization() -> eyre::Result<()> {
        let file_contents = r#"
    
    pragma solidity >= 0.8.0;
    contract Contract {
        uint256[] vals;

        constructor(){
            vals = new uint256[](100);
        }
        function update() public {
            vals[0] = vals[0]+1;
            vals[0]+=1;
        }
    }
 
    "#;

        let mut mock_source =
            MockSource::new().add_source("assign_update_array_value.sol", file_contents);

        let optimization_locations = AssignUpdateArrayValue::find(&mut mock_source.source)?;
        assert_eq!(optimization_locations.len(), 1);
        let report: Option<ReportSectionFragment> = optimization_locations.into();
        if let Some(report) = report {
            let mut f = File::options()
                .append(true)
                .open("mocks/optimization_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }
        Ok(())
    }
}
