use std::{collections::HashSet, vec};

use solang_parser::pt;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Target {
    //Statement Targets
    Args,
    Return,
    Revert,
    RevertNamedArgs,
    Emit,
    Expression,
    VariableDefinition,
    Block,
    If,
    While,
    For,
    DoWhile,
    Try,

    //Expression Targets
    Add,
    And,
    ArrayLiteral,
    ArraySlice,
    ArraySubscript,
    Assign,
    AssignAdd,
    AssignAnd,
    AssignDivide,
    AssignModulo,
    AssignMultiply,
    AssignOr,
    AssignShiftLeft,
    AssignShiftRight,
    AssignSubtract,
    AssignXor,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    Negate,
    Delete,
    Divide,
    Equal,
    FunctionCall,
    FunctionCallBlock,
    Less,
    LessEqual,
    List,
    MemberAccess,
    Modulo,
    More,
    MoreEqual,
    Multiply,
    NamedFunctionCall,
    New,
    Not,
    NotEqual,
    Or,
    Parenthesis,
    PostDecrement,
    PostIncrement,
    PreIncrement,
    PreDecrement,
    ShiftLeft,
    ShiftRight,
    Subtract,
    Type,
    Function,
    UnaryPlus,
    Power,
    BoolLiteral,
    NumberLiteral,
    RationalNumberLiteral,
    HexNumberLiteral,
    HexLiteral,
    StringLiteral,
    AddressLiteral,
    Variable,
    ConditionalOperator,

    //Source Unit / Contract Part
    SourceUnit,
    ContractDefinition,
    EnumDefinition,
    EventDefinition,
    ErrorDefinition,
    FunctionDefinition,
    ImportDirective,
    PragmaDirective,
    StraySemicolon,
    StructDefinition,
    TypeDefinition,
    Using,
    Annotation,

    //If there is no target that corresponds
    None,
}

pub fn new_targets(targets: Vec<Target>) -> HashSet<Target> {
    let mut target_set = HashSet::new();

    for target in targets {
        target_set.insert(target);
    }

    target_set
}

pub fn statement_as_target(statement: &pt::Statement) -> Target {
    match statement {
        pt::Statement::Args(_, _) => Target::Args,
        pt::Statement::Return(_, _) => Target::Return,
        pt::Statement::Revert(_, _, _) => Target::Revert,
        pt::Statement::Emit(_, _) => Target::Emit,
        pt::Statement::RevertNamedArgs(_, _, _) => Target::RevertNamedArgs,
        pt::Statement::Expression(_, _) => Target::Expression,
        pt::Statement::VariableDefinition(_, _, _) => Target::VariableDefinition,
        pt::Statement::Block {
            loc: _,
            unchecked: _,
            statements: _,
        } => Target::Block,
        pt::Statement::If(_, _, _, _) => Target::If,
        pt::Statement::While(_, _, _) => Target::While,
        pt::Statement::For(_, _, _, _, _) => Target::For,
        pt::Statement::DoWhile(_, _, _) => Target::DoWhile,
        pt::Statement::Try(_, _, _, _) => Target::Try,
        _ => Target::None,
    }
}

