use crate::types::order::Order;

pub trait ReadOrder<O, OID> {
    fn read_order(&self) -> Order<O, OID>;
}

pub trait WriteOrder<O, OID> {
    fn write_order(&mut self, order: Order<O, OID>) -> &mut Self;
}
