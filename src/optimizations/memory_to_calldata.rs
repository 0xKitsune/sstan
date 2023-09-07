use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self, CodeLocation};
use solang_parser::{self, pt::SourceUnit};

use super::{MemoryToCalldata, OptimizationOutcome, OptimizationPattern};
use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::primitive::AssignmentExtractor;
use crate::extractors::{primitive::FunctionExtractor, Extractor};

impl OptimizationPattern for MemoryToCalldata {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
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
                                pt::Expression::ArraySubscript(
                                    _,
                                    arr_subscript_box_expression,
                                    _,
                                ) => {
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
                    for (_, parameter) in memory_args {
                        outcome.push_or_insert(
                            path_buf.clone(),
                            parameter.loc(),
                            parameter.to_string(),
                        )
                    }
                }
            }
        }
        //Return the identified optimization locations
        Ok(OptimizationOutcome::MemoryToCalldata(outcome))
    }
}

fn get_function_definition_memory_args(
    function_definition: pt::FunctionDefinition,
) -> HashMap<String, pt::Parameter> {
    let mut memory_args: HashMap<String, pt::Parameter> = HashMap::new();
    for option_param in function_definition.params {
        if let Some(param) = option_param.1 {
            if let Some(pt::StorageLocation::Memory(_)) = &param.storage {
                if let Some(name) = &param.name {
                    memory_args.insert(name.name.clone(), param);
                }
            }
        }
    }

    memory_args
}

mod test {
    use std::{fs::File, io::Write};

    use crate::{
        optimizations::{MemoryToCalldata, OptimizationPattern},
        report::ReportSectionFragment,
        utils::MockSource,
    };

    #[test]
    fn test_memory_to_calldata_optimization() -> eyre::Result<()> {
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

        let mut source = MockSource::new().add_source("memory_to_calldata.sol", file_contents);
        let optimization_locations = MemoryToCalldata::find(&mut source.source)?;
        assert_eq!(optimization_locations.len(), 2);
        let report: Option<ReportSectionFragment> = optimization_locations.into();
        if let Some(report) = report {
            let mut f = File::options()
                .append(true)
                .open("optimization_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }
        Ok(())
    }
}
