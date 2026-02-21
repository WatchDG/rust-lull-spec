use crate::types::market_data::market_data_id::MarketDataId;
use crate::types::market_data::market_data_name::MarketDataName;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarketData<MDIDI, MDI> {
    pub id: MarketDataId<MDIDI>,
    pub name: MarketDataName,
    pub market_data: MDI
}