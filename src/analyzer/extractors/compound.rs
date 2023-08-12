use std::str::FromStr;

use super::{
    primitive::{ContractDefinitionExtractor, PragmaDirectiveExtractor},
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
        let storage_variables = contracts
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
        if let SourceUnitPart::PragmaDirective(_, _, Some(version)) = &pragma_directive[0] {
            let solidity_version = SolidityVersion::from_str(&version.string)?;

            Ok(vec![Some(solidity_version)])
        } else {
            Ok(vec![None])
        }
    }
}
