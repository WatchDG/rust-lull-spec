use crate::enums::side::Side;
use crate::types::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewOrder<OIDI> {
    pub id: OrderId<OIDI>,
    pub side: Side,
}
