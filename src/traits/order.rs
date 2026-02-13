use crate::types::order::Order;

pub trait ReadOrder<OI, OIDI> {
    fn read_order(&self) -> Order<OI, OIDI>;
}
