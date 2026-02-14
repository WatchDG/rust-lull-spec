use crate::enums::order_record_error::OrderRecordError;
use crate::traits::order_id::ReadOrderIdRef;
use crate::types::new_order::NewOrder;
use crate::types::order_record::OrderRecord;
use core::future::Future;

pub trait SaveOrder<OIDI, II> {
    fn save_order(
        &self,
        new_order: NewOrder<OIDI, II>,
    ) -> impl Future<Output = Result<OrderRecord<OIDI, II>, OrderRecordError>> + Send;
}

pub trait LoadOrder<OIDI, II> {
    fn load_order(
        &self,
        id: impl ReadOrderIdRef<OIDI>,
    ) -> impl Future<Output = Result<OrderRecord<OIDI, II>, OrderRecordError>> + Send;
}
