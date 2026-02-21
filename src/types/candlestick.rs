use crate::types::price::PriceValue;
use crate::types::quantity::Quantity;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Candlestick<PVI, QI> {
    pub open: PriceValue<PVI>,
    pub high: PriceValue<PVI>,
    pub low: PriceValue<PVI>,
    pub close: PriceValue<PVI>,
    pub volume: Quantity<QI>,
}
