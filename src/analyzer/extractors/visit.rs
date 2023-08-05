use solang_parser::pt::*;

/// A trait that is invoked while traversing the Solidity Parse Tree.
/// Each method of the [Visitor] trait is a hook that can be potentially overridden.
///
/// Currently the main implementor of this trait is the [`Formatter`](crate::Formatter) struct.
pub trait Visitor {
    type Error: std::error::Error;

    fn visit_source_unit(&mut self, source_unit: &mut SourceUnit) -> Result<(), Self::Error> {
        for part in &mut source_unit.0 {
            self.visit_source_unit_part(part)?;
        }

        Ok(())
    }

    fn visit_source_unit_part(
        &mut self,
        source_unit_part: &mut SourceUnitPart,
    ) -> Result<(), Self::Error> {
        match source_unit_part {
            SourceUnitPart::ContractDefinition(contract) => self.visit_contract(contract)?,
            SourceUnitPart::PragmaDirective(loc, ident, str) => {
                self.visit_pragma(*loc, ident, str)?
            }
            SourceUnitPart::ImportDirective(import) => self.visit_import(import)?,
            SourceUnitPart::EnumDefinition(enumeration) => self.visit_enum(enumeration)?,
            SourceUnitPart::StructDefinition(structure) => self.visit_struct(structure)?,
            SourceUnitPart::EventDefinition(event) => self.visit_event(event)?,
            SourceUnitPart::ErrorDefinition(error) => self.visit_error(error)?,
            SourceUnitPart::FunctionDefinition(function) => self.visit_function(function)?,
            SourceUnitPart::VariableDefinition(variable) => self.visit_var_definition(variable)?,
            SourceUnitPart::TypeDefinition(def) => self.visit_type_definition(def)?,
            SourceUnitPart::StraySemicolon(loc) => self.visit_stray_semicolon(*loc)?,
            SourceUnitPart::Using(using) => self.visit_using(using)?,
            SourceUnitPart::Annotation(annotation) => self.visit_annotation(annotation)?,
        }
        Ok(())
    }

    fn visit_function(&mut self, function: &mut FunctionDefinition) -> Result<(), Self::Error> {
        self.visit_function_type(&mut function.ty)?;
        if let Some(ref mut identifier) = function.name {
            self.visit_ident(identifier.loc, identifier)?;
        }

        self.visit_parameter_list(&mut function.params)?;

        for attribute in function.attributes.iter_mut() {
            self.visit_function_attribute(attribute)?;
        }

        if let Some(ref mut statement) = function.body {
            self.visit_statement(statement)?;
        }

        self.visit_parameter_list(&mut function.returns)?;

        Ok(())
    }

    fn visit_statement(&mut self, statement: &mut Statement) -> Result<(), Self::Error> {
        match statement {
            Statement::Block {
                loc,
                unchecked,
                statements,
            } => self.visit_block(*loc, *unchecked, statements)?,
            Statement::Assembly {
                loc,
                dialect,
                flags,
                block,
            } => self.visit_assembly(*loc, dialect, block, flags)?,
            Statement::Args(loc, args) => self.visit_args(*loc, args)?,
            Statement::If(loc, expr, _if, else_) => self.visit_if(*loc, expr, _if, else_, true)?, //TODO: Revisit this
            Statement::While(loc, expr, body) => self.visit_while(*loc, expr, body)?,
            Statement::Expression(loc, expr) => self.visit_expr(*loc, expr)?,
            Statement::VariableDefinition(loc, var, expr) => {
                self.visit_var_declaration(var)?;
                if let Some(expr) = expr {
                    self.visit_expr(expr.loc(), expr)?;
                }
            }
            Statement::For(loc, init, cond, next, body) => {
                self.visit_for(*loc, init, cond, next, body)?
            }
            Statement::DoWhile(loc, body, cond) => self.visit_do_while(*loc, body, cond)?,
            Statement::Continue(loc) => self.visit_continue(*loc, true)?,
            Statement::Break(loc) => self.visit_break(*loc, true)?,
            Statement::Return(loc, expr) => self.visit_return(*loc, expr)?,
            Statement::Revert(loc, path, args) => self.visit_revert(*loc, path, args)?,
            Statement::RevertNamedArgs(loc, path, args) => {
                self.visit_revert_named_args(*loc, path, args)?
            }
            Statement::Emit(loc, expr) => self.visit_emit(*loc, expr)?,
            Statement::Try(loc, expr, returns, catches) => {
                self.visit_try(*loc, expr, returns, catches)?
            }
            Statement::Error(loc) => self.visit_parser_error(*loc)?,
        }

        Ok(())
    }