pub fn expression_as_target(expression: &pt::Expression) -> Target {
    match expression {
        pt::Expression::Add(_, _, _) => Target::Add,
        pt::Expression::And(_, _, _) => Target::And,
        pt::Expression::ArrayLiteral(_, _) => Target::ArrayLiteral,
        pt::Expression::ArraySlice(_, _, _, _) => Target::ArraySlice,
        pt::Expression::ArraySubscript(_, _, _) => Target::ArraySubscript,
        pt::Expression::Assign(_, _, _) => Target::Assign,
        pt::Expression::AssignAdd(_, _, _) => Target::AssignAdd,
        pt::Expression::AssignAnd(_, _, _) => Target::AssignAnd,
        pt::Expression::AssignDivide(_, _, _) => Target::AssignDivide,
        pt::Expression::AssignModulo(_, _, _) => Target::AssignModulo,
        pt::Expression::AssignMultiply(_, _, _) => Target::AssignMultiply,
        pt::Expression::AssignOr(_, _, _) => Target::AssignOr,
        pt::Expression::AssignShiftLeft(_, _, _) => Target::AssignShiftLeft,
        pt::Expression::AssignShiftRight(_, _, _) => Target::AssignShiftRight,
        pt::Expression::AssignSubtract(_, _, _) => Target::AssignSubtract,
        pt::Expression::AssignXor(_, _, _) => Target::AssignXor,
        pt::Expression::BitwiseAnd(_, _, _) => Target::BitwiseAnd,
        pt::Expression::BitwiseOr(_, _, _) => Target::BitwiseOr,
        pt::Expression::BitwiseXor(_, _, _) => Target::BitwiseXor,
        pt::Expression::Delete(_, _) => Target::Delete,
        pt::Expression::Divide(_, _, _) => Target::Divide,
        pt::Expression::Equal(_, _, _) => Target::Equal,
        pt::Expression::FunctionCall(_, _, _) => Target::FunctionCall,
        pt::Expression::FunctionCallBlock(_, _, _) => Target::FunctionCallBlock,
        pt::Expression::Less(_, _, _) => Target::Less,
        pt::Expression::LessEqual(_, _, _) => Target::LessEqual,
        pt::Expression::List(_, _) => Target::List,
        pt::Expression::MemberAccess(_, _, _) => Target::MemberAccess,
        pt::Expression::Modulo(_, _, _) => Target::Modulo,
        pt::Expression::More(_, _, _) => Target::More,
        pt::Expression::MoreEqual(_, _, _) => Target::MoreEqual,
        pt::Expression::Multiply(_, _, _) => Target::Multiply,
        pt::Expression::NamedFunctionCall(_, _, _) => Target::NamedFunctionCall,
        pt::Expression::New(_, _) => Target::New,
        pt::Expression::Not(_, _) => Target::Not,
        pt::Expression::NotEqual(_, _, _) => Target::NotEqual,
        pt::Expression::Or(_, _, _) => Target::Or,
        pt::Expression::Parenthesis(_, _) => Target::Parenthesis,
        pt::Expression::PostDecrement(_, _) => Target::PostDecrement,
        pt::Expression::PostIncrement(_, _) => Target::PostIncrement,
        pt::Expression::ShiftLeft(_, _, _) => Target::ShiftLeft,
        pt::Expression::ShiftRight(_, _, _) => Target::ShiftRight,
        pt::Expression::Subtract(_, _, _) => Target::Subtract,
        pt::Expression::Type(_, _) => Target::Type,
        pt::Expression::UnaryPlus(_, _) => Target::UnaryPlus,
        pt::Expression::PreIncrement(_, _) => Target::PreIncrement,
        pt::Expression::PreDecrement(_, _) => Target::PreDecrement,
        pt::Expression::Power(_, _, _) => Target::Power,
        pt::Expression::BoolLiteral(_, _) => Target::BoolLiteral,
        pt::Expression::NumberLiteral(_, _, _, _) => Target::NumberLiteral,
        pt::Expression::RationalNumberLiteral(_, _, _, _, _) => Target::RationalNumberLiteral,
        pt::Expression::HexNumberLiteral(_, _, _) => Target::HexNumberLiteral,
        pt::Expression::HexLiteral(_) => Target::HexLiteral,
        pt::Expression::StringLiteral(_) => Target::StringLiteral,
        pt::Expression::AddressLiteral(_, _) => Target::AddressLiteral,
        pt::Expression::Variable(_) => Target::Variable,
        pt::Expression::BitwiseNot(_, _) => Target::BitwiseNot,
        pt::Expression::Negate(_, _) => Target::Negate,
        pt::Expression::ConditionalOperator(_, _, _, _) => Target::ConditionalOperator,
    }
}

