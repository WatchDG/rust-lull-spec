#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradeSignal {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradeSignalError {
    Unexpected,
}
