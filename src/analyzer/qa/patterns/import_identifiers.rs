use std::collections::HashSet;

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::analyzer::extractors::{primitive::PlainImportExtractor, Extractor};

pub fn import_identifiers(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut qa_locations: HashSet<Loc> = HashSet::new();

    let plain_imports = PlainImportExtractor::extract(source_unit)?;
    for import in plain_imports {
        if let pt::Import::Plain(_, loc) = import {
            qa_locations.insert(loc);
        }
    }

    Ok(qa_locations)
}

#[test]
fn test_import_identifiers() {
    let file_contents = r#"
    import "filename.sol";
    contract Contract0 {
        
        function msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgData() private view returns(bytes calldata) {
            return msg.data;
        }

        function msgData() private view returns(bytes calldata) {
            return msg.data;
        }
    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let qa_locations = import_identifiers(&mut source_unit);
    assert_eq!(qa_locations.unwrap().len(), 1);
}
