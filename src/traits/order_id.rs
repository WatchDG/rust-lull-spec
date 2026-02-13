use crate::types::order_id::OrderId;

pub trait ReadOrderIdInnerRef<OIDI> {
    fn read_order_id_inner_ref(&self) -> &OIDI;
}

pub trait ReadOrderIdInner<OIDI> {
    fn read_order_id_inner(&self) -> OIDI;
}

impl<OIDI: Clone, T> ReadOrderIdInner<OIDI> for T
where
    T: ReadOrderIdInnerRef<OIDI>,
{
    fn read_order_id_inner(&self) -> OIDI {
        self.read_order_id_inner_ref().clone()
    }
}

pub trait ReadOrderId<OID> {
    fn read_order_id(&self) -> OrderId<OID>;
}

impl<OIDI, T> ReadOrderId<OIDI> for T
where
    T: ReadOrderIdInner<OIDI>,
{
    fn read_order_id(&self) -> OrderId<OIDI> {
        OrderId::new(self.read_order_id_inner())
    }
}

// pub trait WriteOrderIdInner<OIDI> {
//     fn write_order_id_inner(&mut self, inner: OIDI) -> &mut Self;
// }

// pub trait WriteOrderId<OID> {
//     fn write_order_id(&mut self, id: &impl ReadOrderIdInner<OID>) -> &mut Self;
// }

// impl<OIDI: Clone, T> WriteOrderId<OIDI> for T
// where
//     T: WriteOrderIdInner<OIDI>,
// {
//     fn write_order_id(&mut self, id: &impl ReadOrderIdInner<OIDI>) -> &mut Self {
//         self.write_order_id_inner(id.read_order_id_inner())
//     }
// }
