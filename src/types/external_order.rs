use crate::enums::side::Side;
use crate::types::datetime::DateTime;
use crate::types::external_order_id::ExternalOrderId;

pub struct ExternalOrder<EOIDI, DTI> {
    pub id: ExternalOrderId<EOIDI>,
    pub side: Side,
    pub created_at: DateTime<DTI>,
}
