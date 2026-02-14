use crate::enums::order_size::OrderSize;
use crate::enums::side::Side;
use crate::types::datetime::DateTime;
use crate::types::instrument_id::InstrumentId;
use crate::types::order::Order;
use crate::types::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderRecord<OIDI, II, QI, LI, LSI, DTI> {
    pub id: OrderId<OIDI>,
    pub side: Side,
    pub instrument_id: InstrumentId<II>,
    pub size: OrderSize<QI, LI, LSI>,
    pub created_at: DateTime<DTI>,
}

impl<OIDI, EOIDI, II, QI, LI, LSI, DTI> Into<Order<OIDI, EOIDI, II, QI, LI, LSI>>
    for OrderRecord<OIDI, II, QI, LI, LSI, DTI>
{
    fn into(self) -> Order<OIDI, EOIDI, II, QI, LI, LSI> {
        Order {
            id: self.id,
            side: self.side,
            external_order_id: None,
            instrument_id: self.instrument_id,
            size: self.size,
        }
    }
}
