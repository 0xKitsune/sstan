use crate::qa::QualityAssuranceOutcome;

use super::TableFragment;

//TODO: add this into the macro instead of here
impl Into<TableFragment> for QualityAssuranceOutcome {
    fn into(self) -> TableFragment {
        todo!()
    }
}
