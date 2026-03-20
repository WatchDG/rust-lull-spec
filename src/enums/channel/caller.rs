use crate::types::trade_strategy::trade_strategy_id::TradeStrategyId;
use crate::types::market_data::market_data_id::MarketDataId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Caller<TSIDI, MDIDI> {
    TradeStrategy(TradeStrategyId<TSIDI>),
    MarketData(MarketDataId<MDIDI>),
}