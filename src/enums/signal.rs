#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignalError {
    Unexpected,
}
