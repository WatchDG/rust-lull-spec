use crate::types::order::Order;

// pub trait ReadOrderInnerRef<OI> {
//     fn read_order_inner_ref(&self) -> &OI;
// }

// pub trait ReadOrderInner<OI> {
//     fn read_order_inner(&self) -> OI;
// }

// impl<OI: Clone, T> ReadOrderInner<OI> for T
// where
//     T: ReadOrderInnerRef<OI>,
// {
//     fn read_order_inner(&self) -> OI {
//         self.read_order_inner_ref().clone()
//     }
// }

pub trait ReadOrderRef<OI, OIDI, II> {
    fn read_order_ref(&self) -> &Order<OI, OIDI, II>;
}

// impl<OI, OIDI> ReadOrderRef<OI, OIDI> for Order<OI, OIDI> {
//     fn read_order_ref(&self) -> &Order<OI, OIDI> {
//         self
//     }
// }

// pub trait ReadOrder<OI, OIDI> {
//     fn read_order(&self) -> Order<OI, OIDI>;
// }

// impl<T, OI: Clone, OIDI: Clone> ReadOrder<OI, OIDI> for T
// where
//     T: ReadOrderRef<OI, OIDI>,
// {
//     fn read_order(&self) -> Order<OI, OIDI> {
//         self.read_order_ref().clone()
//     }
// }
