use crate::enums::channel::channel_market_data_request::ChannelMarketDataRequest;
use crate::types::market_data::market_data_id::MarketDataId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelMarketDataRequestPayload<MDIDI, II, DTI> {
    pub market_data_id: MarketDataId<MDIDI>,
    pub channel_market_data_request: ChannelMarketDataRequest<II, DTI>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChannelRequest<MDIDI, II, DTI> {
    ToMarketData(ChannelMarketDataRequestPayload<MDIDI, II, DTI>),
}

