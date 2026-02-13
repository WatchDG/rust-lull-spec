use crate::traits::order_id::{ReadOrderIdInner, WriteOrderIdInner};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId<OID>(OID);

impl<OID> OrderId<OID> {
    pub fn new(inner: OID) -> Self {
        Self(inner)
    }
}

impl<OID: Clone> ReadOrderIdInner<OID> for OrderId<OID> {
    fn read_order_id_inner(&self) -> OID {
        self.0.clone()
    }
}

impl<OID> WriteOrderIdInner<OID> for OrderId<OID> {
    fn write_order_id_inner(&mut self, inner: OID) -> &mut Self {
        self.0 = inner;
        self
    }
}
