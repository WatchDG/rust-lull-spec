use crate::enums::side::Side;
use crate::types::external_order_id::ExternalOrderId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalOrder<EOIDI> {
    pub id: ExternalOrderId<EOIDI>,
    pub side: Side,
    pub created_at: Option<DateTime<Utc>>,
}
