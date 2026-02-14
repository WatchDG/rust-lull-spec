use crate::types::trade_strategy_id::TradeStrategyId;
use crate::types::trade_strategy_settings::TradeStrategySettings;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TradeStrategy<TSIDI, TSSI, TSI> {
    pub id: TradeStrategyId<TSIDI>,
    pub settings: TradeStrategySettings<TSSI>,
    pub strategy: TSI,
}
