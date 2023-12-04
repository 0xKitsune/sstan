use std::{collections::HashMap, path::PathBuf};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{compound::MappingExtractor, Extractor},
};
use solang_parser::{
    helpers::CodeLocation,
    pt::{Expression, Loc, SourceUnit, Type},
};

use super::{QAPattern, QualityAssuranceOutcome, NamedMappingParameters};
impl QAPattern for NamedMappingParameters {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let mappings = MappingExtractor::extract(source_unit)?;
            for mapping in mappings {
                if let Expression::Type(
                    _,
                    Type::Mapping {
                        loc: _,
                        key: _,
                        key_name,
                        value: _,
                        value_name,
                    },
                ) = mapping.ty.clone()
                {
                    if key_name.is_none() || value_name.is_none() {
                        outcome.push_or_insert(
                            path_buf.clone(),
                            mapping.loc(),
                            mapping.to_string(),
                        );
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::NamedMappingParameters(outcome))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_named_mapping_parameters() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract contract0 {

        mapping(uint => uint) public map0;
        mapping(uint => bytes) public map1;
        mapping(uint number1 => uint number2) public map2;
    }
 
    "#;

        let mut mock_source =
            MockSource::new().add_source("named_map_parameters.sol", file_contents);
        let qa_locations = NamedMappingParameters::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 2);

        Ok(())
    }
}
