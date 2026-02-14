use crate::types::currency_code::CurrencyCode;
use crate::types::currency_id::CurrencyId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Currency<CIDI> {
    Id(CurrencyId<CIDI>),
    Code(CurrencyCode),
}
