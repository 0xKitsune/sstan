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

use self::{visitable::Visitable, visitor::Visitor};

pub mod compound;
pub mod primitive;
pub mod visitable;
pub mod visitor;
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
