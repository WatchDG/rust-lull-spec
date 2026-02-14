use crate::types::trade_strategy::trade_strategy_id::TradeStrategyId;
use crate::types::trade_strategy::trade_strategy_name::TradeStrategyName;
use crate::types::trade_strategy::trade_strategy_settings::TradeStrategySettings;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TradeStrategy<TSIDI, TSSI, TSI> {
    pub id: TradeStrategyId<TSIDI>,
    pub name: TradeStrategyName,
    pub settings: TradeStrategySettings<TSSI>,
    pub strategy: TSI,
}
