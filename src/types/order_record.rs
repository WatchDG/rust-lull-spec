use crate::enums::side::Side;
use crate::types::order::Order;
use crate::types::order_id::OrderId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderRecord<OIDI> {
    pub id: OrderId<OIDI>,
    pub side: Side,
    pub created_at: DateTime<Utc>,
}

impl<OIDI, EOIDI> Into<Order<OIDI, EOIDI>> for OrderRecord<OIDI> {
    fn into(self) -> Order<OIDI, EOIDI> {
        Order {
            id: self.id,
            side: self.side,
            external_order_id: None,
        }
    }
}