pub fn source_unit_part_as_target(source_unit_part: &pt::SourceUnitPart) -> Target {
    match source_unit_part {
        pt::SourceUnitPart::ContractDefinition(_) => Target::ContractDefinition,
        pt::SourceUnitPart::EnumDefinition(_) => Target::EnumDefinition,
        pt::SourceUnitPart::ErrorDefinition(_) => Target::ErrorDefinition,
        pt::SourceUnitPart::EventDefinition(_) => Target::EventDefinition,
        pt::SourceUnitPart::FunctionDefinition(_) => Target::FunctionDefinition,
        pt::SourceUnitPart::ImportDirective(_) => Target::ImportDirective,
        pt::SourceUnitPart::PragmaDirective(_, _, _) => Target::PragmaDirective,
        pt::SourceUnitPart::StraySemicolon(_) => Target::StraySemicolon,
        pt::SourceUnitPart::StructDefinition(_) => Target::StructDefinition,
        pt::SourceUnitPart::TypeDefinition(_) => Target::TypeDefinition,
        pt::SourceUnitPart::Using(_) => Target::Using,
        pt::SourceUnitPart::VariableDefinition(_) => Target::VariableDefinition,
        pt::SourceUnitPart::Annotation(_) => Target::Annotation,
    }
}
pub fn contract_part_as_target(contract_part: &pt::ContractPart) -> Target {
    match contract_part {
        pt::ContractPart::EnumDefinition(_) => Target::EnumDefinition,
        pt::ContractPart::ErrorDefinition(_) => Target::ErrorDefinition,
        pt::ContractPart::EventDefinition(_) => Target::EventDefinition,
        pt::ContractPart::FunctionDefinition(_) => Target::FunctionDefinition,
        pt::ContractPart::StraySemicolon(_) => Target::StraySemicolon,
        pt::ContractPart::StructDefinition(_) => Target::StructDefinition,
        pt::ContractPart::TypeDefinition(_) => Target::TypeDefinition,
        pt::ContractPart::Using(_) => Target::Using,
        pt::ContractPart::VariableDefinition(_) => Target::VariableDefinition,
        pt::ContractPart::Annotation(_) => Target::Annotation,
    }
}

pub fn extract_target_from_node(target: Target, node: Node) -> Vec<Node> {
    let mut target_set = HashSet::new();
    target_set.insert(target);

    walk_node_for_targets(&target_set, node)
}

pub fn extract_targets_from_node(targets: Vec<Target>, node: Node) -> Vec<Node> {
    let mut target_set = HashSet::new();

    for target in targets {
        target_set.insert(target);
    }

    walk_node_for_targets(&target_set, node)
}

