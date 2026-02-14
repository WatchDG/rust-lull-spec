use crate::types::instrument_id::InstrumentId;
use crate::types::lot_size::LotSize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instrument<II, LSI> {
    pub id: InstrumentId<II>,
    pub lot_size: LotSize<LSI>,
}
