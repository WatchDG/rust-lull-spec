use crate::types::lot_size::LotSize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lots<LI, LSI> {
    pub lots: LI,
    pub lot_size: LotSize<LSI>,
}
