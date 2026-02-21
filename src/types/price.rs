use crate::enums::currency::Currency;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Price<PI, CIDI> {
    pub price: PriceValue<PI>,
    pub currency: Currency<CIDI>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PriceValue<PVI>(PVI);
