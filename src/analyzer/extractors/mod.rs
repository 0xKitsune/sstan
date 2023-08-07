use solang_parser::pt::{
    ContractDefinition, ContractPart, EventDefinition, FunctionAttribute, FunctionDefinition,
    FunctionTy, NamedArgument, SourceUnit, SourceUnitPart, Statement, StructDefinition,
    VariableDeclaration,
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
    NamedArgument,
    FunctionAttribute,
    FunctionTy,
    EventDefinition,
    StructDefinition,
    Vec<VariableDeclaration>,
    ContractDefinition,
    ContractPart
);
