use crate::enums::side::Side;
use crate::types::external_order_id::ExternalOrderId;
use crate::types::instrument_id::InstrumentId;
use crate::types::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OIDI, EOIDI, II> {
    pub id: OrderId<OIDI>,
    pub external_order_id: Option<ExternalOrderId<EOIDI>>,
    pub instrument_id: InstrumentId<II>,
    pub side: Side,
}
