use crate::enums::trade_strategy_error::TradeStrategyError;
use crate::types::trade_strategy::TradeStrategy;
use crate::types::trade_strategy_id::TradeStrategyId;
use crate::types::trade_strategy_settings::TradeStrategySettings;
use core::future::Future;

pub trait BuildTradeStrategy<TSIDI, TSSI, TSI> {
    fn build_trade_strategy(
        &self,
        id: TradeStrategyId<TSIDI>,
        settings: TradeStrategySettings<TSSI>,
        strategy: TSI,
    ) -> impl Future<Output = Result<TradeStrategy<TSIDI, TSSI, TSI>, TradeStrategyError>> + Send;
}
