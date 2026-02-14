use crate::enums::currency::Currency;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Price<PI, CIDI> {
    pub price: PI,
    pub currency: Currency<CIDI>,
}
