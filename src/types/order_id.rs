use crate::traits::order_id::{ReadOrderIdInner, WriteOrderIdInner};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId<OIDI>(OIDI);

impl<OIDI> OrderId<OIDI> {
    pub fn new(inner: OIDI) -> Self {
        Self(inner)
    }
}

impl<OIDI: Clone> ReadOrderIdInner<OIDI> for OrderId<OIDI> {
    fn read_order_id_inner(&self) -> OIDI {
        self.0.clone()
    }
}

impl<OIDI> WriteOrderIdInner<OIDI> for OrderId<OIDI> {
    fn write_order_id_inner(&mut self, inner: OIDI) -> &mut Self {
        self.0 = inner;
        self
    }
}
