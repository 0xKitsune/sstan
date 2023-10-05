use solang_parser::pt::*;
/// Macro to implmement expression visitor methods
macro_rules! visit_exprs {
    ($func_name:ident, 1) => {
        fn $func_name(
            &mut self,
            _loc: Loc,
            _expr_0: &mut Box<Expression>,
        ) -> Result<(), Self::Error> {
            self.visit_expr(_expr_0.loc(), _expr_0)?;
            Ok(())
        }
    };
    ($func_name:ident, 2) => {
        fn $func_name(
            &mut self,
            _loc: Loc,
            _expr_0: &mut Box<Expression>,
            _expr_1: &mut Box<Expression>,
        ) -> Result<(), Self::Error> {
            self.visit_expr(_expr_0.loc(), _expr_0)?;
            self.visit_expr(_expr_1.loc(), _expr_1)?;
            Ok(())
        }
    };
    ($func_name:ident, 3) => {
        fn $func_name(
            &mut self,
            _loc: Loc,
            _expr_0: &mut Box<Expression>,
            _expr_1: &mut Box<Expression>,
            _expr_2: &mut Box<Expression>,
        ) -> Result<(), Self::Error> {
            self.visit_expr(_expr_0.loc(), _expr_0)?;
            self.visit_expr(_expr_1.loc(), _expr_1)?;
            self.visit_expr(_expr_2.loc(), _expr_2)?;
            Ok(())
        }
    };
}

/// Macro to implement empty extractor implementations
macro_rules! extract {
    ($name:ident, $( $arg_name:ident : $arg_type:ty ),+ ) => {
        fn $name(&mut self, $( $arg_name: $arg_type, )+ ) -> Result<(), Self::Error> {
            Ok(())
        }
    };
}

/// A trait that is invoked while traversing the Solidity Parse Tree.
/// Each method of the [Visitor] trait is a hook that can be potentially overridden.
///
/// Currently the main implementor of this trait is the [`Formatter`](crate::Formatter) struct.
pub trait Visitor {
    type Error: std::error::Error;

    fn visit_source_unit(&mut self, source_unit: &mut SourceUnit) -> Result<(), Self::Error> {
        self.extract_source_unit(source_unit)?;
        for part in &mut source_unit.0 {
            self.visit_source_unit_part(part)?;
        }

        Ok(())
    }

    fn visit_source_unit_part(
        &mut self,
        source_unit_part: &mut SourceUnitPart,
    ) -> Result<(), Self::Error> {
        self.extract_source_unit_part(source_unit_part)?;
        match source_unit_part {
            SourceUnitPart::ContractDefinition(contract) => self.visit_contract(contract)?,
            SourceUnitPart::PragmaDirective(_loc, ident, str) => {
                self.visit_pragma(*_loc, ident, str)?
            }
            SourceUnitPart::ImportDirective(import) => self.visit_import(import)?,
            SourceUnitPart::EnumDefinition(enumeration) => self.visit_enum(enumeration)?,
            SourceUnitPart::StructDefinition(structure) => self.visit_struct(structure)?,
            SourceUnitPart::EventDefinition(event) => self.visit_event(event)?,
            SourceUnitPart::ErrorDefinition(error) => self.visit_error(error)?,
            SourceUnitPart::FunctionDefinition(function) => self.visit_function(function)?,
            SourceUnitPart::VariableDefinition(variable) => self.visit_var_definition(variable)?,
            SourceUnitPart::TypeDefinition(def) => self.visit_type_definition(def)?,
            SourceUnitPart::StraySemicolon(_loc) => self.visit_stray_semicolon(*_loc)?,
            SourceUnitPart::Using(using) => self.visit_using(using)?,
            SourceUnitPart::Annotation(annotation) => self.visit_annotation(annotation)?,
        }
        Ok(())
    }

    fn visit_function(&mut self, function: &mut FunctionDefinition) -> Result<(), Self::Error> {
        self.extract_function(function)?;
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

    /// Statement visitor
    fn visit_statement(&mut self, statement: &mut Statement) -> Result<(), Self::Error> {
        self.extract_statement(statement)?;
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
            Statement::Args(_loc, args) => self.visit_args(*_loc, args)?,
            Statement::If(_loc, expr, _if, else_) => {
                self.visit_if(*_loc, expr, _if, else_, true)?
            } //TODO: Revisit this
            Statement::While(_loc, expr, body) => self.visit_while(*_loc, expr, body)?,
            Statement::Expression(_loc, expr) => self.visit_expr(*_loc, expr)?,
            Statement::VariableDefinition(_loc, var, expr) => {
                self.visit_var_declaration(var)?;
                if let Some(expr) = expr {
                    self.visit_expr(expr.loc(), expr)?;
                }
            }
            Statement::For(_loc, init, cond, next, body) => {
                self.visit_for(*_loc, init, cond, next, body)?
            }
            Statement::DoWhile(_loc, body, cond) => self.visit_do_while(*_loc, body, cond)?,
            Statement::Continue(_loc) => self.visit_continue(*_loc, true)?,
            Statement::Break(_loc) => self.visit_break(*_loc, true)?,
            Statement::Return(_loc, expr) => self.visit_return(*_loc, expr)?,
            Statement::Revert(_loc, path, args) => self.visit_revert(*_loc, path, args)?,
            Statement::RevertNamedArgs(_loc, path, args) => {
                self.visit_revert_named_args(*_loc, path, args)?
            }
            Statement::Emit(_loc, expr) => self.visit_emit(*_loc, expr)?,
            Statement::Try(_loc, expr, returns, catches) => {
                self.visit_try(*_loc, expr, returns, catches)?
            }
            Statement::Error(_loc) => self.visit_parser_error(*_loc)?,
        }

        Ok(())
    }

    fn visit_named_arg(&mut self, arg: &mut NamedArgument) -> Result<(), Self::Error> {
        self.extract_named_arg(arg)?;
        self.visit_expr(arg.expr.loc(), &mut arg.expr)?;
        self.visit_ident(arg.name.loc, &mut arg.name)?;
        Ok(())
    }

    fn visit_returns(&mut self, returns: &mut ParameterList) -> Result<(), Self::Error> {
        self.extract_returns(returns)?;
        for parameter in returns {
            if let Some(ref mut param) = parameter.1 {
                self.visit_parameter(param)?;
            }
        }

        Ok(())
    }

    fn visit_function_attributes(
        &mut self,
        _attributes: &mut Vec<FunctionAttribute>,
    ) -> Result<(), Self::Error> {
        self.extract_function_attributes(_attributes)?;
        for attribute in _attributes {
            self.visit_function_attribute(attribute)?;
        }
        Ok(())
    }

    fn visit_function_attribute(
        &mut self,
        _attribute: &mut FunctionAttribute,
    ) -> Result<(), Self::Error> {
        self.extract_function_attribute(_attribute)?;
        Ok(())
    }
    //pub type ParameterList = Vec<(Loc, Option<Parameter>)>;
    fn visit_parameter_list(
        &mut self,
        parameter_list: &mut ParameterList,
    ) -> Result<(), Self::Error> {
        self.extract_parameter_list(parameter_list)?;
        for parameter in parameter_list {
            if let Some(ref mut param) = parameter.1 {
                self.visit_parameter(param)?;
            }
        }

        Ok(())
    }

    fn visit_function_type(&mut self, _ty: &mut FunctionTy) -> Result<(), Self::Error> {
        self.extract_function_type(_ty)?;
        Ok(())
    }
    fn visit_error(&mut self, error: &mut ErrorDefinition) -> Result<(), Self::Error> {
        self.extract_error(error)?;
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
        self.extract_event(event)?;
        if let Some(ref mut identifier) = event.name {
            self.visit_ident(identifier.loc, identifier)?;
        }
        for event_parameter in event.fields.iter_mut() {
            self.visit_event_parameter(event_parameter)?;
        }
        Ok(())
    }

    fn visit_struct(&mut self, structure: &mut StructDefinition) -> Result<(), Self::Error> {
        self.extract_struct(structure)?;
        if let Some(ident) = structure.name.as_mut() {
            self.visit_ident(ident.loc, ident)?;
        }
        for var_declaration in structure.fields.iter_mut() {
            self.visit_var_declaration(var_declaration)?;
        }
        Ok(())
    }

