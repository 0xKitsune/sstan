use std::collections::HashMap;
use std::path::PathBuf;

use super::{OptimizationOutcome, OptimizationPattern, PackStructVariables};
use solang_parser::pt::{CodeLocation, StructDefinition};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::{primitive::StructDefinitionExtractor, Extractor};
use crate::utils;

///Identifiy opportunities to pack structs to save gas
impl OptimizationPattern for PackStructVariables {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let target_nodes = StructDefinitionExtractor::extract(source_unit)?;
            for node in target_nodes {
                if struct_can_be_packed(node.clone()) {
                    outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                }
            }
        }
        Ok(OptimizationOutcome::PackStructVariables(outcome))
    }
}

fn struct_can_be_packed(struct_definition: StructDefinition) -> bool {
    let mut variable_sizes: Vec<u16> = vec![];

    for variable_declaration in struct_definition.fields {
        variable_sizes.push(utils::get_type_size(variable_declaration.ty));
    }

    //create an unordered list of variable sizes
    let unordered_variable_sizes = variable_sizes.clone();

    //Sort the variable sizes
    variable_sizes.sort();

    //If the ordered version is smaller than the unordered
    utils::storage_slots_used(unordered_variable_sizes) > utils::storage_slots_used(variable_sizes)
}
mod test {
    #[allow(unused)]
    use crate::utils::MockSource;
    #[allow(unused)]
    use super::*;
    #[test]
    fn test_pack_struct_variables_optimization() -> eyre::Result<()> {
        let file_contents = r#"

    //should not match
    struct Ex {
        uint256 spotPrice;
        uint128 res0;
        uint128 res1;
    }

    //should match
    struct Ex1 {
        bool isUniV2;
        bytes32 salt;
        bytes16 initBytecode;
    }
    

contract OrderRouter {
  
    
    //should not match
    struct Ex2 {
        bool isUniV2;
        address factoryAddress;
        bytes16 initBytecode;
    }

    //should match
    struct Ex3 {
        bool isUniV2;
        bytes32 salt;
        bytes16 initBytecode;
    }

    //should not match
    struct Ex4 {
        bytes16 initBytecode;
        bool isUniV2;
        address factoryAddress;
    }

    //should not match
    struct Ex5 {
        bool isUniV2;
        bytes16 initBytecode;
        address factoryAddress;
    }

    //should match
    struct Ex6 {
        uint128 thing3;
        uint256 thing1;
        uint128 thing2;
    }

    // Should match
    struct Ex7 {
        address owner; // 160 bits
        uint256 num0;  // 256 bits
        bytes4 b0;     // 32 bits
        uint64 num1;   // 64 bits
    }
}
    "#;

        let mut source = MockSource::new().add_source("pack_struct_variables.sol", file_contents);
        let optimization_locations = PackStructVariables::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 4);

        Ok(())
    }
}
