use serde::{Deserialize, Serialize};
use crate::entity::ohlc_data::Model as OhlcDataModel;

/// OHLCR数据按日期范围查询请求参数
#[derive(Deserialize, Debug)]
pub struct OhlcDateRangeRequest {
    pub start: String,
    pub end: String,
}

/// OHLCR数据响应结构
#[derive(Serialize, Debug)]
pub struct OhlcResponse {
    pub data: Vec<OhlcDataModel>,
}