    fn visit_fields(&mut self, _fields: &mut Vec<VariableDeclaration>) -> Result<(), Self::Error> {
        self.extract_fields(_fields)?;
        for field in _fields {
            self.visit_var_declaration(field)?;
        }
        Ok(())
    }

    fn visit_contract(&mut self, contract: &mut ContractDefinition) -> Result<(), Self::Error> {
        self.extract_contract(contract)?;
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

    fn visit_contract_part(
        &mut self,
        _contract_part: &mut ContractPart,
    ) -> Result<(), Self::Error> {
        self.extract_contract_part(_contract_part)?;
        match _contract_part {
            ContractPart::StructDefinition(struct_definition) => {
                self.visit_struct(struct_definition)?;
            }
            ContractPart::EventDefinition(event_definition) => {
                self.visit_event(event_definition)?;
            }
            ContractPart::EnumDefinition(enum_definition) => {
                self.visit_enum(enum_definition)?;
            }
            ContractPart::ErrorDefinition(error_definition) => {
                self.visit_error(error_definition)?;
            }
            ContractPart::VariableDefinition(variable_definition) => {
                self.visit_var_definition(variable_definition)?;
            }
            ContractPart::FunctionDefinition(function_definition) => {
                self.visit_function(function_definition)?;
            }
            ContractPart::TypeDefinition(type_definition) => {
                self.visit_type_definition(type_definition)?;
            }
            ContractPart::Annotation(annotation) => {
                self.visit_annotation(annotation)?;
            }
            ContractPart::Using(using) => {
                self.visit_using(using)?;
            }
            ContractPart::StraySemicolon(loc) => {
                self.visit_stray_semicolon(*loc)?;
            }
        }

        Ok(())
    }

    fn visit_contract_type(&mut self, _ty: &mut ContractTy) -> Result<(), Self::Error> {
        self.extract_contract_type(_ty)?;
        match _ty {
            ContractTy::Abstract(loc) => {
                self.visit_abstract(*loc)?;
            }
            ContractTy::Contract(loc) => {
                self.visit_contract_keyword(*loc)?;
            }
            ContractTy::Interface(loc) => {
                self.visit_interface(*loc)?;
            }
            ContractTy::Library(loc) => {
                self.visit_library(*loc)?;
            }
        }

        Ok(())
    }

    fn visit_abstract(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_contract_keyword(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_interface(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_library(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_import(&mut self, _import: &mut Import) -> Result<(), Self::Error> {
        self.extract_import(_import)?;
        match _import {
            Import::Plain(str, loc) => {
                self.visit_import_plain(*loc, str)?;
            }
            Import::GlobalSymbol(str, ident, loc) => {
                self.visit_import_global(*loc, str, ident)?;
            }
            Import::Rename(string, imports, from) => {
                self.visit_import_renames(*from, imports, string)?;
            }
        }
        Ok(())
    }

    fn visit_enum(&mut self, enum_definition: &mut Box<EnumDefinition>) -> Result<(), Self::Error> {
        self.extract_enum(enum_definition)?;
        if let Some(ref mut identifier) = enum_definition.name {
            self.visit_ident(identifier.loc, identifier)?;
        }

        for ref mut value in enum_definition.values.clone().into_iter().flatten() {
            self.visit_ident(value.loc, value)?;
        }

        Ok(())
    }

    fn visit_annotation(&mut self, _annotation: &mut Annotation) -> Result<(), Self::Error> {
        self.extract_annotation(_annotation)?;
        self.visit_ident(_annotation.id.loc, &mut _annotation.id)?;
        if let Some(ref mut value) = _annotation.value {
            self.visit_expr(value.loc(), value)?;
        }
        Ok(())
    }

    fn visit_pragma(
        &mut self,
        _loc: Loc,
        _ident: &mut Option<Identifier>,
        _str: &mut Option<StringLiteral>,
    ) -> Result<(), Self::Error> {
        self.extract_pragma(_loc, _ident, _str)?;
        if let Some(ident) = _ident {
            self.visit_ident(ident.loc, ident)?;
        }
        if let Some(ref mut string_literal) = _str {
            self.visit_string_literal(string_literal)?;
        }
        Ok(())
    }
    fn visit_import_plain(
        &mut self,
        _loc: Loc,
        _import: &mut ImportPath,
    ) -> Result<(), Self::Error> {
        self.extract_import_plain(_loc, _import)?;
        // self.visit_string_literal(_import)?;
        Ok(())
    }
    fn visit_import_global(
        &mut self,
        _loc: Loc,
        _global: &mut ImportPath,
        _alias: &mut Identifier,
    ) -> Result<(), Self::Error> {
        self.extract_import_global(_loc, _global, _alias)?;
        // self.visit_string_literal(_global)?;
        // self.visit_ident(_alias.loc, _alias)?;
        Ok(())
    }

    fn visit_import_renames(
        &mut self,
        _loc: Loc,
        _imports: &mut [(Identifier, Option<Identifier>)],
        _from: &mut ImportPath,
    ) -> Result<(), Self::Error> {
        self.extract_import_renames(_loc, _imports, _from)?;
        // self.visit_string_literal(_from)?;
        for (ident_0, ident_1) in _imports {
            self.visit_ident(ident_0.loc, ident_0)?;
            if let Some(ident) = ident_1 {
                self.visit_ident(ident.loc, ident)?;
            }
        }
        Ok(())
    }

    fn visit_assembly(
        &mut self,
        _loc: Loc,
        _dialect: &mut Option<StringLiteral>,
        _block: &mut YulBlock,
        _flags: &mut Option<Vec<StringLiteral>>,
    ) -> Result<(), Self::Error> {
        self.extract_assembly(_loc, _dialect, _block, _flags)?;
        if let Some(ref mut dialect) = _dialect {
            self.visit_string_literal(dialect)?;
        }
        self.visit_yul_block(_block.loc, &mut _block.statements, false)?; //TODO: Figure out what the bool is for, set to false as per the Visitable implementation
        if let Some(ref mut flags) = _flags {
            for flag in flags {
                self.visit_string_literal(flag)?;
            }
        }
        Ok(())
    }

    fn visit_block(
        &mut self,
        _loc: Loc,
        _unchecked: bool,
        _statements: &mut Vec<Statement>,
    ) -> Result<(), Self::Error> {
        self.extract_block(_loc, _unchecked, _statements)?;
        for statement in _statements {
            self.visit_statement(statement)?;
        }
        Ok(())
    }

    fn visit_args(&mut self, _loc: Loc, _args: &mut Vec<NamedArgument>) -> Result<(), Self::Error> {
        self.extract_args(_loc, _args)?;
        for arg in _args {
            self.visit_named_arg(arg)?;
        }
        Ok(())
    }

    /// Expressions
    /// Don't write semicolon at the end because expressions can appear as both
    /// part of other node and a statement in the function body
    fn visit_expr(&mut self, _loc: Loc, _expr: &mut Expression) -> Result<(), Self::Error> {
        self.extract_expr(_loc, _expr)?;
        match _expr {
            Expression::PostIncrement(_loc, expr) => {
                self.visit_post_increment(*_loc, expr)?;
            }
            Expression::PostDecrement(_loc, expr) => {
                self.visit_post_decrement(*_loc, expr)?;
            }
            Expression::New(_loc, expr) => {
                self.visit_new(*_loc, expr)?;
            }
            Expression::ArraySubscript(_loc, _expr_0, _expr_1) => {
                self.visit_array_subscript(*_loc, _expr_0, _expr_1)?;
            }
            Expression::ArraySlice(_loc, expr, _option_expr_0, _option_expr_1) => {
                self.visit_array_slice(*_loc, expr, _option_expr_0, _option_expr_1)?;
            }
            Expression::Parenthesis(_loc, expr) => {
                self.visit_parenthesis(*_loc, expr)?;
            }
            Expression::MemberAccess(_loc, expr, ident) => {
                self.visit_member_access(*_loc, expr, ident)?;
            }
            Expression::FunctionCall(_loc, expr, params) => {
                self.visit_function_call(*_loc, expr, params)?;
            }
            Expression::FunctionCallBlock(_loc, expr, statement) => {
                self.visit_function_call_block(*_loc, expr, statement)?;
            }
            Expression::NamedFunctionCall(_loc, expr, params) => {
                self.visit_named_function_call(*_loc, expr, params)?;
            }
            Expression::Not(_loc, expr) => {
                self.visit_not(*_loc, expr)?;
            }
            Expression::BitwiseNot(_loc, expr) => {
                self.visit_bitwise_not(*_loc, expr)?;
            }
            Expression::Delete(_loc, expr) => {
                self.visit_delete(*_loc, expr)?;
            }
            Expression::PreIncrement(_loc, expr) => {
                self.visit_pre_increment(*_loc, expr)?;
            }
            Expression::PreDecrement(_loc, expr) => {
                self.visit_pre_decrement(*_loc, expr)?;
            }
            Expression::UnaryPlus(_loc, expr) => {
                self.visit_unary_plus(*_loc, expr)?;
            }
            Expression::Negate(_loc, expr) => {
                self.visit_negate(*_loc, expr)?;
            }
            Expression::Power(_loc, expr_0, expr_1) => {
                self.visit_power(*_loc, expr_0, expr_1)?;
            }
            Expression::Multiply(_loc, expr_0, expr_1) => {
                self.visit_multiply(*_loc, expr_0, expr_1)?;
            }
            Expression::Divide(_loc, expr_0, expr_1) => {
                self.visit_divide(*_loc, expr_0, expr_1)?;
            }
            Expression::Modulo(_loc, expr_0, expr_1) => {
                self.visit_modulo(*_loc, expr_0, expr_1)?;
            }

            Expression::Add(_loc, expr_0, expr_1) => {
                self.visit_add(*_loc, expr_0, expr_1)?;
            }

            Expression::Subtract(_loc, expr_0, expr_1) => {
                self.visit_subtract(*_loc, expr_0, expr_1)?;
            }

            Expression::ShiftLeft(_loc, expr_0, expr_1) => {
                self.visit_shift_left(*_loc, expr_0, expr_1)?;
            }

            Expression::ShiftRight(_loc, expr_0, expr_1) => {
                self.visit_shift_right(*_loc, expr_0, expr_1)?;
            }

            Expression::BitwiseAnd(_loc, expr_0, expr_1) => {
                self.visit_bitwise_and(*_loc, expr_0, expr_1)?;
            }

            Expression::BitwiseXor(_loc, expr_0, expr_1) => {
                self.visit_bitwise_xor(*_loc, expr_0, expr_1)?;
            }

            Expression::BitwiseOr(_loc, expr_0, expr_1) => {
                self.visit_bitwise_or(*_loc, expr_0, expr_1)?;
            }

            Expression::Less(_loc, expr_0, expr_1) => {
                self.visit_less(*_loc, expr_0, expr_1)?;
            }

            Expression::LessEqual(_loc, expr_0, expr_1) => {
                self.visit_less_equal(*_loc, expr_0, expr_1)?;
            }

            Expression::More(_loc, expr_0, expr_1) => {
                self.visit_more(*_loc, expr_0, expr_1)?;
            }

            Expression::MoreEqual(_loc, expr_0, expr_1) => {
                self.visit_more_equal(*_loc, expr_0, expr_1)?;
            }

            Expression::Equal(_loc, expr_0, expr_1) => {
                self.visit_equal(*_loc, expr_0, expr_1)?;
            }

            Expression::NotEqual(_loc, expr_0, expr_1) => {
                self.visit_not_equal(*_loc, expr_0, expr_1)?;
            }

            Expression::And(_loc, expr_0, expr_1) => {
                self.visit_and(*_loc, expr_0, expr_1)?;
            }

            Expression::Or(_loc, expr_0, expr_1) => {
                self.visit_or(*_loc, expr_0, expr_1)?;
            }

            Expression::ConditionalOperator(_loc, expr_0, expr_1, expr_2) => {
                self.visit_conditional_operator(*_loc, expr_0, expr_1, expr_2)?;
            }

            Expression::Assign(_loc, expr_0, expr_1) => {
                self.visit_assign(*_loc, expr_0, expr_1)?;
            }

            Expression::AssignOr(_loc, expr_0, expr_1) => {
                self.visit_assign_or(*_loc, expr_0, expr_1)?;
            }

            Expression::AssignXor(_loc, expr_0, expr_1) => {
                self.visit_assign_xor(*_loc, expr_0, expr_1)?;
            }

            Expression::AssignAnd(_loc, expr_0, expr_1) => {
                self.visit_assign_and(*_loc, expr_0, expr_1)?;
            }

            Expression::AssignShiftLeft(_loc, expr_0, expr_1) => {
                self.visit_assign_shift_left(*_loc, expr_0, expr_1)?;
            }

            Expression::AssignShiftRight(_loc, expr_0, expr_1) => {
                self.visit_assign_shift_right(*_loc, expr_0, expr_1)?;
            }

            Expression::AssignAdd(_loc, expr_0, expr_1) => {
                self.visit_assign_add(*_loc, expr_0, expr_1)?;
            }

            Expression::AssignSubtract(_loc, expr_0, expr_1) => {
                self.visit_assign_subtract(*_loc, expr_0, expr_1)?;
            }

            Expression::AssignMultiply(_loc, expr_0, expr_1) => {
                self.visit_assign_multiply(*_loc, expr_0, expr_1)?;
            }

            Expression::AssignDivide(_loc, expr_0, expr_1) => {
                self.visit_assign_divide(*_loc, expr_0, expr_1)?;
            }

            Expression::AssignModulo(_loc, expr_0, expr_1) => {
                self.visit_assign_modulo(*_loc, expr_0, expr_1)?;
            }

            Expression::BoolLiteral(_loc, _value) => {
                self.visit_bool_literal(*_loc, _value)?;
            }

            Expression::NumberLiteral(_loc, string_0, string_1, ident) => {
                self.visit_number_literal(*_loc, string_0, string_1, ident)?;
            }

            Expression::RationalNumberLiteral(_loc, string_0, string_1, string_2, ident) => {
                self.visit_rational_number_literal(*_loc, string_0, string_1, string_2, ident)?;
            }

            Expression::HexNumberLiteral(_loc, string_0, ident) => {
                self.visit_hex_number_literal(*_loc, string_0, ident)?;
            }

            Expression::StringLiteral(string_literal_vec) => {
                self.visit_string_literal_vec(string_literal_vec)?;
            }

            Expression::Type(_loc, _type) => {
                self.visit_type(*_loc, _type)?;
            }

            Expression::HexLiteral(hex_literal_vec) => {
                for literal in hex_literal_vec.iter_mut() {
                    self.visit_hex_literal(literal)?;
                }
            }

            Expression::AddressLiteral(_loc, value) => {
                self.visit_address_literal(*_loc, value)?;
            }

            Expression::Variable(ident) => {
                self.visit_variable(ident.loc, ident)?;
            }

            Expression::List(_loc, parameter_list) => {
                self.visit_list(*_loc, parameter_list)?;
            }

            Expression::ArrayLiteral(_loc, expr_vec) => {
                self.visit_array_literal(*_loc, expr_vec)?;
            }
        }
        Ok(())
    }

    fn visit_list(
        &mut self,
        _loc: Loc,
        _parameter_list: &mut Vec<(Loc, Option<Parameter>)>,
    ) -> Result<(), Self::Error> {
        self.extract_list(_loc, _parameter_list)?;
        for (_, parameter) in _parameter_list.iter_mut() {
            if let Some(parameter) = parameter {
                self.visit_parameter(parameter)?;
            }
        }
        Ok(())
    }

    fn visit_array_literal(
        &mut self,
        _loc: Loc,
        _expr_vec: &mut Vec<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_array_literal(_loc, _expr_vec)?;
        for expr in _expr_vec.iter_mut() {
            self.visit_expr(expr.loc(), expr)?;
        }
        Ok(())
    }

    fn visit_hex_literal(&mut self, _hex_literal: &mut HexLiteral) -> Result<(), Self::Error> {
        self.extract_hex_literal(_hex_literal)?;
        Ok(())
    }

    fn visit_variable(&mut self, _loc: Loc, _ident: &mut Identifier) -> Result<(), Self::Error> {
        self.extract_variable(_loc, _ident)?;
        self.visit_ident(_ident.loc, _ident)?;
        Ok(())
    }

    fn visit_address_literal(&mut self, _loc: Loc, _value: &mut String) -> Result<(), Self::Error> {
        self.extract_address_literal(_loc, _value)?;
        Ok(())
    }
    fn visit_type(&mut self, _loc: Loc, _type: &mut Type) -> Result<(), Self::Error> {
        self.extract_type(_loc, _type)?;
        Ok(())
    }

    fn visit_string_literal_vec(
        &mut self,
        _string_literal_vec: &mut Vec<StringLiteral>,
    ) -> Result<(), Self::Error> {
        self.extract_string_literal_vec(_string_literal_vec)?;
        Ok(())
    }

    fn visit_string_literal(
        &mut self,
        _string_literal: &mut StringLiteral,
    ) -> Result<(), Self::Error> {
        self.extract_string_literal(_string_literal)?;
        Ok(())
    }

    fn visit_hex_number_literal(
        &mut self,
        _loc: Loc,
        _string_0: &mut String,
        ident: &mut Option<Identifier>,
    ) -> Result<(), Self::Error> {
        self.extract_hex_number_literal(_loc, _string_0, ident)?;
        if let Some(ident) = ident {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_rational_number_literal(
        &mut self,
        _loc: Loc,
        _string_0: &mut String,
        _string_1: &mut String,
        _string_2: &mut String,
        ident: &mut Option<Identifier>,
    ) -> Result<(), Self::Error> {
        self.extract_rational_number_literal(_loc, _string_0, _string_1, _string_2, ident)?;
        if let Some(ident) = ident {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_number_literal(
        &mut self,
        _loc: Loc,
        _string_0: &mut String,
        _string_1: &mut String,
        ident: &mut Option<Identifier>,
    ) -> Result<(), Self::Error> {
        self.extract_number_literal(_loc, _string_0, _string_1, ident)?;
        if let Some(ident) = ident {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_bool_literal(&mut self, _loc: Loc, _value: &mut bool) -> Result<(), Self::Error> {
        self.extract_bool_literal(_loc, _value)?;
        Ok(())
    }

    visit_exprs!(visit_assign_add, 2);
    visit_exprs!(visit_assign_modulo, 2);
    visit_exprs!(visit_assign_divide, 2);
    visit_exprs!(visit_assign_multiply, 2);
    visit_exprs!(visit_assign_subtract, 2);
    visit_exprs!(visit_assign_shift_right, 2);
    visit_exprs!(visit_assign_shift_left, 2);
    visit_exprs!(visit_assign_and, 2);
    visit_exprs!(visit_assign_xor, 2);
    visit_exprs!(visit_assign_or, 2);
    visit_exprs!(visit_assign, 2);
    visit_exprs!(visit_or, 2);
    visit_exprs!(visit_and, 2);
    visit_exprs!(visit_not_equal, 2);
    visit_exprs!(visit_equal, 2);
    visit_exprs!(visit_more_equal, 2);
    visit_exprs!(visit_more, 2);
    visit_exprs!(visit_less_equal, 2);
    visit_exprs!(visit_less, 2);
    visit_exprs!(visit_bitwise_or, 2);
    visit_exprs!(visit_conditional_operator, 3);
    visit_exprs!(visit_bitwise_xor, 2);
    visit_exprs!(visit_bitwise_and, 2);
    visit_exprs!(visit_shift_right, 2);
    visit_exprs!(visit_shift_left, 2);
    visit_exprs!(visit_subtract, 2);
    visit_exprs!(visit_add, 2);
    visit_exprs!(visit_modulo, 2);
    visit_exprs!(visit_divide, 2);
    visit_exprs!(visit_multiply, 2);
    visit_exprs!(visit_power, 2);
    visit_exprs!(visit_negate, 1);
    visit_exprs!(visit_delete, 1);
    visit_exprs!(visit_bitwise_not, 1);
    visit_exprs!(visit_not, 1);

    fn visit_named_function_call(
        &mut self,
        _loc: Loc,
        _ident: &mut Box<Expression>,
        _params: &mut Vec<NamedArgument>,
    ) -> Result<(), Self::Error> {
        self.extract_named_function_call(_loc, _ident, _params)?;
        self.visit_expr(_ident.loc(), _ident)?;
        for named_arg in _params.iter_mut() {
            self.visit_named_arg(named_arg)?;
        }
        Ok(())
    }

    fn visit_function_call_block(
        &mut self,
        _loc: Loc,
        _expr: &mut Box<Expression>,
        _statement: &mut Box<Statement>,
    ) -> Result<(), Self::Error> {
        self.extract_function_call_block(_loc, _expr, _statement)?;
        self.visit_expr(_expr.loc(), _expr)?;
        self.visit_statement(_statement)?;
        Ok(())
    }

    fn visit_function_call(
        &mut self,
        _loc: Loc,
        _expr: &mut Box<Expression>,
        _params: &mut Vec<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_function_call(_loc, _expr, _params)?;
        self.visit_expr(_expr.loc(), _expr)?;
        for param in _params.iter_mut() {
            self.visit_expr(param.loc(), param)?;
        }
        Ok(())
    }

    fn visit_member_access(
        &mut self,
        _loc: Loc,
        _expr: &mut Box<Expression>,
        _ident: &mut Identifier,
    ) -> Result<(), Self::Error> {
        self.extract_member_access(_loc, _expr, _ident)?;
        self.visit_expr(_expr.loc(), _expr)?;
        self.visit_ident(_ident.loc, _ident)?;
        Ok(())
    }

    fn visit_parenthesis(
        &mut self,
        _loc: Loc,
        _expr: &mut Box<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_parenthesis(_loc, _expr)?;
        self.visit_expr(_expr.loc(), _expr)?;
        Ok(())
    }

    fn visit_array_slice(
        &mut self,
        _loc: Loc,
        _expr: &mut Box<Expression>,
        _option_expr_0: &mut Option<Box<Expression>>,
        _option_expr_1: &mut Option<Box<Expression>>,
    ) -> Result<(), Self::Error> {
        self.extract_array_slice(_loc, _expr, _option_expr_0, _option_expr_1)?;
        self.visit_expr(_expr.loc(), _expr)?;
        if let Some(expr) = _option_expr_0 {
            self.visit_expr(expr.loc(), expr)?;
        }
        if let Some(expr) = _option_expr_1 {
            self.visit_expr(expr.loc(), expr)?;
        }
        Ok(())
    }

    fn visit_array_subscript(
        &mut self,
        _loc: Loc,
        _expr_0: &mut Box<Expression>,
        _expr_1: &mut Option<Box<Expression>>,
    ) -> Result<(), Self::Error> {
        self.extract_array_subscript(_loc, _expr_0, _expr_1)?;
        self.visit_expr(_expr_0.loc(), _expr_0)?;
        if let Some(expr) = _expr_1 {
            self.visit_expr(expr.loc(), expr)?;
        }
        Ok(())
    }

    fn visit_new(&mut self, _loc: Loc, _expr: &mut Box<Expression>) -> Result<(), Self::Error> {
        self.extract_new(_loc, _expr)?;
        self.visit_expr(_expr.loc(), _expr)?;
        Ok(())
    }

    fn visit_unary_plus(
        &mut self,
        _loc: Loc,
        _expr: &mut Box<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_unary_plus(_loc, _expr)?;
        self.visit_expr(_expr.loc(), _expr)?;
        Ok(())
    }

    fn visit_pre_decrement(
        &mut self,
        _loc: Loc,
        _expr: &mut Box<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_pre_decrement(_loc, _expr)?;
        self.visit_expr(_expr.loc(), _expr)?;
        Ok(())
    }

    fn visit_post_decrement(
        &mut self,
        _loc: Loc,
        _expr: &mut Box<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_post_decrement(_loc, _expr)?;
        self.visit_expr(_expr.loc(), _expr)?;
        Ok(())
    }

    fn visit_pre_increment(
        &mut self,
        _loc: Loc,
        _expr: &mut Box<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_pre_increment(_loc, _expr)?;
        self.visit_expr(_expr.loc(), _expr)?;
        Ok(())
    }

    fn visit_post_increment(
        &mut self,
        _loc: Loc,
        _expr: &mut Box<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_post_increment(_loc, _expr)?;
        self.visit_expr(_expr.loc(), _expr)?;
        Ok(())
    }

    fn visit_contract_ident(
        &mut self,
        _loc: Loc,
        _ident: &mut Identifier,
    ) -> Result<(), Self::Error> {
        self.extract_contract_ident(_loc, _ident)?;
        self.visit_ident(_ident.loc, _ident)?;
        Ok(())
    }

    fn visit_ident(&mut self, _loc: Loc, _ident: &mut Identifier) -> Result<(), Self::Error> {
        self.extract_ident(_loc, _ident)?;
        Ok(())
    }

    fn visit_ident_path(
        &mut self,
        _identifier_path: &mut IdentifierPath,
    ) -> Result<(), Self::Error> {
        self.extract_ident_path(_identifier_path)?;
        Ok(())
    }

    fn visit_emit(&mut self, _loc: Loc, _event: &mut Expression) -> Result<(), Self::Error> {
        self.extract_emit(_loc, _event)?;
        self.visit_expr(_event.loc(), _event)?;
        Ok(())
    }
    fn visit_var_definition(&mut self, var: &mut VariableDefinition) -> Result<(), Self::Error> {
        self.extract_var_definition(var)?;
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
        _loc: Loc,
        _declaration: &mut VariableDeclaration,
        _expr: &mut Option<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_var_definition_stmt(_loc, _declaration, _expr)?;
        self.visit_var_declaration(_declaration)?;
        if let Some(expr) = _expr {
            self.visit_expr(expr.loc(), expr)?;
        }
        Ok(())
    }

    fn visit_var_declaration(
        &mut self,
        var_declaration: &mut VariableDeclaration,
    ) -> Result<(), Self::Error> {
        self.extract_var_declaration(var_declaration)?;
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
        _loc: Loc,
        _storage: &mut StorageLocation,
    ) -> Result<(), Self::Error> {
        self.extract_storage_loc(_loc, _storage)?;
        match _storage {
            StorageLocation::Storage(loc) => self.visit_storage(*loc)?,
            StorageLocation::Memory(loc) => self.visit_memory(*loc)?,
            StorageLocation::Calldata(loc) => self.visit_calldata(*loc)?,
        }
        Ok(())
    }

    fn visit_storage(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_memory(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_calldata(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_return(
        &mut self,
        _loc: Loc,
        _expr: &mut Option<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_return(_loc, _expr)?;
        if let Some(expr) = _expr {
            self.visit_expr(expr.loc(), expr)?;
        }
        Ok(())
    }

    fn visit_revert(
        &mut self,
        _loc: Loc,
        _error: &mut Option<IdentifierPath>,
        _args: &mut Vec<Expression>,
    ) -> Result<(), Self::Error> {
        self.extract_revert(_loc, _error, _args)?;
        if let Some(ref mut error) = _error {
            self.visit_ident_path(error)?;
        }
        for arg in _args.iter_mut() {
            self.visit_expr(arg.loc(), arg)?;
        }
        Ok(())
    }

    fn visit_revert_named_args(
        &mut self,
        _loc: Loc,
        _error: &mut Option<IdentifierPath>,
        _args: &mut Vec<NamedArgument>,
    ) -> Result<(), Self::Error> {
        self.extract_revert_named_args(_loc, _error, _args)?;
        if let Some(ref mut error) = _error {
            self.visit_ident_path(error)?;
        }
        for arg in _args.iter_mut() {
            self.visit_named_arg(arg)?;
        }
        Ok(())
    }

    fn visit_break(&mut self, _loc: Loc, _semicolon: bool) -> Result<(), Self::Error> {
        self.extract_break(_loc, _semicolon)?;
        Ok(())
    }

    fn visit_continue(&mut self, _loc: Loc, _semicolon: bool) -> Result<(), Self::Error> {
        self.extract_continue(_loc, _semicolon)?;
        Ok(())
    }
    #[allow(clippy::type_complexity)]
    fn visit_try(
        &mut self,
        _loc: Loc,
        _expr: &mut Expression,
        _returns: &mut Option<(Vec<(Loc, Option<Parameter>)>, Box<Statement>)>,
        _clauses: &mut Vec<CatchClause>,
    ) -> Result<(), Self::Error> {
        self.extract_try(_loc, _expr, _returns, _clauses)?;
        self.visit_expr(_expr.loc(), _expr)?;
        if let Some(returns) = _returns {
            for (_, param) in returns.0.iter_mut() {
                if let Some(param) = param {
                    self.visit_parameter(param)?;
                }
            }
            self.visit_statement(&mut returns.1)?;
        }
        for clause in _clauses.iter_mut() {
            self.visit_catch_clause(clause)?;
        }
        Ok(())
    }
    //TODO:
    fn visit_catch_clause(&mut self, _clause: &mut CatchClause) -> Result<(), Self::Error> {
        self.extract_catch_clause(_clause)?;
        Ok(())
    }

    fn visit_if(
        &mut self,
        _loc: Loc,
        _cond: &mut Expression,
        _if_branch: &mut Box<Statement>,
        _else_branch: &mut Option<Box<Statement>>,
        _is_first_stmt: bool,
    ) -> Result<(), Self::Error> {
        self.extract_if(_loc, _cond, _if_branch, _else_branch, _is_first_stmt)?;
        self.visit_expr(_cond.loc(), _cond)?;
        self.visit_statement(_if_branch)?;
        if let Some(else_branch) = _else_branch {
            self.visit_statement(else_branch)?;
        }
        Ok(())
    }

    fn visit_do_while(
        &mut self,
        _loc: Loc,
        _body: &mut Statement,
        _cond: &mut Expression,
    ) -> Result<(), Self::Error> {
        self.extract_do_while(_loc, _body, _cond)?;
        self.visit_statement(_body)?;
        self.visit_expr(_cond.loc(), _cond)?;
        Ok(())
    }

    fn visit_while(
        &mut self,
        _loc: Loc,
        _cond: &mut Expression,
        _body: &mut Statement,
    ) -> Result<(), Self::Error> {
        self.extract_while(_loc, _cond, _body)?;
        self.visit_expr(_cond.loc(), _cond)?;
        self.visit_statement(_body)?;
        Ok(())
    }

    fn visit_for(
        &mut self,
        _loc: Loc,
        _init: &mut Option<Box<Statement>>,
        _cond: &mut Option<Box<Expression>>,
        _update: &mut Option<Box<Expression>>,
        _body: &mut Option<Box<Statement>>,
    ) -> Result<(), Self::Error> {
        self.extract_for(_loc, _init, _cond, _update, _body)?;
        if let Some(init) = _init {
            self.visit_statement(init)?;
        }
        if let Some(cond) = _cond {
            self.visit_expr(cond.loc(), cond)?;
        }
        if let Some(update) = _update {
            self.visit_expr(update.loc(), update)?;
        }
        if let Some(body) = _body {
            self.visit_statement(body)?;
        }
        Ok(())
    }

    fn visit_var_attribute(
        &mut self,
        _variable_attribute: &mut VariableAttribute,
    ) -> Result<(), Self::Error> {
        self.extract_var_attribute(_variable_attribute)?;
        match _variable_attribute {
            VariableAttribute::Visibility(visibility) => {
                self.visit_visibility(visibility)?;
            }
            VariableAttribute::Constant(loc) => {
                self.visit_constant(*loc)?;
            }

            VariableAttribute::Immutable(loc) => {
                self.visit_immutable(*loc)?;
            }

            VariableAttribute::Override(loc, paths) => {
                self.visit_override(*loc, paths)?;
            }
        }
        Ok(())
    }

    fn visit_override(
        &mut self,
        _loc: Loc,
        _paths: &mut Vec<IdentifierPath>,
    ) -> Result<(), Self::Error> {
        self.extract_override(_loc, _paths)?;
        for path in _paths.iter_mut() {
            self.visit_ident_path(path)?;
        }
        Ok(())
    }

    fn visit_immutable(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_constant(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_visibility(&mut self, _visibility: &mut Visibility) -> Result<(), Self::Error> {
        self.extract_visibility(_visibility)?;
        match _visibility {
            Visibility::Private(loc) => {
                self.visit_private(*loc)?;
            }
            Visibility::Public(loc) => {
                self.visit_public(*loc)?;
            }
            Visibility::Internal(loc) => {
                self.visit_internal(*loc)?;
            }
            Visibility::External(loc) => {
                self.visit_external(*loc)?;
            }
        }
        Ok(())
    }

    fn visit_private(&mut self, _loc: Option<Loc>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_public(&mut self, _loc: Option<Loc>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_internal(&mut self, _loc: Option<Loc>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_external(&mut self, _loc: Option<Loc>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_base(&mut self, base: &mut Base) -> Result<(), Self::Error> {
        self.extract_base(base)?;
        self.visit_ident_path(&mut base.name)?;

        if let Some(ref mut args) = base.args {
            for expr in args.iter_mut() {
                self.visit_expr(expr.loc(), expr)?;
            }
        }
        Ok(())
    }
    fn visit_parameter(&mut self, parameter: &mut Parameter) -> Result<(), Self::Error> {
        self.extract_parameter(parameter)?;
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
        self.extract_event_parameter(param)?;
        self.visit_expr(param.ty.loc(), &mut param.ty)?;
        if let Some(ref mut ident) = param.name {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_error_parameter(&mut self, param: &mut ErrorParameter) -> Result<(), Self::Error> {
        self.extract_error_parameter(param)?;
        self.visit_expr(param.ty.loc(), &mut param.ty)?;
        if let Some(ref mut ident) = param.name {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }
    fn visit_type_definition(&mut self, def: &mut TypeDefinition) -> Result<(), Self::Error> {
        self.extract_type_definition(def)?;
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
        self.extract_using(using)?;
        self.visit_using_list(&mut using.list)?;
        if let Some(ref mut ty) = using.ty {
            self.visit_expr(ty.loc(), ty)?;
        }
        if let Some(ref mut ident) = using.global {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_using_list(&mut self, using_list: &mut UsingList) -> Result<(), Self::Error> {
        self.extract_using_list(using_list)?;
        match using_list {
            UsingList::Library(path) => {
                self.visit_using_library(path)?;
            }
            UsingList::Functions(functions) => {
                for function in functions.iter_mut() {
                    self.visit_using_function(function)?;
                }
            }
            UsingList::Error => self.visit_using_error()?,
        }
        Ok(())
    }

    fn visit_using_error(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_using_library(&mut self, path: &mut IdentifierPath) -> Result<(), Self::Error> {
        self.extract_using_library(path)?;
        self.visit_ident_path(path)?;
        Ok(())
    }

    fn visit_using_function(&mut self, function: &mut UsingFunction) -> Result<(), Self::Error> {
        self.extract_using_function(function)?;
        self.visit_ident_path(&mut function.path)?;
        if let Some(ref mut op) = function.oper {
            self.visit_user_defined_operator(op)?;
        }
        Ok(())
    }

    fn visit_user_defined_operator(
        &mut self,
        _op: &mut UserDefinedOperator,
    ) -> Result<(), Self::Error> {
        self.extract_user_defined_operator(_op)?;
        Ok(())
    }
    fn visit_yul_block(
        &mut self,
        _loc: Loc,
        _stmts: &mut Vec<YulStatement>,
        _attempt_single_line: bool,
    ) -> Result<(), Self::Error> {
        self.extract_yul_block(_loc, _stmts, _attempt_single_line)?;
        for stmt in _stmts.iter_mut() {
            self.visit_yul_statement(stmt)?;
        }
        Ok(())
    }

    fn visit_yul_statement(&mut self, _stmt: &mut YulStatement) -> Result<(), Self::Error> {
        self.extract_yul_statement(_stmt)?;
        match _stmt {
            YulStatement::Assign(loc, exprs, expr) => {
                self.visit_yul_assignment(*loc, exprs, &mut Some(expr))?;
            }
            YulStatement::VariableDeclaration(loc, idents, expr) => {
                self.visit_yul_var_declaration(*loc, idents, expr)?;
            }
            YulStatement::If(loc, expr, block) => {
                self.visit_yul_if(*loc, expr, block)?;
            }
            YulStatement::For(stmt) => {
                self.visit_yul_for(stmt)?;
            }
            YulStatement::Switch(stmt) => {
                self.visit_yul_switch(stmt)?;
            }
            YulStatement::Leave(loc) => {
                self.visit_yul_leave(*loc)?;
            }
            YulStatement::Break(loc) => {
                self.visit_yul_break(*loc)?;
            }
            YulStatement::Continue(loc) => {
                self.visit_yul_continue(*loc)?;
            }
            YulStatement::Block(stmt) => {
                self.visit_yul_block(stmt.loc, &mut stmt.statements, false)?;
            }
            YulStatement::FunctionDefinition(stmt) => {
                self.visit_yul_fun_def(stmt)?;
            }
            YulStatement::FunctionCall(stmt) => {
                self.visit_yul_function_call(stmt)?;
            }
            YulStatement::Error(loc) => {
                self.visit_yul_error(*loc)?;
            }
        }
        Ok(())
    }

    fn visit_yul_error(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_break(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_continue(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_expr(&mut self, _loc: Loc, expr: &mut YulExpression) -> Result<(), Self::Error> {
        self.extract_yul_expr(_loc, expr)?;
        match expr {
            YulExpression::BoolLiteral(loc, value, ident) => {
                self.visit_yul_bool_literal(*loc, value, ident)?;
            }
            YulExpression::NumberLiteral(loc, string_0, string_1, ident) => {
                self.visit_yul_number_literal(*loc, string_0, string_1, ident)?;
            }
            YulExpression::HexNumberLiteral(loc, string, ident) => {
                self.visit_hex_number_literal(*loc, string, ident)?;
            }
            YulExpression::HexStringLiteral(expr, ident) => {
                self.visit_yul_hex_string_literal(expr, ident)?;
            }
            YulExpression::StringLiteral(expr, ident) => {
                self.visit_yul_string_literal(expr, ident)?;
            }
            YulExpression::Variable(ident) => {
                self.visit_variable(ident.loc, ident)?;
            }
            YulExpression::FunctionCall(expr) => {
                self.visit_yul_function_call(expr)?;
            }
            YulExpression::SuffixAccess(loc, expr, ident) => {
                self.visit_yul_suffix_access(*loc, expr, ident)?;
            }
        }
        Ok(())
    }

    fn visit_yul_hex_string_literal(
        &mut self,
        expr: &mut HexLiteral,
        ident: &mut Option<Identifier>,
    ) -> Result<(), Self::Error> {
        self.extract_yul_hex_string_literal(expr, ident)?;
        self.visit_hex_literal(expr)?;

        if let Some(ref mut ident) = ident {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_yul_suffix_access(
        &mut self,
        _loc: Loc,
        _expr: &mut YulExpression,
        _ident: &mut Identifier,
    ) -> Result<(), Self::Error> {
        self.extract_yul_suffix_access(_loc, _expr, _ident)?;
        self.visit_yul_expr(_expr.loc(), _expr)?;
        self.visit_ident(_ident.loc, _ident)?;
        Ok(())
    }

    fn visit_yul_string_literal(
        &mut self,
        string_literal: &mut StringLiteral,
        ident: &mut Option<Identifier>,
    ) -> Result<(), Self::Error> {
        self.extract_yul_string_literal(string_literal, ident)?;
        self.visit_string_literal(string_literal)?;
        if let Some(ref mut ident) = ident {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_yul_number_literal(
        &mut self,
        _loc: Loc,
        _number: &mut String,
        _suffix: &mut String,
        _ident: &mut Option<Identifier>,
    ) -> Result<(), Self::Error> {
        self.extract_yul_number_literal(_loc, _number, _suffix, _ident)?;
        if let Some(ident) = _ident {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }

    fn visit_yul_bool_literal(
        &mut self,
        _loc: Loc,
        _val: &mut bool,
        _ident: &mut Option<Identifier>,
    ) -> Result<(), Self::Error> {
        self.extract_yul_bool_literal(_loc, _val, _ident)?;
        if let Some(ident) = _ident {
            self.visit_ident(ident.loc, ident)?;
        }
        Ok(())
    }
    fn visit_yul_assignment(
        &mut self,
        _loc: Loc,
        _exprs: &mut Vec<YulExpression>,
        _expr: &mut Option<&mut YulExpression>,
    ) -> Result<(), Self::Error> {
        self.extract_yul_assignment(_loc, _exprs, _expr)?;
        for expr in _exprs {
            self.visit_yul_expr(expr.loc(), expr)?;
        }
        if let Some(expr) = _expr {
            self.visit_yul_expr(expr.loc(), expr)?;
        }
        Ok(())
    }

    fn visit_yul_for(&mut self, _stmt: &mut YulFor) -> Result<(), Self::Error> {
        self.extract_yul_for(_stmt)?;
        self.visit_yul_block(
            _stmt.init_block.loc,
            &mut _stmt.init_block.statements,
            false,
        )?;
        self.visit_yul_expr(_stmt.condition.loc(), &mut _stmt.condition)?;
        self.visit_yul_block(
            _stmt.post_block.loc,
            &mut _stmt.post_block.statements,
            false,
        )?;
        self.visit_yul_block(
            _stmt.execution_block.loc,
            &mut _stmt.execution_block.statements,
            false,
        )?;
        Ok(())
    }

    fn visit_yul_function_call(&mut self, _stmt: &mut YulFunctionCall) -> Result<(), Self::Error> {
        self.extract_yul_function_call(_stmt)?;
        self.visit_ident(_stmt.id.loc, &mut _stmt.id)?;
        for arg in _stmt.arguments.iter_mut() {
            self.visit_yul_expr(arg.loc(), arg)?;
        }
        Ok(())
    }

    fn visit_yul_fun_def(&mut self, _stmt: &mut YulFunctionDefinition) -> Result<(), Self::Error> {
        self.extract_yul_fun_def(_stmt)?;
        self.visit_ident(_stmt.id.loc, &mut _stmt.id)?;
        for param in _stmt.params.iter_mut() {
            self.visit_yul_typed_ident(param)?;
        }
        for ret in _stmt.returns.iter_mut() {
            self.visit_yul_typed_ident(ret)?;
        }
        self.visit_yul_block(_stmt.body.loc, &mut _stmt.body.statements, false)?;
        Ok(())
    }

    fn visit_yul_if(
        &mut self,
        _loc: Loc,
        _expr: &mut YulExpression,
        _block: &mut YulBlock,
    ) -> Result<(), Self::Error> {
        self.extract_yul_if(_loc, _expr, _block)?;
        self.visit_yul_expr(_expr.loc(), _expr)?;
        self.visit_yul_block(_block.loc, &mut _block.statements, false)?;
        Ok(())
    }

    fn visit_yul_leave(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_yul_switch(&mut self, _stmt: &mut YulSwitch) -> Result<(), Self::Error> {
        self.extract_yul_switch(_stmt)?;
        self.visit_yul_expr(_stmt.condition.loc(), &mut _stmt.condition)?;
        for case in _stmt.cases.iter_mut() {
            self.visit_yul_switch_options(case)?;
        }
        if let Some(default) = _stmt.default.as_mut() {
            self.visit_yul_switch_options(default)?;
        }
        Ok(())
    }

    fn visit_yul_switch_options(
        &mut self,
        _stmt: &mut YulSwitchOptions,
    ) -> Result<(), Self::Error> {
        self.extract_yul_switch_options(_stmt)?;
        match _stmt {
            YulSwitchOptions::Case(loc, expr, block) => {
                self.visit_yul_switch_case(*loc, expr, block)?;
            }
            YulSwitchOptions::Default(loc, block) => {
                self.visit_yul_switch_default(*loc, block)?;
            }
        }
        Ok(())
    }

    fn visit_yul_switch_default(
        &mut self,
        _loc: Loc,
        _block: &mut YulBlock,
    ) -> Result<(), Self::Error> {
        self.extract_yul_switch_default(_loc, _block)?;
        self.visit_yul_block(_block.loc, &mut _block.statements, false)?;
        Ok(())
    }

    fn visit_yul_switch_case(
        &mut self,
        _loc: Loc,
        _expr: &mut YulExpression,
        _block: &mut YulBlock,
    ) -> Result<(), Self::Error> {
        self.extract_yul_switch_case(_loc, _expr, _block)?;
        self.visit_yul_expr(_expr.loc(), _expr)?;
        self.visit_yul_block(_block.loc, &mut _block.statements, false)?;
        Ok(())
    }
    fn visit_yul_var_declaration(
        &mut self,
        _loc: Loc,
        _idents: &mut Vec<YulTypedIdentifier>,
        _expr: &mut Option<YulExpression>,
    ) -> Result<(), Self::Error> {
        self.extract_yul_var_declaration(_loc, _idents, _expr)?;
        for ident in _idents {
            self.visit_yul_typed_ident(ident)?;
        }
        if let Some(expr) = _expr {
            self.visit_yul_expr(expr.loc(), expr)?;
        }
        Ok(())
    }

    fn visit_yul_typed_ident(&mut self, ident: &mut YulTypedIdentifier) -> Result<(), Self::Error> {
        self.extract_yul_typed_ident(ident)?;
        self.visit_ident(ident.id.loc, &mut ident.id)?;

        Ok(())
    }

    fn visit_parser_error(&mut self, _loc: Loc) -> Result<(), Self::Error> {
        Ok(())
    }

    extract!(extract_source_unit, _source_unit: &mut SourceUnit);
    extract!(extract_source_unit_part, _source_unit_part: &mut SourceUnitPart);
    extract!(extract_function, _function: &mut FunctionDefinition);
    extract!(extract_statement, _statement: &mut Statement);
    extract!(extract_named_arg, _arg: &mut NamedArgument);
    extract!(extract_returns, _returns: &mut ParameterList);
    extract!(extract_function_attributes, _attributes: &mut Vec<FunctionAttribute>);
    extract!(extract_function_attribute, _attribute: &mut FunctionAttribute);
    extract!(extract_parameter_list, _parameter_list: &mut ParameterList);
    extract!(extract_function_type, _ty: &mut FunctionTy);
    extract!(extract_error, _error: &mut ErrorDefinition);
    extract!(extract_event, _event: &mut EventDefinition);
    extract!(extract_struct, _structure: &mut StructDefinition);
    extract!(extract_fields, _fields: &mut Vec<VariableDeclaration>);
    extract!(extract_contract, _contract: &mut ContractDefinition);
    extract!(extract_contract_part, _contract_part: &mut ContractPart);
    extract!(extract_contract_type, _ty: &mut ContractTy);
    extract!(extract_import, _import: &mut Import);
    extract!(extract_enum, _enum_definition: &mut Box<EnumDefinition>);
    extract!(extract_annotation, _annotation: &mut Annotation);
    extract!(extract_pragma, _loc: Loc, _ident: &mut Option<Identifier>, _str: &mut Option<StringLiteral>);
    extract!(extract_import_plain, _loc: Loc, _import: &mut ImportPath);
    extract!(extract_import_global, _loc: Loc, _global: &mut ImportPath, _alias: &mut Identifier);
    extract!(extract_import_renames, _loc: Loc, _imports: &mut [(Identifier, Option<Identifier>)], _from: &mut ImportPath);
    extract!(extract_assembly, _loc: Loc, _dialect: &mut Option<StringLiteral>, _block: &mut YulBlock, _flags: &mut Option<Vec<StringLiteral>>);
    extract!(extract_block, _loc: Loc, _unchecked: bool, _statements: &mut Vec<Statement>);
    extract!(extract_args, _loc: Loc, _args: &mut Vec<NamedArgument>);
    extract!(extract_expr, _loc: Loc, _expr: &mut Expression);
    extract!(extract_list, _loc: Loc, _parameter_list: &mut Vec<(Loc, Option<Parameter>)>);
    extract!(extract_array_literal, _loc: Loc, _expr_vec: &mut Vec<Expression>);
    extract!(extract_hex_literal, _hex_literal: &mut HexLiteral);
    extract!(extract_variable, _loc: Loc, _ident: &mut Identifier);
    extract!(extract_address_literal, _loc: Loc, _value: &mut String);
    extract!(extract_type, _loc: Loc, _type: &mut Type);
    extract!(extract_string_literal_vec, _string_literal_vec: &mut Vec<StringLiteral>);
    extract!(extract_string_literal, _string_literal: &mut StringLiteral);
    extract!(extract_hex_number_literal, _loc: Loc, _string_0: &mut String, _ident: &mut Option<Identifier>);
    extract!(extract_rational_number_literal, _loc: Loc, _string_0: &mut String, _string_1: &mut String, _string_2: &mut String, _ident: &mut Option<Identifier>);
    extract!(extract_number_literal, _loc: Loc, _string_0: &mut String, _string_1: &mut String, _ident: &mut Option<Identifier>);
    extract!(extract_bool_literal, _loc: Loc, _value: &mut bool);
    extract!(extract_assign_add, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_assign_modulo, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_assign_divide, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_assign_multiply, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_assign_subtract, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_assign_shift_right, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_assign_shift_left, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_assign_and, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_assign_xor, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_assign_or, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_assign, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_or, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_and, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_not_equal, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_equal, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_more_equal, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_more, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_less_equal, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_less, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_bitwise_or, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_conditional_operator, _expr: &mut Expression, _expr1: &mut Expression, _expr2: &mut Expression);
    extract!(extract_bitwise_xor, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_bitwise_and, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_shift_right, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_shift_left, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_subtract, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_add, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_modulo, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_divide, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_multiply, _expr: &mut Expression, _expr1: &mut Expression);
    extract!(extract_power, _expr: &mut Expression, _expr2: &mut Expression);
    extract!(extract_negate, _expr: &mut Expression);
    extract!(extract_delete, _expr: &mut Expression);
    extract!(extract_bitwise_not, _expr: &mut Expression);
    extract!(extract_not, _expr: &mut Expression);
    extract!(extract_named_function_call, _loc: Loc, _ident: &mut Box<Expression>, _params: &mut Vec<NamedArgument>);
    extract!(extract_function_call_block, _loc: Loc, _expr: &mut Box<Expression>, _statement: &mut Box<Statement>);
    extract!(extract_function_call, _loc: Loc, _expr: &mut Box<Expression>, _params: &mut Vec<Expression>);
    extract!(extract_member_access, _loc: Loc, _expr: &mut Box<Expression>, _ident: &mut Identifier);
    extract!(extract_parenthesis, _loc: Loc, _expr: &mut Box<Expression>);
    extract!(extract_array_slice, _loc: Loc, _expr: &mut Box<Expression>, _option_expr_0: &mut Option<Box<Expression>>, _option_expr_1: &mut Option<Box<Expression>>);
    extract!(extract_array_subscript, _loc: Loc, _expr_0: &mut Box<Expression>, _expr_1: &mut Option<Box<Expression>>);
    extract!(extract_new, _loc: Loc, _expr: &mut Box<Expression>);
    extract!(extract_unary_plus, _loc: Loc, _expr: &mut Box<Expression>);
    extract!(extract_pre_decrement, _loc: Loc, _expr: &mut Box<Expression>);
    extract!(extract_post_decrement, _loc: Loc, _expr: &mut Box<Expression>);
    extract!(extract_pre_increment, _loc: Loc, _expr: &mut Box<Expression>);
    extract!(extract_post_increment, _loc: Loc, _expr: &mut Box<Expression>);
    extract!(extract_contract_ident, _loc: Loc, _ident: &mut Identifier);
    extract!(extract_ident, _loc: Loc, _ident: &mut Identifier);
    extract!(extract_ident_path, _identifier_path: &mut IdentifierPath);
    extract!(extract_emit, _loc: Loc, _event: &mut Expression);
    extract!(extract_var_definition, _var: &mut VariableDefinition);
    extract!(extract_var_definition_stmt, _loc: Loc, _declaration: &mut VariableDeclaration, _expr: &mut Option<Expression>);
    extract!(extract_var_declaration, _var_declaration: &mut VariableDeclaration);
    extract!(extract_storage_loc, _loc: Loc, _storage: &mut StorageLocation);
    extract!(extract_return, _loc: Loc, _expr: &mut Option<Expression>);
    extract!(extract_revert, _loc: Loc, _error: &mut Option<IdentifierPath>, _args: &mut Vec<Expression>);
    extract!(extract_revert_named_args, _loc: Loc, _error: &mut Option<IdentifierPath>, _args: &mut Vec<NamedArgument>);
    extract!(extract_break, _loc: Loc, _semicolon: bool);
    extract!(extract_continue, _loc: Loc, _semicolon: bool);
    extract!(extract_try, _loc: Loc, _expr: &mut Expression, _returns: &mut Option<(Vec<(Loc, Option<Parameter>)>, Box<Statement>)>, _clauses: &mut Vec<CatchClause>);
    extract!(extract_catch_clause, _clause: &mut CatchClause);
    extract!(extract_if, _loc: Loc, _cond: &mut Expression, _if_branch: &mut Box<Statement>, _else_branch: &mut Option<Box<Statement>>, _is_first_stmt: bool);
    extract!(extract_do_while, _loc: Loc, _body: &mut Statement, _cond: &mut Expression);
    extract!(extract_while, _loc: Loc, _cond: &mut Expression, _body: &mut Statement);
    extract!(extract_for, _loc: Loc, _init: &mut Option<Box<Statement>>, _cond: &mut Option<Box<Expression>>, _update: &mut Option<Box<Expression>>, _body: &mut Option<Box<Statement>>);
    extract!(extract_var_attribute, _variable_attribute: &mut VariableAttribute);
    extract!(extract_override, _loc: Loc, _paths: &mut Vec<IdentifierPath>);
    extract!(extract_visibility, _visibility: &mut Visibility);
    extract!(extract_base, _base: &mut Base);
    extract!(extract_parameter,_parameter: &mut Parameter);
    extract!(extract_event_parameter, _param: &mut EventParameter);
    extract!(extract_error_parameter, _param: &mut ErrorParameter);
    extract!(extract_type_definition, _def: &mut TypeDefinition);
    extract!(extract_using, _using: &mut Using);
    extract!(extract_using_list, _using_list: &mut UsingList);
    extract!(extract_using_library, _path: &mut IdentifierPath);
    extract!(extract_using_function, _function: &mut UsingFunction);
    extract!(extract_user_defined_operator, _op: &mut UserDefinedOperator);
    extract!(extract_yul_block, _loc: Loc, _stmts: &mut Vec<YulStatement>, _attempt_single_line: bool);
    extract!(extract_yul_statement, _stmt: &mut YulStatement);
    extract!(extract_yul_expr, _loc: Loc, _expr: &mut YulExpression);
    extract!(extract_yul_hex_string_literal, _expr: &mut HexLiteral, _ident: &mut Option<Identifier>);
    extract!(extract_yul_suffix_access, _loc: Loc, _expr: &mut YulExpression, _ident: &mut Identifier);
    extract!(extract_yul_string_literal, _string_literal: &mut StringLiteral, _ident: &mut Option<Identifier>);
    extract!(extract_yul_number_literal, _loc: Loc, _number: &mut String, _suffix: &mut String, _ident: &mut Option<Identifier>);
    extract!(extract_yul_bool_literal, _loc: Loc, _val: &mut bool, _ident: &mut Option<Identifier>);
    extract!(extract_yul_assignment, _loc: Loc, _exprs: &mut Vec<YulExpression>, _expr: &mut Option<&mut YulExpression>);
    extract!(extract_yul_for, _stmt: &mut YulFor);
    extract!(extract_yul_function_call, _stmt: &mut YulFunctionCall);
    extract!(extract_yul_fun_def, _stmt: &mut YulFunctionDefinition);
    extract!(extract_yul_if, _loc: Loc, _expr: &mut YulExpression, _block: &mut YulBlock);
    extract!(extract_yul_switch, _stmt: &mut YulSwitch);
    extract!(extract_yul_switch_options, _stmt: &mut YulSwitchOptions);
    extract!(extract_yul_switch_default, _loc: Loc, _block: &mut YulBlock);
    extract!(extract_yul_switch_case, _loc: Loc, _expr: &mut YulExpression, _block: &mut YulBlock);
    extract!(extract_yul_var_declaration, _loc: Loc, _idents: &mut Vec<YulTypedIdentifier>, _expr: &mut Option<YulExpression>);
    extract!(extract_yul_typed_ident, _ident: &mut YulTypedIdentifier);
}
