use std::collections::{HashMap, HashSet};

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::primitive::AssignmentExtractor;
use crate::analyzer::extractors::{primitive::FunctionExtractor, Extractor};

pub fn memory_to_calldata_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    //Extract the target nodes from the source_unit
    let mut target_nodes = FunctionExtractor::extract(source_unit)?;

    //For each target node that was extracted, check for the optimization patterns
    for node in target_nodes.iter_mut() {
        //extract the box function definition depending on if the node is a contract part or a source unit part

        let mut memory_args = get_function_definition_memory_args(node.clone());

        // Constructor can only use `memory`
        if node.ty == pt::FunctionTy::Constructor {
            continue;
        }

        if let Some(ref mut body) = node.body {
            let assign_nodes = AssignmentExtractor::extract(body)?;

            for assign_node in assign_nodes {
                if let pt::Expression::Assign(_, box_expression, _) = assign_node {
                    //check if the left hand side is a variable
                    match *box_expression {
                        //if assignment is to variable
                        pt::Expression::Variable(identifier) => {
                            memory_args.remove(&identifier.name);
                        }

                        //if assignment is array subscript
                        pt::Expression::ArraySubscript(_, arr_subscript_box_expression, _) => {
                            if let pt::Expression::Variable(identifier) =
                                *arr_subscript_box_expression
                            {
                                //remove the variable name from the memory_args hashmap
                                memory_args.remove(&identifier.name);
                            }
                        }

                        _ => {}
                    }
                }
            }

            //for each arg in memory args left, add it to the optimization locations
            for (_, loc) in memory_args {
                optimization_locations.insert(loc);
            }
        }
    }

    //Return the identified optimization locations
    Ok(optimization_locations)
}

fn get_function_definition_memory_args(
    function_definition: pt::FunctionDefinition,
) -> HashMap<String, Loc> {
    let mut memory_args: HashMap<String, Loc> = HashMap::new();
    for option_param in function_definition.params {
        if let Some(param) = option_param.1 {
            if let Some(pt::StorageLocation::Memory(loc)) = param.storage {
                if let Some(name) = param.name {
                    memory_args.insert(name.name.clone(), loc);
                }
            }
        }
    }

    memory_args
}

#[test]
fn test_memory_to_calldata_optimization() {
    let file_contents = r#"
   
contract Contract1 {

    constructor(uint256[] memory arr){
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
        }
    }

    //loop with i++
    function memoryArray(uint256[] memory arr) public {
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
        }
    }

    //loop with i++
    function calldataArray(uint256[] calldata arr) public {
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
        }
    }

    //loop with i++
    function memoryArray2(uint256[] memory arr) public {
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
            arr[i] = j + 10;
        }
    }

    //loop with i++
    function memoryBytes(bytes memory byteArr) public {
        bytes j;
        for (uint256 i; i < arr.length; i++) {
            j = byteArr;
        }
    }

    //loop with i++
    function calldataBytes(bytes calldata byteArr) public {
        bytes j;
        for (uint256 i; i < arr.length; i++) {
            j = byteArr;
        }
    }


    //loop with i++
    function memoryBytes1(bytes memory byteArr) public {
        bytes j;
        for (uint256 i; i < arr.length; i++) {
            byteArr = j;
        }
    }

}
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = memory_to_calldata_optimization(&mut source_unit);
    assert_eq!(optimization_locations.unwrap().len(), 2)
}
