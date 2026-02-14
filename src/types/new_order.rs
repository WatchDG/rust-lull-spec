use crate::enums::order_size::OrderSize;
use crate::enums::side::Side;
use crate::types::instrument_id::InstrumentId;
use crate::types::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewOrder<OIDI, II, QI, LI, LSI> {
    pub id: Option<OrderId<OIDI>>,
    pub instrument_id: InstrumentId<II>,
    pub side: Side,
    pub size: OrderSize<QI, LI, LSI>,
}
