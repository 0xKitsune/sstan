use super::{
    primitive::ContractDefinitionExtractor, visitable::Visitable, visitor::Visitor,
    ExtractionError, Extractor, Target,
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

// fn extract(v: &mut V) -> Result<Vec<VariableDefinition>, ExtractionError> {
//     let mut extractor_instance = Self::new();
//     let contracts = ContractDefinitionExtractor::extract(v)?;

//     for node in contracts {
//         'outer: for part in node.parts {
//             if let ContractPart::VariableDefinition(box_variable_definition) = part {
//                 let mut variable_attributes: Option<Vec<VariableAttribute>> = None;
//                 //if the variable is constant, mark constant_variable as true

//                 if !box_variable_definition.attrs.is_empty() {
//                     for attribute in box_variable_definition.attrs.clone() {
//                         if let VariableAttribute::Constant(_) = attribute {
//                             if ignore_constants {
//                                 continue 'outer;
//                             }
//                         } else if let VariableAttribute::Immutable(_) = attribute {
//                             if ignore_immutables {
//                                 continue 'outer;
//                             }
//                         }
//                     }

//                     variable_attributes = Some(box_variable_definition.attrs);
//                 }

//                 if let pt::Expression::Type(loc, ty) = box_variable_definition.ty {
//                     if let pt::Type::Mapping { .. } = ty {
//                     } else if let Some(name) = box_variable_definition.name {
//                         storage_variables.insert(name.to_string(), (variable_attributes, loc));
//                     }
//                 }
//             }
//         }
//     }

//     todo!()
// }
