use std::{collections::HashSet, str::FromStr, vec};

use super::{
    primitive::{ContractDefinitionExtractor, FunctionExtractor, PragmaDirectiveExtractor},
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

compound_extractor!(
    NonConstantImmutableStorageVariableExtractor,
    VariableDefinition
);

impl NonConstantImmutableStorageVariableExtractor {
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
impl<V: Visitable> Extractor<V, VariableDefinition>
    for NonConstantImmutableStorageVariableExtractor
{
    fn extract(v: &mut V) -> Result<Vec<VariableDefinition>, ExtractionError> {
        let storage_variables = StorageVariableExtractor::extract(v)?;
        let constant_storage_variables = ConstantStorageVariableExtractor::extract(v)?;
        let immutable_storage_variables = ImmutableStorageVariableExtractor::extract(v)?;

        let constant_storage_variable_names = Self::extract_names(constant_storage_variables);
        let immutable_storage_variable_names = Self::extract_names(immutable_storage_variables);

        let mut non_constant_immutable_storage_vars: Vec<VariableDefinition> = vec![];

        for variable in storage_variables {
            if let Some(name) = variable.name.clone() {
                if !constant_storage_variable_names.contains(&name.name)
                    && !immutable_storage_variable_names.contains(&name.name)
                {
                    non_constant_immutable_storage_vars.push(variable);
                }
            }
        }

        Ok(non_constant_immutable_storage_vars)
    }
}

compound_extractor!(ConstructorExtractor, FunctionDefinition);

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
                            !(matches!(attribute, VariableAttribute::Constant(_))
                                || matches!(attribute, VariableAttribute::Immutable(_)))
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

compound_extractor!(SolidityVerisonExtractor, Option<SolidityVersion>);

impl<V: Visitable> Extractor<V, Option<SolidityVersion>> for SolidityVerisonExtractor {
    fn extract(v: &mut V) -> Result<Vec<Option<SolidityVersion>>, ExtractionError> {
        let pragma_directive = PragmaDirectiveExtractor::extract(v)?;

        //TODO: right now this is only getting the first one, we should loop through this in case we are going through multiple contracts
        if pragma_directive.len() == 0 {
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