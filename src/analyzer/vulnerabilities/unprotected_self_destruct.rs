use std::collections::HashSet;

use solang_parser::pt::{
    Base, Expression, FunctionAttribute, FunctionDefinition, FunctionTy, Identifier,
    IdentifierPath, Loc, SourceUnit, Visibility,
};

use crate::analyzer::extractors::{
    compound::ContractPartFunctionExtractor, primitive::FunctionCallExtractor, Extractor,
};

pub const SELF_DESTRUCT: &str = "selfdestruct";
pub const SUICIDE: &str = "suicide";
pub const ONLY: &str = "only";
pub const MSG: &str = "msg";
pub const SENDER: &str = "sender";

pub fn unprotected_self_destruct_vulnerability(
    source_unit: &mut SourceUnit,
) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each vulnerability target identified
    let mut vulnerability_locations: HashSet<Loc> = HashSet::new();

    let mut function_nodes = ContractPartFunctionExtractor::extract(source_unit)?;

    for node in function_nodes.iter_mut() {
        //If there is function body
        if node.body.is_some() {
            //Skip the constructor as it cannot be affected
            if node.ty == FunctionTy::Constructor {
                continue;
            }

            if !node.attributes.is_empty() {
                //Skip functions that are not public or external as they cannot be affected
                if !_is_public_or_external(node) {
                    continue;
                }

                let mut function_body_nodes = FunctionCallExtractor::extract(node)?;
                for function_body_node in function_body_nodes.iter_mut() {
                    if let Expression::FunctionCall(loc, box_identifier, ..) = function_body_node {
                        //If the function is a selfdestruct call
                        if is_self_destruct(*box_identifier.clone()) {
                            //Check if a function is protected using modifiers or conditions.
                            //This check is not exhaustive. For instance, it does not check if the modifier
                            //is implemented correctly. It only checks if the modifier name contains the word "only".
                            //Otherwise, it checks if there are any conditions on `msg.sender` applied.
                            if contains_protection_modifiers(node)
                                || contains_msg_sender_conditions(node)?
                            {
                                continue;
                            }

                            //If the function is not protected, add the loc of the
                            //selfdestruct call to the vulnerability_locations set.
                            vulnerability_locations.insert(*loc);
                        }
                    }
                }
            }
        }
    }

    //Return the identified vulnerability locations
    Ok(vulnerability_locations)
}

//Return true if the visibility of a given function is public or external. Return false otherwise.
fn _is_public_or_external(function_definition: &FunctionDefinition) -> bool {
    let mut public_or_external = false;
    if !function_definition.attributes.is_empty() {
        for attr in &function_definition.attributes {
            if let FunctionAttribute::Visibility(visibility) = attr {
                match visibility {
                    Visibility::External(_) => {
                        public_or_external = true;
                    }
                    Visibility::Public(_) => {
                        public_or_external = true;
                    }
                    _ => {}
                }
            }
        }
    }

    public_or_external
}

//Check if a given function's name is "selfdestruct" or "suicide"
fn is_self_destruct(box_identifier: Expression) -> bool {
    let mut is_selfdestruct = false;
    if let Expression::Variable(identifier) = box_identifier {
        //If the function name is "selfdestruct" or "suicide"
        if identifier.name == SELF_DESTRUCT || identifier.name == SUICIDE {
            is_selfdestruct = true;
        }
    }

    is_selfdestruct
}

//Check if a given function contains any modifier with "only" in its name
fn contains_protection_modifiers(function_definition: &FunctionDefinition) -> bool {
    //If the function has no arguments, early-return false
    if function_definition.attributes.is_empty() {
        return false;
    }

    for attr in &function_definition.attributes {
        //If the function has any modifier

        if let FunctionAttribute::BaseOrModifier(_, Base { name, .. }) = attr {
            let IdentifierPath { identifiers, .. } = name;

            for identifier in identifiers {
                //If the modifier name contains "only"
                if identifier.name.contains(ONLY) {
                    return true;
                }
            }
        }
    }

    false
}

//Check if there are any conditions applied on msg.sender
//examples: `require(msg.sender == owner)` or `check(msg.sender)`
fn contains_msg_sender_conditions(
    function_definition: &mut FunctionDefinition,
) -> eyre::Result<bool> {
    if let Some(ref mut statement) = function_definition.body {
        let function_body_nodes = FunctionCallExtractor::extract(statement)?;

        for node in function_body_nodes {
            //We can use unwrap because Target::MemberAccess is an expression
            if let Expression::FunctionCall(_, box_identifier, function_args) = node {
                //Skip if the function call is a selfdestruct, as it does not affect this vulnerability
                if is_self_destruct(*box_identifier) {
                    continue;
                }

                for expression in function_args {
                    match expression {
                        //Match for both `function(msg.sender == owner)` or `function(msg.sender != owner)`
                        Expression::Equal(_, box_expression, _)
                        | Expression::NotEqual(_, box_expression, _) => {
                            if let Expression::MemberAccess(_, box_expression, identifier) =
                                *box_expression
                            {
                                //If the member access identifier is "msg.sender"
                                let Identifier { name: right, .. } = identifier;
                                if let Expression::Variable(Identifier { name: left, .. }) =
                                    *box_expression
                                {
                                    if left == MSG && right == SENDER {
                                        return Ok(true);
                                    }
                                }
                            }
                        }

                        //Match for `function(msg.sender)`
                        Expression::MemberAccess(_, box_expression, identifier) => {
                            //If the member access identifier is "msg.sender"
                            let Identifier { name: right, .. } = identifier;
                            if let Expression::Variable(Identifier { name: left, .. }) =
                                *box_expression
                            {
                                if left == MSG && right == SENDER {
                                    return Ok(true);
                                }
                            }
                        }

                        _ => {}
                    };
                }
            }
        }
    }

    Ok(false)
}

#[test]
fn test_unprotected_selfdestruct_vulnerability() {
    let file_contents = r#"
    
    contract Contract0 {
        // unsafe
        function unprotectedKill() public {
            selfdestruct(msg.sender);
        }

        // unsafe
        function unprotectedKill2() external {
            suicide(owner);
        }

        // safe
        function protectedKill() public {
            require(msg.sender == owner);
            selfdestruct(msg.sender);
        }

        // safe
        function protectedKill2() public onlyOwner {
            selfdestruct(msg.sender);
        }

        // safe
        function internalKill() internal {
            selfdestruct(msg.sender);
        }
    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let vulnerability_locations = unprotected_self_destruct_vulnerability(&mut source_unit);
    assert_eq!(vulnerability_locations.unwrap().len(), 2)
}
