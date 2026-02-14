use crate::enums::side::Side;
use crate::types::instrument_id::InstrumentId;
use crate::types::order::Order;
use crate::types::order_id::OrderId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderRecord<OIDI, II> {
    pub id: OrderId<OIDI>,
    pub side: Side,
    pub instrument_id: InstrumentId<II>,
    pub created_at: DateTime<Utc>,
}

impl<OIDI, EOIDI, II> Into<Order<OIDI, EOIDI, II>> for OrderRecord<OIDI, II> {
    fn into(self) -> Order<OIDI, EOIDI, II> {
        Order {
            id: self.id,
            side: self.side,
            external_order_id: None,
            instrument_id: self.instrument_id,
        }
    }
}
