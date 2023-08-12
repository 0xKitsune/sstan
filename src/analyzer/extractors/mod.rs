pub mod compound;
pub mod primitive;
mod visitable;
mod visitor;
use self::{visitable::Visitable, visitor::Visitor};
use solang_parser::pt::{
    Annotation, CatchClause, ContractDefinition, ContractPart, ContractTy, EnumDefinition,
    ErrorDefinition, ErrorParameter, EventDefinition, EventParameter, Expression,
    FunctionAttribute, FunctionDefinition, FunctionTy, HexLiteral, Identifier, IdentifierPath,
    Import, Loc, NamedArgument, Parameter, ParameterList, SourceUnit, SourceUnitPart, Statement,
    StorageLocation, StringLiteral, StructDefinition, Type, TypeDefinition, UserDefinedOperator,
    Using, UsingFunction, UsingList, VariableAttribute, VariableDeclaration, VariableDefinition,
    Visibility, YulBlock, YulExpression, YulFunctionCall, YulFunctionDefinition, YulStatement,
    YulSwitch, YulSwitchOptions, YulTypedIdentifier,
};
use thiserror::Error;

pub trait Extractor<V, T>: Visitor
where
    V: Visitable,
    T: Target,
{
    fn extract(v: &mut V) -> Result<Vec<T>, Self::Error>;
}

//TODO: this is just a placeholder, we will need to update this
#[derive(Error, Debug)]
pub enum ExtractionError {
    // #[error("Error while extracting target")]
    // Example error ,
}

pub trait Target {}

macro_rules! impl_target {
    ($($t:ty),*) => {
       $(
           impl Target for $t {}
       )*
    }
}

impl_target!(
    SourceUnit,
    SourceUnitPart,
    FunctionDefinition,
    Statement,
    Expression,
    Box<Expression>,
    ParameterList,
    Vec<FunctionAttribute>,
    ErrorDefinition,
    NamedArgument,
    FunctionAttribute,
    FunctionTy,
    EventDefinition,
    StructDefinition,
    Vec<VariableDeclaration>,
    ContractDefinition,
    ContractPart,
    ContractTy,
    Import,
    EnumDefinition,
    Annotation,
    Identifier,
    StringLiteral,
    YulBlock,
    Vec<StringLiteral>,
    Parameter,
    Loc,
    Vec<Expression>,
    HexLiteral,
    Type,
    Box<Statement>,
    VariableDefinition,
    StorageLocation,
    IdentifierPath,
    CatchClause,
    VariableAttribute,
    Visibility,
    EventParameter,
    ErrorParameter,
    TypeDefinition,
    Using,
    UsingList,
    UsingFunction,
    UserDefinedOperator,
    YulStatement,
    Vec<YulStatement>,
    YulExpression,
    Vec<YulExpression>,
    YulFunctionDefinition,
    YulFunctionCall,
    YulSwitch,
    YulSwitchOptions,
    YulTypedIdentifier
);

/// Macro that defines a new extractor struct and implements the Extractor trait for it.
/// The second argument defines the target type that the extractor will extract.
#[macro_export]
macro_rules! default_extractor {
    ($extractor_name:ident, $target_type:ty) => {
        pub struct $extractor_name {
            targets: Vec<$target_type>,
        }

        impl $extractor_name {
            pub fn new() -> Self {
                Self { targets: vec![] }
            }
        }

        impl<V: Visitable> Extractor<V, $target_type> for $extractor_name {
            fn extract(v: &mut V) -> Result<Vec<$target_type>, ExtractionError> {
                let mut extractor_instance = Self::new();
                v.visit(&mut extractor_instance)?;
                Ok(extractor_instance.targets)
            }
        }
    };
}

#[macro_export]
macro_rules! compound_extractor {
    ($extractor_name:ident, $target_type:ty) => {
        pub struct $extractor_name {}

        impl Visitor for $extractor_name {
            type Error = ExtractionError;
        }
    };
}
