use std::collections::HashSet;

use solang_parser::pt::{Loc, StructDefinition};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::{primitive::StructDefinitionExtractor, Extractor};
use crate::utils;

///Identifiy opportunities to pack structs to save gas
pub fn pack_struct_variables_optimization(
    source_unit: &mut SourceUnit,
) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let target_nodes = StructDefinitionExtractor::extract(source_unit)?;
    for node in target_nodes {
        if struct_can_be_packed(node.clone()) {
            optimization_locations.insert(node.loc);
        }
    }

    Ok(optimization_locations)
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

#[test]
fn test_pack_struct_variables_optimization() {
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

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = pack_struct_variables_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 4)
}
