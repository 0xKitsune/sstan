use super::visitor::Visitor;
use solang_parser::pt::*;
/// All [`solang_parser::pt`] types, such as [Statement], should implement the [Visitable] trait
/// that accepts a trait [Visitor] implementation, which has various callback handles for Solidity
/// Parse Tree nodes.
///
/// We want to take a `&mut self` to be able to implement some advanced features in the future such
/// as modifying the Parse Tree before formatting it.
pub trait Visitable {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor;
}

impl<T> Visitable for &mut T
where
    T: Visitable,
{
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        T::visit(self, v)
    }
}

impl<T> Visitable for Option<T>
where
    T: Visitable,
{
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        if let Some(inner) = self.as_mut() {
            inner.visit(v)
        } else {
            Ok(())
        }
    }
}

impl<T> Visitable for Box<T>
where
    T: Visitable,
{
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        T::visit(self, v)
    }
}

impl<T> Visitable for Vec<T>
where
    T: Visitable,
{
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        for item in self.iter_mut() {
            item.visit(v)?;
        }
        Ok(())
    }
}

impl Visitable for SourceUnitPart {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            SourceUnitPart::ContractDefinition(contract) => v.visit_contract(contract),
            SourceUnitPart::PragmaDirective(loc, ident, str) => v.visit_pragma(*loc, ident, str),
            SourceUnitPart::ImportDirective(import) => import.visit(v),
            SourceUnitPart::EnumDefinition(enumeration) => v.visit_enum(enumeration),
            SourceUnitPart::StructDefinition(structure) => v.visit_struct(structure),
            SourceUnitPart::EventDefinition(event) => v.visit_event(event),
            SourceUnitPart::ErrorDefinition(error) => v.visit_error(error),
            SourceUnitPart::FunctionDefinition(function) => v.visit_function(function),
            SourceUnitPart::VariableDefinition(variable) => v.visit_var_definition(variable),
            SourceUnitPart::TypeDefinition(def) => v.visit_type_definition(def),
            SourceUnitPart::StraySemicolon(loc) => v.visit_stray_semicolon(*loc),
            SourceUnitPart::Using(using) => v.visit_using(using),
            SourceUnitPart::Annotation(annotation) => v.visit_annotation(annotation),
        }
    }
}

impl Visitable for Import {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            Import::Plain(import, loc) => v.visit_import_plain(*loc, import),
            Import::GlobalSymbol(global, import_as, loc) => {
                v.visit_import_global(*loc, global, import_as)
            }
            Import::Rename(from, imports, loc) => v.visit_import_renames(*loc, imports, from),
        }
    }
}

impl Visitable for ContractPart {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            ContractPart::StructDefinition(structure) => v.visit_struct(structure),
            ContractPart::EventDefinition(event) => v.visit_event(event),
            ContractPart::ErrorDefinition(error) => v.visit_error(error),
            ContractPart::EnumDefinition(enumeration) => v.visit_enum(enumeration),
            ContractPart::VariableDefinition(variable) => v.visit_var_definition(variable),
            ContractPart::FunctionDefinition(function) => v.visit_function(function),
            ContractPart::TypeDefinition(def) => v.visit_type_definition(def),
            ContractPart::StraySemicolon(loc) => v.visit_stray_semicolon(*loc),
            ContractPart::Using(using) => v.visit_using(using),
            ContractPart::Annotation(annotation) => v.visit_annotation(annotation),
        }
    }
}

