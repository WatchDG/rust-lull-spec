use crate::enums::order_size::OrderSize;
use crate::enums::order_type::OrderType;
use crate::enums::side::Side;
use crate::types::external_order_id::ExternalOrderId;
use crate::types::instrument_id::InstrumentId;
use crate::types::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OIDI, EOIDI, II, QI, LI, LSI> {
    pub id: OrderId<OIDI>,
    pub external_order_id: Option<ExternalOrderId<EOIDI>>,
    pub instrument_id: InstrumentId<II>,
    pub side: Side,
    pub size: OrderSize<QI, LI, LSI>,
    pub r#type: OrderType,
}
