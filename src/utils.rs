use crate::extractors::{primitive::ContractDefinitionExtractor, Extractor};
use regex::Regex;
use solang_parser::pt::{self, ContractPart, Loc};
use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub type LineNumber = i32;
pub type Outcome = (PathBuf, Loc);

//TODO: outcome should be updated to be code blocks, etc

// This is used as the initial string when parsing a solidity version
pub const ZERO_ZERO_ZERO: &str = "0.0.0";
pub const MINOR_MAJOR_PATCH_REGEX: &str = r"\d+\.\d+\.+\d+";
//Returns the size of the type in bits
pub fn get_type_size(expression: pt::Expression) -> u16 {
    if let pt::Expression::Type(_, ty) = expression {
        match ty {
            pt::Type::Address => return 160,
            pt::Type::AddressPayable => return 160,
            pt::Type::Bytes(_size) => return (_size as u16) * 8,
            pt::Type::Bool => return 8,
            pt::Type::Int(_size) => return _size,
            pt::Type::Uint(_size) => return _size,
            _ => return 256,
        }
    }

    //TODO: add error handling that bubbles up if the expression passed in is not a type
    256
}

//get line number of start of character range
pub fn get_line_number(char_number: usize, file_contents: &str) -> usize {
    let re = Regex::new(r"\n").unwrap();
    let mut i = 1;
    for capture in re.captures_iter(file_contents) {
        for c in capture.iter() {
            if c.unwrap().start() > char_number {
                //+1 since line numbers start at 1
                return i;
            } else {
                i += 1;
            }
        }
    }

    0
}

pub fn storage_slots_used(variables: Vec<u16>) -> u32 {
    //set a variable to keep track of how many bytes have been used in the slot
    let mut bytes_used_in_slot = 0;
    //--------------------- test slot usage of unordered variable sizes ---------------------------------------

    //loop through the unordered variable sizes and count the amount of slots used
    let mut slots_used = 0;
    for variable_size in variables {
        //if the next variable size
        if bytes_used_in_slot + variable_size > 256 {
            //add a slot used
            slots_used += 1;

            //update bytes used in slot
            bytes_used_in_slot = variable_size;
        } else {
            bytes_used_in_slot += variable_size;
        }
    }

    //if the bytes in slot is > 0 and the last variable has been accounted for, add one more slot used
    if bytes_used_in_slot > 0 {
        slots_used += 1;
    }

    slots_used
}

//TODO: move this to a compound extractor
pub fn get_32_byte_storage_variables(
    source_unit: &mut pt::SourceUnit,
    ignore_constants: bool,
    ignore_immutables: bool,
) -> HashMap<String, ContractPart> {
    let mut storage_variables: HashMap<String, ContractPart> = HashMap::new();

    let contracts =
        ContractDefinitionExtractor::extract(source_unit).expect("TODO: handle this error");
    for node in contracts {
        'outer: for part in node.parts {
            if let pt::ContractPart::VariableDefinition(box_variable_definition) = &part {
                //if the variable is constant, mark constant_variable as true

                if !box_variable_definition.attrs.is_empty() {
                    for attribute in box_variable_definition.attrs.clone() {
                        if let pt::VariableAttribute::Constant(_) = attribute {
                            if ignore_constants {
                                continue 'outer;
                            }
                        } else if let pt::VariableAttribute::Immutable(_) = attribute {
                            if ignore_immutables {
                                continue 'outer;
                            }
                        }
                    }
                }

                if let pt::Expression::Type(_, ty) = &box_variable_definition.ty {
                    if let pt::Type::Mapping { .. } = ty {
                    } else if let Some(name) = &box_variable_definition.name {
                        //TODO: need to update this to box varialb definition so that we can use the node in the report
                        storage_variables.insert(name.to_string(), part);
                    }
                }
            }
        }
    }

    storage_variables
}
#[derive(Debug)]
pub struct MockSource {
    pub source: HashMap<PathBuf, &'static mut pt::SourceUnit>,
    pub counter: usize,
}
impl Default for MockSource {
    fn default() -> Self {
        Self::new()
    }
}

impl MockSource {
    pub fn new() -> Self {
        Self {
            source: HashMap::new(),
            counter: 0,
        }
    }
}
impl MockSource {
    pub fn add_source(mut self, contents: &str) -> Self {
        let file_name: &str = &format!("test_source.sol");
        let path = PathBuf::from(format!("src/qa/{}", file_name));
        let mut file = File::options().append(true).open(&path.clone()).expect("Failed to create file").write_all(contents.as_bytes()).expect("Failed to write contents to file");
   
        let source_unit = solang_parser::parse(contents, self.counter).unwrap().0;
        let leaked_source_unit = Box::leak(Box::new(source_unit));
        self.source.insert(path, leaked_source_unit);
        self
    }
}

impl Drop for MockSource {
    fn drop(&mut self) {
        // for file in self.source.keys() {
        //     std::fs::remove_file(file).expect("Failed to delete file");
        // }
    }
}
//TODO: create a scruct
/// Macro to create a file with given name and content, used as a helper function during testing.
#[macro_export]
macro_rules! create_test_source {
    ($contents:expr) => {{
        use ::std::io::Write;
        const FILE_NAME: &str = "test.sol";
        let path = std::path::PathBuf::from(FILE_NAME);

        // Create the file
        let mut file = std::fs::File::create(&path).expect("Failed to create file");
        file.write_all($contents.as_bytes())
            .expect("Failed to write contents to file");

        let mut source = std::collections::HashMap::new();
        let source_unit = solang_parser::parse(&$contents, 0).unwrap().0;

        // Leak the SourceUnit to make it 'static
        let leaked_source_unit = Box::leak(Box::new(source_unit));
        source.insert(path.clone(), leaked_source_unit);

        source
    }};
}

/// Macro to delete a file with a given name, used as a helper function during testing.
#[macro_export]
macro_rules! cleanup_test_source {
    () => {{
        use std::fs::remove_file;

        const FILE_NAME: &str = "test.sol";
        let path = std::path::PathBuf::from(FILE_NAME);
        remove_file(&path).expect("Failed to delete file");
    }};
}
