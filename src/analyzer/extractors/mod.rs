use solang_parser::pt::{
    ContractDefinition, ContractPart, EventDefinition, FunctionAttribute, FunctionDefinition,
    FunctionTy, NamedArgument, SourceUnit, SourceUnitPart, Statement, StructDefinition,
    VariableDeclaration, Expression, ParameterList, ErrorDefinition, ContractTy, Import, EnumDefinition, Annotation, Identifier, StringLiteral, YulBlock, Parameter, Loc, HexLiteral, Type, VariableDefinition, StorageLocation, IdentifierPath, CatchClause, VariableAttribute, Visibility, EventParameter, ErrorParameter, TypeDefinition, Using, UsingList, UsingFunction, UserDefinedOperator, YulStatement, YulExpression, YulFunctionDefinition, YulFunctionCall, YulSwitch, YulSwitchOptions, YulTypedIdentifier,
};

use self::visit::Visitor;

pub mod compound;
pub mod primitive;
pub mod visit;

pub trait Extractor<V, T>: Visitor
where
    V: Target,
    T: Target,
{
    fn extract(v: V) -> Vec<T>;
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
