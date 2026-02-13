use crate::enums::side::Side;
use crate::types::external_order_id::ExternalOrderId;
use crate::types::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OIDI, EOIDI> {
    pub id: OrderId<OIDI>,
    pub external_order_id: Option<ExternalOrderId<EOIDI>>,
    pub side: Side,
}
