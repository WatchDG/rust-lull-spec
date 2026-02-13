use crate::traits::order::{ReadOrder, WriteOrder};
use crate::traits::order_id::ReadOrderId;
use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<O, OIDI>(O, PhantomData<OIDI>);

impl<O, OIDI> Order<O, OIDI> {
    pub fn new(inner: O) -> Self {
        Self(inner, PhantomData)
    }
}

impl<O: Clone + ReadOrderId<OIDI>, OIDI: Clone> ReadOrder<O, OIDI> for Order<O, OIDI> {
    fn read_order(&self) -> Order<O, OIDI> {
        self.clone()
    }
}

impl<O, OIDI> WriteOrder<O, OIDI> for Order<O, OIDI> {
    fn write_order(&mut self, order: Order<O, OIDI>) -> &mut Self {
        self.0 = order.0;
        self
    }
}
