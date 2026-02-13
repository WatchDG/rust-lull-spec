use crate::traits::order_id::{ReadOrderIdInnerRef, WriteOrderIdInner};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId<OIDI>(OIDI);

impl<OIDI> OrderId<OIDI> {
    pub fn new(inner: OIDI) -> Self {
        Self(inner)
    }
}

impl<OIDI> ReadOrderIdInnerRef<OIDI> for OrderId<OIDI> {
    fn read_order_id_inner_ref(&self) -> &OIDI {
        &self.0
    }
}

impl<OIDI> WriteOrderIdInner<OIDI> for OrderId<OIDI> {
    fn write_order_id_inner(&mut self, inner: OIDI) -> &mut Self {
        self.0 = inner;
        self
    }
}