impl Visitable for Statement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            Statement::Block {
                loc,
                unchecked,
                statements,
            } => v.visit_block(*loc, *unchecked, statements),
            Statement::Assembly {
                loc,
                dialect,
                block,
                flags,
            } => v.visit_assembly(*loc, dialect, block, flags),
            Statement::Args(loc, args) => v.visit_args(*loc, args),
            Statement::If(loc, cond, if_branch, else_branch) => {
                v.visit_if(*loc, cond, if_branch, else_branch, true)
            }
            Statement::While(loc, cond, body) => v.visit_while(*loc, cond, body),
            Statement::Expression(loc, expr) => {
                v.visit_expr(*loc, expr)?;
                v.visit_stray_semicolon(*loc)
            }
            Statement::VariableDefinition(loc, declaration, expr) => {
                v.visit_var_definition_stmt(*loc, declaration, expr)
            }
            Statement::For(loc, init, cond, update, body) => {
                v.visit_for(*loc, init, cond, update, body)
            }
            Statement::DoWhile(loc, body, cond) => v.visit_do_while(*loc, body, cond),
            Statement::Continue(loc) => v.visit_continue(*loc, true),
            Statement::Break(loc) => v.visit_break(*loc, true),
            Statement::Return(loc, expr) => v.visit_return(*loc, expr),
            Statement::Revert(loc, error, args) => v.visit_revert(*loc, error, args),
            Statement::RevertNamedArgs(loc, error, args) => {
                v.visit_revert_named_args(*loc, error, args)
            }
            Statement::Emit(loc, event) => v.visit_emit(*loc, event),
            Statement::Try(loc, expr, returns, clauses) => {
                v.visit_try(*loc, expr, returns, clauses)
            }
            Statement::Error(loc) => v.visit_parser_error(*loc),
        }
    }
}

impl Visitable for Expression {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        v.visit_expr(self.loc(), self)
    }
}

impl Visitable for Identifier {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        v.visit_ident(self.loc, self)
    }
}

impl Visitable for VariableDeclaration {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        v.visit_var_declaration(self)
    }
}

impl Visitable for YulBlock {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        v.visit_yul_block(self.loc, self.statements.as_mut(), false)
    }
}

impl Visitable for YulStatement {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        match self {
            YulStatement::Assign(loc, exprs, expr) => {
                v.visit_yul_assignment(*loc, exprs, &mut Some(expr))
            }
            YulStatement::Block(block) => {
                v.visit_yul_block(block.loc, block.statements.as_mut(), false)
            }
            YulStatement::Break(loc) => v.visit_break(*loc, false),
            YulStatement::Continue(loc) => v.visit_continue(*loc, false),
            YulStatement::For(stmt) => v.visit_yul_for(stmt),
            YulStatement::FunctionCall(stmt) => v.visit_yul_function_call(stmt),
            YulStatement::FunctionDefinition(stmt) => v.visit_yul_fun_def(stmt),
            YulStatement::If(loc, expr, block) => v.visit_yul_if(*loc, expr, block),
            YulStatement::Leave(loc) => v.visit_yul_leave(*loc),
            YulStatement::Switch(stmt) => v.visit_yul_switch(stmt),
            YulStatement::VariableDeclaration(loc, idents, expr) => {
                v.visit_yul_var_declaration(*loc, idents, expr)
            }
            YulStatement::Error(loc) => v.visit_parser_error(*loc),
        }
    }
}

impl Visitable for YulExpression {
    fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
    where
        V: Visitor,
    {
        v.visit_yul_expr(self.loc(), self)
    }
}

macro_rules! impl_visitable {
    ($type:ty, $func:ident) => {
        impl Visitable for $type {
            fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
            where
                V: Visitor,
            {
                v.$func(self)
            }
        }
    };
}

impl_visitable!(SourceUnit, visit_source_unit);
impl_visitable!(FunctionAttribute, visit_function_attribute);
impl_visitable!(VariableAttribute, visit_var_attribute);
impl_visitable!(Parameter, visit_parameter);
impl_visitable!(Base, visit_base);
impl_visitable!(EventParameter, visit_event_parameter);
impl_visitable!(ErrorParameter, visit_error_parameter);
impl_visitable!(IdentifierPath, visit_ident_path);
impl_visitable!(YulTypedIdentifier, visit_yul_typed_ident);
impl_visitable!(ContractDefinition, visit_contract);
impl_visitable!(FunctionDefinition, visit_function);
impl_visitable!(VariableDefinition, visit_var_definition);
