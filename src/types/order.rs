use crate::traits::order::ReadOrderInnerRef;
use crate::traits::order_id::{ReadOrderId, ReadOrderIdInnerRef, ReadOrderIdRef};
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

impl<OI, OIDI> ReadOrderInnerRef<OI> for Order<OI, OIDI> {
    fn read_order_inner_ref(&self) -> &OI {
        &self.inner
    }
}

impl<OI, OIDI> ReadOrderIdInnerRef<OIDI> for Order<OI, OIDI> {
    fn read_order_id_inner_ref(&self) -> &OIDI {
        self.id.read_order_id_inner_ref()
    }
}

impl<OI, OIDI> ReadOrderIdRef<OIDI> for Order<OI, OIDI> {
    fn read_order_id_ref(&self) -> &OrderId<OIDI> {
        &self.id
    }
}
