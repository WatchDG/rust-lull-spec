use crate::traits::order::{ReadOrder, WriteOrder};
use crate::traits::order_id::{ReadOrderId, WriteOrderId};
use crate::types::order_id::OrderId;
use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<O, OID>(O, PhantomData<OID>);

impl<O, OID> Order<O, OID> {
    pub fn new(inner: O) -> Self {
        Self(inner, PhantomData)
    }
}

impl<O: ReadOrderId<OID>, OID> ReadOrderId<OID> for Order<O, OID> {
    fn read_order_id(&self) -> OrderId<OID> {
        self.0.read_order_id()
    }
}

impl<O: WriteOrderId<OID>, OID> WriteOrderId<OID> for Order<O, OID> {
    fn write_order_id(&mut self, id: OrderId<OID>) -> &mut Self {
        self.0.write_order_id(id);
        self
    }
}

impl<O: Clone + ReadOrderId<OID>, OID: Clone> ReadOrder<O, OID> for Order<O, OID> {
    fn read_order(&self) -> Order<O, OID> {
        self.clone()
    }
}

impl<O, OID> WriteOrder<O, OID> for Order<O, OID> {
    fn write_order(&mut self, order: Order<O, OID>) -> &mut Self {
        self.0 = order.0;
        self
    }
}