//Extract target ast node types from a parent node
pub fn walk_node_for_targets(targets: &HashSet<Target>, node: Node) -> Vec<Node> {
    let mut matches = vec![];

    if targets.contains(&node.as_target()) {
        matches.push(node.clone());
    }

    match node {
        Node::SourceUnit(source_unit) => {
            for source_unit_part in source_unit.0 {
                matches.append(&mut walk_node_for_targets(targets, source_unit_part.into()));
            }
        }
        Node::SourceUnitPart(source_unit_part) => match source_unit_part {
            pt::SourceUnitPart::ContractDefinition(box_contract_definition) => {
                //Walk the contract definition base for targets
                for base in box_contract_definition.base {
                    if let Some(args) = base.args {
                        for arg in args {
                            matches.append(&mut walk_node_for_targets(targets, arg.into()));
                        }
                    }
                }

                //Walk the contract definition parts for targets
                for part in box_contract_definition.parts {
                    matches.append(&mut walk_node_for_targets(targets, part.into()));
                }
            }

            pt::SourceUnitPart::ErrorDefinition(box_error_definition) => {
                for error_parameter in box_error_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        error_parameter.ty.into(),
                    ));
                }
            }

            pt::SourceUnitPart::EventDefinition(box_event_definition) => {
                for event_parameter in box_event_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        event_parameter.ty.into(),
                    ));
                }
            }

            pt::SourceUnitPart::FunctionDefinition(box_function_definition) => {
                //Walk params for targets
                for (_, option_parameter) in box_function_definition.params {
                    if let Some(parameter) = option_parameter {
                        matches.append(&mut walk_node_for_targets(targets, parameter.ty.into()));
                    }
                }
                //Walk return params for targets
                for (_, option_parameter) in box_function_definition.returns {
                    if let Some(parameter) = option_parameter {
                        matches.append(&mut walk_node_for_targets(targets, parameter.ty.into()));
                    }
                }

                //Walk the function body for targets
                if let Some(body) = box_function_definition.body {
                    matches.append(&mut walk_node_for_targets(targets, body.into()));
                }
            }

            pt::SourceUnitPart::StructDefinition(box_struct_definition) => {
                for variable_declaration in box_struct_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        variable_declaration.ty.into(),
                    ));
                }
            }

            pt::SourceUnitPart::TypeDefinition(box_type_definition) => {
                matches.append(&mut walk_node_for_targets(
                    targets,
                    box_type_definition.ty.into(),
                ));
            }

            pt::SourceUnitPart::Using(box_using) => {
                if let Some(ty) = box_using.ty {
                    matches.append(&mut walk_node_for_targets(targets, ty.into()));
                }
            }
            pt::SourceUnitPart::VariableDefinition(box_variable_definition) => {
                matches.append(&mut walk_node_for_targets(
                    targets,
                    box_variable_definition.ty.into(),
                ));

                if let Some(initializer) = box_variable_definition.initializer {
                    matches.append(&mut walk_node_for_targets(targets, initializer.into()));
                }
            }

            _ => {
                //Pragma Directive
                //Stray Semicolon
                //EnumDefinition
                //Import directive
            }
        },

        Node::ContractPart(contract_part) => match contract_part {
            pt::ContractPart::ErrorDefinition(box_error_definition) => {
                for error_parameter in box_error_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        error_parameter.ty.into(),
                    ));
                }
            }

            pt::ContractPart::EventDefinition(box_event_definition) => {
                for event_parameter in box_event_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        event_parameter.ty.into(),
                    ));
                }
            }

            pt::ContractPart::FunctionDefinition(box_function_definition) => {
                //Walk params for targets
                for (_, option_parameter) in box_function_definition.params {
                    if let Some(parameter) = option_parameter {
                        matches.append(&mut walk_node_for_targets(targets, parameter.ty.into()));
                    }
                }
                //Walk return params for targets
                for (_, option_parameter) in box_function_definition.returns {
                    if let Some(parameter) = option_parameter {
                        matches.append(&mut walk_node_for_targets(targets, parameter.ty.into()));
                    }
                }

                //Walk the function body for targets
                if let Some(body) = box_function_definition.body {
                    matches.append(&mut walk_node_for_targets(targets, body.into()));
                }
            }

            pt::ContractPart::StructDefinition(box_struct_definition) => {
                for variable_declaration in box_struct_definition.fields {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        variable_declaration.ty.into(),
                    ));
                }
            }

            pt::ContractPart::TypeDefinition(box_type_definition) => {
                matches.append(&mut walk_node_for_targets(
                    targets,
                    box_type_definition.ty.into(),
                ));
            }

            pt::ContractPart::Using(box_using) => {
                if let Some(ty) = box_using.ty {
                    matches.append(&mut walk_node_for_targets(targets, ty.into()));
                }
            }

            pt::ContractPart::VariableDefinition(box_variable_definition) => {
                matches.append(&mut walk_node_for_targets(
                    targets,
                    box_variable_definition.ty.into(),
                ));

                if let Some(initializer) = box_variable_definition.initializer {
                    matches.append(&mut walk_node_for_targets(targets, initializer.into()));
                }
            }

            _ => {
                //Stray Semicolon
                //EnumDefinition
            }
        },

        Node::Statement(statement) => match statement {
            pt::Statement::Args(_, named_arguments) => {
                for argument in named_arguments {
                    matches.append(&mut walk_node_for_targets(targets, argument.expr.into()));
                }
            }

            pt::Statement::Return(_, Some(expression)) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));
            }

            pt::Statement::Revert(_, _, vec_expression) => {
                for expression in vec_expression {
                    matches.append(&mut walk_node_for_targets(targets, expression.into()));
                }
            }

            pt::Statement::Emit(_, expression) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));
            }

            pt::Statement::RevertNamedArgs(_, _, vec_named_arguments) => {
                for named_argument in vec_named_arguments {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        named_argument.expr.into(),
                    ));
                }
            }

            pt::Statement::Expression(_, expression) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));
            }

            pt::Statement::VariableDefinition(_, variable_declaration, option_expression) => {
                matches.append(&mut walk_node_for_targets(
                    targets,
                    variable_declaration.ty.into(),
                ));

                if let Some(expression) = option_expression {
                    matches.append(&mut walk_node_for_targets(targets, expression.into()));
                }
            }

            pt::Statement::Block {
                loc: _,
                unchecked: _,
                statements,
            } => {
                for statement in statements {
                    matches.append(&mut walk_node_for_targets(targets, statement.into()));
                }
            }

            pt::Statement::If(_, expression, box_statement, option_box_statement) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_statement.into()));

                if let Some(box_statement) = option_box_statement {
                    matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
                }
            }

            pt::Statement::While(_, expression, box_statement) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
            }

            pt::Statement::For(
                _,
                option_box_statement,
                option_box_expression,
                option_box_statement_1,
                option_box_statement_2,
            ) => {
                if let Some(box_statement) = option_box_statement {
                    matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
                }

                if let Some(box_expression) = option_box_expression {
                    matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                }

                if let Some(box_statement) = option_box_statement_1 {
                    matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
                }

                if let Some(box_statement) = option_box_statement_2 {
                    matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
                }
            }

            pt::Statement::DoWhile(_, box_statement, expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
                matches.append(&mut walk_node_for_targets(targets, expression.into()));
            }

            pt::Statement::Try(_, expression, option_paramlist_box_statement, _) => {
                matches.append(&mut walk_node_for_targets(targets, expression.into()));

                if let Some((param_list, box_statement)) = option_paramlist_box_statement {
                    for (_, option_param) in param_list {
                        if let Some(param) = option_param {
                            matches.append(&mut walk_node_for_targets(targets, param.ty.into()));
                        }
                    }

                    matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
                }
            }

            _ => {
                //Assembly block
                //Continue
                //Break
            }
        },

        Node::Expression(expression) => match expression {
            pt::Expression::Add(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::And(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::ArrayLiteral(_, vec_expression) => {
                for expression in vec_expression {
                    matches.append(&mut walk_node_for_targets(targets, expression.into()));
                }
            }

            pt::Expression::ArraySlice(
                _,
                box_expression,
                option_box_expression,
                option_box_expression_1,
            ) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                if let Some(box_expression) = option_box_expression {
                    matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                }

                if let Some(box_expression) = option_box_expression_1 {
                    matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                }
            }
            pt::Expression::ArraySubscript(_, box_expression, option_box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                if let Some(box_expression) = option_box_expression {
                    matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                }
            }

            pt::Expression::Assign(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignAdd(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }
            pt::Expression::AssignAnd(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignDivide(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignModulo(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignMultiply(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignOr(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }
            pt::Expression::AssignShiftLeft(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignShiftRight(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::AssignSubtract(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }
            pt::Expression::AssignXor(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::BitwiseAnd(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::BitwiseOr(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::BitwiseXor(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Delete(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }
            pt::Expression::Divide(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Equal(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::FunctionCall(_, box_expression, vec_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                for expression in vec_expression {
                    matches.append(&mut walk_node_for_targets(targets, expression.into()));
                }
            }

            pt::Expression::FunctionCallBlock(_, box_expression, box_statement) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                matches.append(&mut walk_node_for_targets(targets, box_statement.into()));
            }

            pt::Expression::Less(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::LessEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::List(_, parameter_list) => {
                for (_, option_parameter) in parameter_list {
                    if let Some(parameter) = option_parameter {
                        matches.append(&mut walk_node_for_targets(targets, parameter.ty.into()));
                    }
                }
            }

            pt::Expression::MemberAccess(_, box_expression, _) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            pt::Expression::Modulo(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::More(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }
            pt::Expression::MoreEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Multiply(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::NamedFunctionCall(_, box_expression, vec_named_argument) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));

                for named_argument in vec_named_argument {
                    matches.append(&mut walk_node_for_targets(
                        targets,
                        named_argument.expr.into(),
                    ));
                }
            }

            pt::Expression::New(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }
            pt::Expression::Not(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }
            pt::Expression::NotEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Or(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Parenthesis(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            pt::Expression::PostDecrement(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            pt::Expression::PostIncrement(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            pt::Expression::ShiftLeft(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::ShiftRight(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }
            pt::Expression::Subtract(_, box_expression, box_expression_1) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
                matches.append(&mut walk_node_for_targets(targets, box_expression_1.into()));
            }

            pt::Expression::Type(_, ty) => match ty {
                pt::Type::Mapping { key, value, .. } => {
                    matches.append(&mut walk_node_for_targets(targets, key.into()));

                    matches.append(&mut walk_node_for_targets(targets, value.into()));
                }

                pt::Type::Function {
                    params,
                    attributes,
                    returns,
                } => {
                    for param in params {
                        if let Some(param) = param.1 {
                            matches.append(&mut walk_node_for_targets(targets, param.ty.into()));
                        }
                    }

                    for attribute in attributes {
                        match attribute {
                            pt::FunctionAttribute::BaseOrModifier(_, base) => {
                                if let Some(args) = base.args {
                                    for arg in args {
                                        matches.append(&mut walk_node_for_targets(
                                            targets,
                                            arg.into(),
                                        ));
                                    }
                                }
                            }

                            _ => {}
                        }
                    }

                    if let Some((parameter_list, function_attributes)) = returns {
                        for (_, option_parameter) in parameter_list {
                            if let Some(parameter) = option_parameter {
                                matches.append(&mut walk_node_for_targets(
                                    targets,
                                    parameter.ty.into(),
                                ));
                            }
                        }

                        for attribute in function_attributes {
                            match attribute {
                                pt::FunctionAttribute::BaseOrModifier(_, base) => {
                                    if let Some(args) = base.args {
                                        for arg in args {
                                            matches.append(&mut walk_node_for_targets(
                                                targets,
                                                arg.into(),
                                            ));
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                }

                _ => {}
            },

            pt::Expression::UnaryPlus(_, box_expression) => {
                matches.append(&mut walk_node_for_targets(targets, box_expression.into()));
            }

            _ => {

                //TODO: FIXME: add the new ones

                //Address literal
                //Bool literal
                //Hex literal
                //Hex number literal
                //Number literal
                // Rational number literal
                //String literal
                //This
                //Variable
            }
        },
    }

    matches
}

impl Node {
    pub fn as_target(&self) -> Target {
        match self {
            Self::Expression(expression) => expression_as_target(expression),
            Self::Statement(statement) => statement_as_target(statement),
            Self::SourceUnit(_) => Target::SourceUnit,
            Self::SourceUnitPart(source_unit_part) => source_unit_part_as_target(source_unit_part),
            Self::ContractPart(contract_part) => contract_part_as_target(contract_part),
        }
    }

    pub fn expression(self) -> Option<pt::Expression> {
        match self {
            Self::Expression(expression) => Some(expression),
            _ => None,
        }
    }

    pub fn statement(self) -> Option<pt::Statement> {
        match self {
            Self::Statement(statement) => Some(statement),
            _ => None,
        }
    }

    pub fn source_unit(self) -> Option<pt::SourceUnit> {
        match self {
            Self::SourceUnit(source_unit) => Some(source_unit),
            _ => None,
        }
    }

    pub fn source_unit_part(self) -> Option<pt::SourceUnitPart> {
        match self {
            Self::SourceUnitPart(source_unit_part) => Some(source_unit_part),
            _ => None,
        }
    }

    pub fn is_source_unit_part(&self) -> bool {
        matches!(self, Self::SourceUnitPart(_))
    }

    pub fn contract_part(self) -> Option<pt::ContractPart> {
        match self {
            Self::ContractPart(contract_part) => Some(contract_part),
            _ => None,
        }
    }

    pub fn is_contract_part(&self) -> bool {
        matches!(self, Self::ContractPart(_))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Node {
    Statement(pt::Statement),
    Expression(pt::Expression),
    SourceUnit(pt::SourceUnit),
    SourceUnitPart(pt::SourceUnitPart),
    ContractPart(pt::ContractPart),
}

impl From<pt::Statement> for Node {
    fn from(val: pt::Statement) -> Self {
        Node::Statement(val)
    }
}

impl From<Box<pt::Statement>> for Node {
    fn from(val: Box<pt::Statement>) -> Self {
        Node::Statement(*val)
    }
}

impl From<pt::Expression> for Node {
    fn from(val: pt::Expression) -> Self {
        Node::Expression(val)
    }
}
impl From<Box<pt::Expression>> for Node {
    fn from(val: Box<pt::Expression>) -> Self {
        Node::Expression(*val)
    }
}

impl From<pt::ContractPart> for Node {
    fn from(val: pt::ContractPart) -> Self {
        Node::ContractPart(val)
    }
}

impl From<pt::SourceUnitPart> for Node {
    fn from(val: pt::SourceUnitPart) -> Self {
        Node::SourceUnitPart(val)
    }
}

impl From<pt::SourceUnit> for Node {
    fn from(val: pt::SourceUnit) -> Self {
        Node::SourceUnit(val)
    }
}
