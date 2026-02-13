use crate::types::new_order::NewOrder;
use crate::types::persistent_order::PersistentOrder;
use core::future::Future;

pub trait SaveNewOrder<OIDI, E> {
    fn save_new_order(
        &self,
        new_order: NewOrder<OIDI>,
    ) -> impl Future<Output = Result<PersistentOrder<OIDI>, E>> + Send;
}
