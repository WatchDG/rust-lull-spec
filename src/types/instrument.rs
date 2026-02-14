use crate::types::instrument_id::InstrumentId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instrument<II> {
    pub id: InstrumentId<II>,
}
