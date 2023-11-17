use std::{collections::HashSet, str::FromStr, vec};

use super::{
    primitive::{
        ContractDefinitionExtractor, FunctionCallExtractor, FunctionExtractor,
        PragmaDirectiveExtractor, YulFunctionCallExtractor,
    },
    visitable::Visitable,
    visitor::Visitor,
    ExtractionError, Extractor, SolidityVersion,
};
use crate::compound_extractor;
use solang_parser::pt::*;

compound_extractor!(StorageVariableExtractor, VariableDefinition);

impl<V: Visitable> Extractor<V, VariableDefinition> for StorageVariableExtractor {
    fn extract(v: &mut V) -> Result<Vec<VariableDefinition>, ExtractionError> {
        let contracts = ContractDefinitionExtractor::extract(v)?;
        let storage_variables: Vec<VariableDefinition> = contracts
            .iter()
            .flat_map(|contract| {
                contract.parts.iter().filter_map(|part| {
                    if let ContractPart::VariableDefinition(variable_definition) = part {
                        Some(*variable_definition.clone())
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<VariableDefinition>>();

        Ok(storage_variables)
    }
}

impl StorageVariableExtractor {
    pub fn extract_names(storage_variables: Vec<VariableDefinition>) -> HashSet<String> {
        let mut names = HashSet::new();
        for variable in storage_variables {
            if let Some(name) = variable.name {
                names.insert(name.name);
            }
        }

        names
    }
}

compound_extractor!(ContractExtractor, ContractDefinition);

impl<V: Visitable> Extractor<V, ContractDefinition> for ContractExtractor {
    fn extract(v: &mut V) -> Result<Vec<ContractDefinition>, ExtractionError> {
        let contracts = ContractDefinitionExtractor::extract(v)?;
        let filtered_contracts = contracts
            .iter()
            .filter_map(|contract| {
                if matches!(contract.ty, ContractTy::Contract(_)) {
                    Some(contract.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<ContractDefinition>>();

        Ok(filtered_contracts)
    }
}

compound_extractor!(AbstractContractExtractor, ContractDefinition);

impl<V: Visitable> Extractor<V, ContractDefinition> for AbstractContractExtractor {
    fn extract(v: &mut V) -> Result<Vec<ContractDefinition>, ExtractionError> {
        let contracts = ContractDefinitionExtractor::extract(v)?;
        let filtered_contracts = contracts
            .iter()
            .filter_map(|contract| {
                if matches!(contract.ty, ContractTy::Abstract(_)) {
                    Some(contract.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<ContractDefinition>>();

        Ok(filtered_contracts)
    }
}

compound_extractor!(InterfaceExtractor, ContractDefinition);

impl<V: Visitable> Extractor<V, ContractDefinition> for InterfaceExtractor {
    fn extract(v: &mut V) -> Result<Vec<ContractDefinition>, ExtractionError> {
        let contracts = ContractDefinitionExtractor::extract(v)?;
        let filtered_contracts = contracts
            .iter()
            .filter_map(|contract| {
                if matches!(contract.ty, ContractTy::Interface(_)) {
                    Some(contract.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<ContractDefinition>>();

        Ok(filtered_contracts)
    }
}

compound_extractor!(LibraryExtractor, ContractDefinition);

impl<V: Visitable> Extractor<V, ContractDefinition> for LibraryExtractor {
    fn extract(v: &mut V) -> Result<Vec<ContractDefinition>, ExtractionError> {
        let contracts = ContractDefinitionExtractor::extract(v)?;
        let filtered_contracts = contracts
            .iter()
            .filter_map(|contract| {
                if matches!(contract.ty, ContractTy::Library(_)) {
                    Some(contract.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<ContractDefinition>>();

        Ok(filtered_contracts)
    }
}

compound_extractor!(ConstructorExtractor, FunctionDefinition);
#[allow(clippy::unnecessary_filter_map)]
impl<V: Visitable> Extractor<V, FunctionDefinition> for ConstructorExtractor {
    fn extract(v: &mut V) -> Result<Vec<FunctionDefinition>, ExtractionError> {
        let functions = FunctionExtractor::extract(v)?;
        let constructors = functions
            .iter()
            .filter_map(|function| {
                if let FunctionTy::Constructor = function.ty {
                    Some(function)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<FunctionDefinition>>();
        Ok(constructors)
    }
}

compound_extractor!(ContractPartFunctionExtractor, FunctionDefinition);

impl<V: Visitable> Extractor<V, FunctionDefinition> for ContractPartFunctionExtractor {
    fn extract(v: &mut V) -> Result<Vec<FunctionDefinition>, ExtractionError> {
        let mut contracts = ContractDefinitionExtractor::extract(v)?;
        let mut contract_part_functions = vec![];
        for contract in contracts.iter_mut() {
            contract_part_functions.extend(FunctionExtractor::extract(contract)?);
        }

        Ok(contract_part_functions)
    }
}

compound_extractor!(ImmutableStorageVariableExtractor, VariableDefinition);

impl<V: Visitable> Extractor<V, VariableDefinition> for ImmutableStorageVariableExtractor {
    fn extract(v: &mut V) -> Result<Vec<VariableDefinition>, ExtractionError> {
        let contracts = ContractDefinitionExtractor::extract(v)?;
        let storage_variables =
            contracts
                .iter()
                .flat_map(|contract| {
                    contract.parts.iter().filter_map(|part| {
                        if let ContractPart::VariableDefinition(variable_definition) = part {
                            // Check if there's any attribute that's an Immutable
                            if variable_definition.attrs.iter().any(|attribute| {
                                matches!(attribute, VariableAttribute::Immutable(_))
                            }) {
                                Some(*variable_definition.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<VariableDefinition>>();

        Ok(storage_variables)
    }
}

compound_extractor!(PublicFunctionExtractor, FunctionDefinition);

impl<V: Visitable> Extractor<V, FunctionDefinition> for PublicFunctionExtractor {
    fn extract(v: &mut V) -> Result<Vec<FunctionDefinition>, ExtractionError> {
        let functions = FunctionExtractor::extract(v)?;
        let public_functions = functions
            .iter()
            .filter_map(|function| {
                // Check if there's any attribute that's an Immutable
                if function.attributes.iter().any(|attribute| {
                    matches!(
                        attribute,
                        FunctionAttribute::Visibility(Visibility::Public(_))
                    )
                }) {
                    Some(function.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<FunctionDefinition>>();
        Ok(public_functions)
    }
}

compound_extractor!(ConstantStorageVariableExtractor, VariableDefinition);

impl<V: Visitable> Extractor<V, VariableDefinition> for ConstantStorageVariableExtractor {
    fn extract(v: &mut V) -> Result<Vec<VariableDefinition>, ExtractionError> {
        let contracts = ContractDefinitionExtractor::extract(v)?;
        let storage_variables =
            contracts
                .iter()
                .flat_map(|contract| {
                    contract.parts.iter().filter_map(|part| {
                        if let ContractPart::VariableDefinition(variable_definition) = part {
                            // Check if there's any attribute that's an constant
                            if variable_definition.attrs.iter().any(|attribute| {
                                matches!(attribute, VariableAttribute::Constant(_))
                            }) {
                                Some(*variable_definition.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<VariableDefinition>>();

        Ok(storage_variables)
    }
}

compound_extractor!(MutableStorageVariableExtractor, VariableDefinition);

impl<V: Visitable> Extractor<V, VariableDefinition> for MutableStorageVariableExtractor {
    fn extract(v: &mut V) -> Result<Vec<VariableDefinition>, ExtractionError> {
        let contracts = ContractDefinitionExtractor::extract(v)?;
        let storage_variables = contracts
            .iter()
            .flat_map(|contract| {
                contract.parts.iter().filter_map(|part| {
                    if let ContractPart::VariableDefinition(variable_definition) = part {
                        // Check if the variable is not constant or immutable
                        if variable_definition.attrs.iter().any(|attribute| {
                            (matches!(attribute, VariableAttribute::Constant(_))
                                || matches!(attribute, VariableAttribute::Immutable(_)))
                        }) {
                            None
                        } else {
                            Some(*variable_definition.clone())
                        }
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<VariableDefinition>>();

        Ok(storage_variables)
    }
}

compound_extractor!(SolidityVerisonExtractor, Option<SolidityVersion>);

impl<V: Visitable> Extractor<V, Option<SolidityVersion>> for SolidityVerisonExtractor {
    fn extract(v: &mut V) -> Result<Vec<Option<SolidityVersion>>, ExtractionError> {
        let pragma_directive = PragmaDirectiveExtractor::extract(v)?;

        //TODO: right now this is only getting the first one, we should loop through this in case we are going through multiple contracts
        if pragma_directive.is_empty() {
            return Ok(vec![None]);
        };
        if let SourceUnitPart::PragmaDirective(_, _, Some(version)) = &pragma_directive[0] {
            let solidity_version = SolidityVersion::from_str(&version.string)?;

            Ok(vec![Some(solidity_version)])
        } else {
            Ok(vec![None])
        }
    }
}

compound_extractor!(InternalFunctionExtractor, FunctionDefinition);

impl<V: Visitable> Extractor<V, FunctionDefinition> for InternalFunctionExtractor {
    fn extract(v: &mut V) -> Result<Vec<FunctionDefinition>, ExtractionError> {
        let functions = FunctionExtractor::extract(v)?;
        let internal_functions = functions
            .iter()
            .flat_map(|function| {
                function
                    .attributes
                    .iter()
                    .filter_map(|attr: &FunctionAttribute| {
                        if let FunctionAttribute::Visibility(Visibility::Internal(_)) = attr {
                            Some(function.clone())
                        } else {
                            None
                        }
                    })
            })
            .collect::<Vec<FunctionDefinition>>();

        Ok(internal_functions)
    }
}

compound_extractor!(ExternalFunctionExtractor, FunctionDefinition);

impl<V: Visitable> Extractor<V, FunctionDefinition> for ExternalFunctionExtractor {
    fn extract(v: &mut V) -> Result<Vec<FunctionDefinition>, ExtractionError> {
        let functions = FunctionExtractor::extract(v)?;
        let internal_functions = functions
            .iter()
            .flat_map(|function| {
                function
                    .attributes
                    .iter()
                    .filter_map(|attr: &FunctionAttribute| {
                        if let FunctionAttribute::Visibility(Visibility::External(_)) = attr {
                            Some(function.clone())
                        } else {
                            None
                        }
                    })
            })
            .collect::<Vec<FunctionDefinition>>();

        Ok(internal_functions)
    }
}

compound_extractor!(PrivateFunctionExtractor, FunctionDefinition);

impl<V: Visitable> Extractor<V, FunctionDefinition> for PrivateFunctionExtractor {
    fn extract(v: &mut V) -> Result<Vec<FunctionDefinition>, ExtractionError> {
        let functions = FunctionExtractor::extract(v)?;
        let internal_functions = functions
            .iter()
            .flat_map(|function| {
                function
                    .attributes
                    .iter()
                    .filter_map(|attr: &FunctionAttribute| {
                        if let FunctionAttribute::Visibility(Visibility::Private(_)) = attr {
                            Some(function.clone())
                        } else {
                            None
                        }
                    })
            })
            .collect::<Vec<FunctionDefinition>>();

        Ok(internal_functions)
    }
}

compound_extractor!(WriteFunctionExtractor, FunctionDefinition);

impl<V: Visitable> Extractor<V, FunctionDefinition> for WriteFunctionExtractor {
    fn extract(v: &mut V) -> Result<Vec<FunctionDefinition>, ExtractionError> {
        let functions = FunctionExtractor::extract(v)?;
        let non_view_functions = functions
            .iter()
            .flat_map(|func| {
                func.attributes.iter().filter_map(|attr| match attr {
                    FunctionAttribute::Mutability(Mutability::View(_))
                    | FunctionAttribute::Mutability(Mutability::Pure(_)) => None,
                    _ => Some(func.clone()),
                })
            })
            .collect::<Vec<FunctionDefinition>>();

        Ok(non_view_functions)
    }
}

compound_extractor!(ReadFunctionExtractor, FunctionDefinition);

impl<V: Visitable> Extractor<V, FunctionDefinition> for ReadFunctionExtractor {
    fn extract(v: &mut V) -> Result<Vec<FunctionDefinition>, ExtractionError> {
        let functions = FunctionExtractor::extract(v)?;
        let non_view_functions = functions
            .iter()
            .flat_map(|func| {
                func.attributes.iter().filter_map(|attr| match attr {
                    FunctionAttribute::Mutability(Mutability::View(_))
                    | FunctionAttribute::Mutability(Mutability::Pure(_)) => Some(func.clone()),
                    _ => None,
                })
            })
            .collect::<Vec<FunctionDefinition>>();

        Ok(non_view_functions)
    }
}

compound_extractor!(YulShiftExtractor, YulFunctionCall);

impl<V: Visitable> Extractor<V, YulFunctionCall> for YulShiftExtractor {
    fn extract(v: &mut V) -> Result<Vec<YulFunctionCall>, ExtractionError> {
        let functions = YulFunctionCallExtractor::extract(v)?;
        let shift_functions = functions
            .iter()
            .filter_map(|function| {
                if function.id.name == "shl" || function.id.name == "shr" {
                    Some(function.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<YulFunctionCall>>();

        Ok(shift_functions)
    }
}

compound_extractor!(RequireExtractor, Expression);

impl<V: Visitable> Extractor<V, Expression> for RequireExtractor {
    fn extract(v: &mut V) -> Result<Vec<Expression>, ExtractionError> {
        let function_calls = FunctionCallExtractor::extract(v)?;
        let require_function_calls = function_calls
            .iter()
            .filter_map(|function| {
                if let Expression::FunctionCall(_, function_identifier, _) = function.clone() {
                    if let Expression::Variable(identifier) = *function_identifier {
                        if identifier.name == "require" || identifier.name == "revert" {
                            return Some(function.clone());
                        }
                    }
                }
                None
            })
            .collect::<Vec<Expression>>();

        Ok(require_function_calls)
    }
}
