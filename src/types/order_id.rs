use crate::traits::order_id::{ReadOrderId, WriteOrderId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId<OID>(OID);

impl<OID> OrderId<OID> {
    pub fn new(inner: OID) -> Self {
        Self(inner)
    }
}

impl<OID: Clone> ReadOrderId<OID> for OrderId<OID> {
    fn read_order_id(&self) -> OrderId<OID> {
        self.clone()
    }
}

impl<OID: Clone> WriteOrderId<OID> for OrderId<OID> {
    fn write_order_id(&mut self, id: OrderId<OID>) -> &mut Self {
        self.0 = id.0;
        self
    }
}
