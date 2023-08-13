use std::collections::{HashMap, HashSet};

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::compound::ConstructorExtractor;
use crate::analyzer::extractors::primitive::{
    AssignmentExtractor, FunctionExtractor, IncrementorExtractor,
};
use crate::analyzer::extractors::Extractor;
use crate::analyzer::utils::get_32_byte_storage_variables;

pub fn immutable_variables_optimization(
    source_unit: &mut SourceUnit,
) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Get all storage variables that are not marked constant or immutable
    let storage_variables = get_32_byte_storage_variables(source_unit, true, true);
    let mut potential_immutable_variables =
        get_storage_variables_assigned_in_constructor(&mut source_unit.clone(), storage_variables)?;
    let mut functions = FunctionExtractor::extract(source_unit).unwrap();
    for node in functions.iter_mut() {
        if let pt::FunctionTy::Constructor = node.ty {
        } else {
            let target_nodes = AssignmentExtractor::extract(node)
                .unwrap()
                .into_iter()
                .chain(IncrementorExtractor::extract(node).unwrap().into_iter())
                .collect::<Vec<pt::Expression>>();
            for target in target_nodes {
                match target {
                    pt::Expression::Assign(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::PreIncrement(_, box_expression) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::PostIncrement(_, box_expression) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::PreDecrement(_, box_expression) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::PostDecrement(_, box_expression) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }

                    pt::Expression::AssignAdd(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignAnd(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignDivide(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignModulo(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignMultiply(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignOr(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignShiftLeft(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignShiftRight(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignSubtract(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    pt::Expression::AssignXor(_, box_expression, _) => {
                        if let pt::Expression::Variable(identifier) = *box_expression {
                            //if the variable name exists in the storage variable hashmap
                            if potential_immutable_variables.contains_key(&identifier.name) {
                                //if the variable has been used, remove it from storage variables
                                potential_immutable_variables.remove(&identifier.name);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    //if the variable is not been reassigned, add it to the optimization locations
    for variable in potential_immutable_variables {
        optimization_locations.insert(variable.1);
    }

    //Return the identified optimization locations
    Ok(optimization_locations)
}

pub fn get_storage_variables_assigned_in_constructor(
    source_unit: &mut SourceUnit,
    storage_variables: HashMap<String, (Option<Vec<pt::VariableAttribute>>, Loc)>,
) -> eyre::Result<HashMap<String, Loc>> {
    let mut potential_immutable_variables: HashMap<String, Loc> = HashMap::new();
    let mut constructors = ConstructorExtractor::extract(source_unit)?;
    for constructor in constructors.iter_mut() {
        let assignment_nodes = AssignmentExtractor::extract(constructor)?;
        for node in assignment_nodes {
            if let pt::Expression::Assign(_, box_expression, box_assigned_value) = node {
                if is_a_non_value_type(*box_assigned_value) {
                    continue;
                }

                if let pt::Expression::Variable(identifier) = *box_expression {
                    //if the variable name exists in the storage variable hashmap
                    if storage_variables.contains_key(&identifier.name) {
                        let storage_var = storage_variables.get(&identifier.name);

                        if let Some(storage_var) = storage_var {
                            let loc = storage_var.1;
                            //add the variable to the variable usage map
                            potential_immutable_variables.insert(identifier.name, loc);
                        }
                    }
                }
            }
        }

        //
    }

    Ok(potential_immutable_variables)
}

fn is_a_non_value_type(assigned_value: pt::Expression) -> bool {
    match assigned_value {
        // string types
        pt::Expression::StringLiteral(_) => return true,
        // Dynamic bytes
        pt::Expression::FunctionCall(_, box_fn_call, _) => {
            // bytes (ex: bytes name = abi.encode("Vitalik"))
            if let pt::Expression::MemberAccess(_, box_member_access_variable, _) =
                *box_fn_call.clone()
            {
                if let pt::Expression::Variable(member_access_identifier) =
                    *box_member_access_variable
                {
                    return member_access_identifier.name == "abi";
                }
            }

            // bytes (ex: bytes name = bytes("Vitalik"))
            if let pt::Expression::Type(_, ty) = *box_fn_call {
                return pt::Type::DynamicBytes == ty;
            }
        }
        _ => (),
    }

    false
}

#[test]
fn test_immutable_variables_optimization() {
    let file_contents = r#"
    
    pragma solidity >= 0.8.0;
    contract Contract {


        uint256 immutable num0;
        uint256 num1;
        uint256 num2;
        address addr1 = address(0);
        string str1;
        string str2;
        bytes b1;
        bytes b2;
        bytes b3;


        constructor(){
            num1 = 100;
            num2 = 100;
            str1 = "Test Name";
            str2 = "Another test content";
            b1 = abi.encode("Test content");
            b2 = abi.encodePacked("Test content");
            b3 = bytes("Vitalik");
        }

       
        function testFunction() public {
            addr1 = address(0);
            uint256 thing = num1;
            str2 = "i can no longer be immutable anymore";
        }
    }
 
    "#;
    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = immutable_variables_optimization(&mut source_unit);
    assert_eq!(optimization_locations.unwrap().len(), 2)
}
