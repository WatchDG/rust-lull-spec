use crate::enums::side::Side;
use crate::types::instrument_id::InstrumentId;
use crate::types::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewOrder<OIDI, II> {
    pub id: Option<OrderId<OIDI>>,
    pub instrument_id: InstrumentId<II>,
    pub side: Side,
}