    fn visit_named_arg(&mut self, arg: &mut NamedArgument) -> Result<(), Self::Error> {
        self.visit_expr(arg.expr.loc(), &mut arg.expr)?;
        self.visit_ident(arg.name.loc, &mut arg.name)?;
        Ok(())
    }

    fn visit_returns(&mut self, returns: &mut ParameterList) -> Result<(), Self::Error> {
        for parameter in returns {
            if let Some(ref mut param) = parameter.1 {
                self.visit_return_parameter(param)?;
            }
        }

        Ok(())
    }

    fn visit_return_parameter(
        &mut self,
        _return_parameter: &mut Parameter,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_function_attributes(
        &mut self,
        _attributes: &mut Vec<FunctionAttribute>,
    ) -> Result<(), Self::Error> {
        for attribute in _attributes {
            self.visit_function_attribute(attribute)?;
        }
        Ok(())
    }

    fn visit_function_attribute(
        &mut self,
        _attribute: &mut FunctionAttribute,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    //pub type ParameterList = Vec<(Loc, Option<Parameter>)>;
    fn visit_parameter_list(
        &mut self,
        parameter_list: &mut ParameterList,
    ) -> Result<(), Self::Error> {
        for parameter in parameter_list {
            if let Some(ref mut param) = parameter.1 {
                self.visit_parameter(param)?;
            }
        }

        Ok(())
    }

    fn visit_function_type(&mut self, _ty: &mut FunctionTy) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_error(&mut self, error: &mut ErrorDefinition) -> Result<(), Self::Error> {
        self.visit_expr(error.keyword.loc(), &mut error.keyword)?;

        if let Some(ref mut identifier) = error.name {
            self.visit_ident(identifier.loc, identifier)?;
        }

        for error_parameter in error.fields.iter_mut() {
            self.visit_error_parameter(error_parameter)?;
        }
        Ok(())
    }

    fn visit_event(&mut self, event: &mut EventDefinition) -> Result<(), Self::Error> {
        if let Some(ref mut identifier) = event.name {
            self.visit_ident(identifier.loc, identifier)?;
        }
        for event_parameter in event.fields.iter_mut() {
            self.visit_event_parameter(event_parameter)?;
        }
        Ok(())
    }

    fn visit_struct(&mut self, structure: &mut StructDefinition) -> Result<(), Self::Error> {
        if let Some(ident) = structure.name.as_mut() {
            self.visit_ident(ident.loc, ident)?;
        }
        for var_declaration in structure.fields.iter_mut() {
            self.visit_var_declaration(var_declaration)?;
        }
        Ok(())
    }

    fn visit_fields(&mut self, _fields: &mut Vec<VariableDeclaration>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_contract(&mut self, contract: &mut ContractDefinition) -> Result<(), Self::Error> {
        self.visit_contract_type(&mut contract.ty)?;
        if let Some(ref mut identifier) = contract.name {
            self.visit_ident(identifier.loc, identifier)?;
        }

        for base in contract.base.iter_mut() {
            self.visit_base(base)?;
        }

        for part in contract.parts.iter_mut() {
            self.visit_contract_part(part)?;
        }
        Ok(())
    }

    fn visit_contract_part(&mut self, contract_part: &mut ContractPart) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_contract_type(&mut self, _ty: &mut ContractTy) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_import(&mut self, import: &mut Import) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_enum(&mut self, enum_definition: &mut Box<EnumDefinition>) -> Result<(), Self::Error> {
        if let Some(ref mut identifier) = enum_definition.name {
            self.visit_ident(identifier.loc, identifier)?;
        }

        for value in enum_definition.values.clone() {
            if let Some(mut value) = value {
                self.visit_ident(value.loc, &mut value)?;
            }
        }

        Ok(())
    }

    fn visit_annotation(&mut self, annotation: &mut Annotation) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_pragma(
        &mut self,
        loc: Loc,
        _ident: &mut Option<Identifier>,
        _str: &mut Option<StringLiteral>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_import_plain(
        &mut self,
        _loc: Loc,
        _import: &mut StringLiteral,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_import_global(
        &mut self,
        _loc: Loc,
        _global: &mut StringLiteral,
        _alias: &mut Identifier,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_import_renames(
        &mut self,
        _loc: Loc,
        _imports: &mut [(Identifier, Option<Identifier>)],
        _from: &mut StringLiteral,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_assembly(
        &mut self,
        loc: Loc,
        _dialect: &mut Option<StringLiteral>,
        _block: &mut YulBlock,
        _flags: &mut Option<Vec<StringLiteral>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_block(
        &mut self,
        loc: Loc,
        _unchecked: bool,
        _statements: &mut Vec<Statement>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_args(&mut self, loc: Loc, _args: &mut Vec<NamedArgument>) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Don't write semicolon at the end because expressions can appear as both
    /// part of other node and a statement in the function body
    fn visit_expr(&mut self, loc: Loc, _expr: &mut Expression) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_contract_ident(
        &mut self,
        loc: Loc,
        _ident: &mut Identifier,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_ident(&mut self, loc: Loc, _ident: &mut Identifier) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_ident_path(&mut self, idents: &mut IdentifierPath) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_emit(&mut self, loc: Loc, _event: &mut Expression) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_var_definition(&mut self, var: &mut VariableDefinition) -> Result<(), Self::Error> {
        self.visit_expr(var.ty.loc(), &mut var.ty)?;
        for attr in var.attrs.iter_mut() {
            self.visit_var_attribute(attr)?;
        }
        if let Some(ref mut identifier) = var.name {
            self.visit_ident(identifier.loc, identifier)?;
        }

        if let Some(ref mut initializer) = var.initializer {
            self.visit_expr(initializer.loc(), initializer)?;
        }

        Ok(())
    }

    fn visit_var_definition_stmt(
        &mut self,
        loc: Loc,
        _declaration: &mut VariableDeclaration,
        _expr: &mut Option<Expression>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_var_declaration(
        &mut self,
        var_declaration: &mut VariableDeclaration,
    ) -> Result<(), Self::Error> {
        self.visit_expr(var_declaration.ty.loc(), &mut var_declaration.ty)?;

        if let Some(ref mut storage) = var_declaration.storage {
            self.visit_storage_loc(storage.loc(), storage)?;
        }
        if let Some(ref mut ident) = var_declaration.name {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_storage_loc(
        &mut self,
        loc: Loc,
        _storage: &mut StorageLocation,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_return(
        &mut self,
        loc: Loc,
        _expr: &mut Option<Expression>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_revert(
        &mut self,
        loc: Loc,
        _error: &mut Option<IdentifierPath>,
        _args: &mut Vec<Expression>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_revert_named_args(
        &mut self,
        loc: Loc,
        _error: &mut Option<IdentifierPath>,
        _args: &mut Vec<NamedArgument>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_break(&mut self, loc: Loc, _semicolon: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_continue(&mut self, loc: Loc, _semicolon: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn visit_try(
        &mut self,
        loc: Loc,
        _expr: &mut Expression,
        _returns: &mut Option<(Vec<(Loc, Option<Parameter>)>, Box<Statement>)>,
        _clauses: &mut Vec<CatchClause>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_if(
        &mut self,
        loc: Loc,
        _cond: &mut Expression,
        _if_branch: &mut Box<Statement>,
        _else_branch: &mut Option<Box<Statement>>,
        _is_first_stmt: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_do_while(
        &mut self,
        loc: Loc,
        _body: &mut Statement,
        _cond: &mut Expression,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_while(
        &mut self,
        loc: Loc,
        _cond: &mut Expression,
        _body: &mut Statement,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_for(
        &mut self,
        loc: Loc,
        _init: &mut Option<Box<Statement>>,
        _cond: &mut Option<Box<Expression>>,
        _update: &mut Option<Box<Expression>>,
        _body: &mut Option<Box<Statement>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_var_attribute(
        &mut self,
        attribute: &mut VariableAttribute,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_base(&mut self, base: &mut Base) -> Result<(), Self::Error> {
        self.visit_ident_path(&mut base.name)?;

        if let Some(ref mut args) = base.args {
            for expr in args.iter_mut() {
                self.visit_expr(expr.loc(), expr)?;
            }
        }
        Ok(())
    }
    fn visit_parameter(&mut self, parameter: &mut Parameter) -> Result<(), Self::Error> {
        if let Some(ref mut annotation) = parameter.annotation {
            self.visit_annotation(annotation)?;
        }

        self.visit_expr(parameter.ty.loc(), &mut parameter.ty)?;

        if let Some(ref mut storage) = parameter.storage {
            self.visit_storage_loc(storage.loc(), storage)?;
        }
        if let Some(ref mut ident) = parameter.name {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }
    fn visit_event_parameter(&mut self, param: &mut EventParameter) -> Result<(), Self::Error> {
        self.visit_expr(param.ty.loc(), &mut param.ty)?;
        if let Some(ref mut ident) = param.name {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_error_parameter(&mut self, param: &mut ErrorParameter) -> Result<(), Self::Error> {
        self.visit_expr(param.ty.loc(), &mut param.ty)?;
        if let Some(ref mut ident) = param.name {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }
    fn visit_type_definition(&mut self, def: &mut TypeDefinition) -> Result<(), Self::Error> {
        self.visit_ident(def.name.loc, &mut def.name)?;
        self.visit_expr(def.ty.loc(), &mut def.ty)?;

        Ok(())
    }

    fn visit_stray_semicolon(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_opening_paren(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_closing_paren(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_newline(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn visit_using(&mut self, using: &mut Using) -> Result<(), Self::Error> {
        self.visit_using_list(&mut using.list)?;
        if let Some(ref mut ty) = using.ty {
            self.visit_expr(ty.loc(), ty)?;
        }
        if let Some(ref mut ident) = using.global {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_using_list(&mut self, list: &mut UsingList) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_block(
        &mut self,
        loc: Loc,
        _stmts: &mut Vec<YulStatement>,
        _attempt_single_line: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_expr(&mut self, expr: &mut YulExpression) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_assignment<T>(
        &mut self,
        loc: Loc,
        _exprs: &mut Vec<T>,
        _expr: &mut Option<&mut YulExpression>,
    ) -> Result<(), Self::Error>
    where
        T: Visitable + CodeLocation,
    {
        Ok(())
    }

    fn visit_yul_for(&mut self, stmt: &mut YulFor) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_function_call(&mut self, stmt: &mut YulFunctionCall) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_fun_def(&mut self, stmt: &mut YulFunctionDefinition) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_if(
        &mut self,
        loc: Loc,
        _expr: &mut YulExpression,
        _block: &mut YulBlock,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_leave(&mut self, loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_switch(&mut self, stmt: &mut YulSwitch) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_var_declaration(
        &mut self,
        loc: Loc,
        _idents: &mut Vec<YulTypedIdentifier>,
        _expr: &mut Option<YulExpression>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_typed_ident(&mut self, ident: &mut YulTypedIdentifier) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_parser_error(&mut self, loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }
}

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

                //TODO: check this out if we need this or not
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

// impl Visitable for Loc {
//     fn visit<V>(&mut self, v: &mut V) -> Result<(), V::Error>
//     where
//         V: Visitor,
//     {
//         v.visit_source(*self)
//     }
// }

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
impl_visitable!(YulExpression, visit_yul_expr);
impl_visitable!(YulTypedIdentifier, visit_yul_typed_ident);
