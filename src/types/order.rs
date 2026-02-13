use crate::traits::order::ReadOrder;
use crate::traits::order_id::ReadOrderId;
use crate::types::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OI, OIDI> {
    pub inner: OI,
    pub id: OrderId<OIDI>,
}

impl<OI, OIDI> Order<OI, OIDI>
where
    OI: ReadOrderId<OIDI>,
{
    pub fn new(inner: OI) -> Self {
        let id = inner.read_order_id();
        Self { inner, id }
    }
}

impl<OI: Clone, OIDI: Clone> ReadOrder<OI, OIDI> for Order<OI, OIDI> {
    fn read_order(&self) -> Order<OI, OIDI> {
        self.clone()
    }
}
