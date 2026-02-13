use crate::traits::order::{ReadOrder, WriteOrder};
use crate::traits::order_id::ReadOrderId;
use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<O, OID>(O, PhantomData<OID>);

impl<O, OID> Order<O, OID> {
    pub fn new(inner: O) -> Self {
        Self(inner, PhantomData)
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
