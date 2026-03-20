use crate::types::market_data::market_data_id::MarketDataId;
use crate::types::trade_strategy::trade_strategy_id::TradeStrategyId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Callee<TSIDI, MDIDI> {
    TradeStrategy(TradeStrategyId<TSIDI>),
    MarketData(MarketDataId<MDIDI>),
}