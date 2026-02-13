use crate::enums::side::Side;
use crate::types::order_id::OrderId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersistentOrder<OIDI> {
    pub id: OrderId<OIDI>,
    pub side: Side,
    pub created_at: DateTime<Utc>,
}
