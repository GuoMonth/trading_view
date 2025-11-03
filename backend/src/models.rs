use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// K线数据模型 (OHLC)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OHLCData {
    /// 时间戳
    pub timestamp: NaiveDateTime,
    /// 开盘价
    pub open: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 收盘价
    pub close: f64,
    /// 成交量
    pub volume: Option<f64>,
}

/// 移动平均线指标
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MAIndicator {
    /// 时间戳
    pub timestamp: NaiveDateTime,
    /// 短期移动平均线 (如5日均线)
    pub short: Option<f64>,
    /// 中期移动平均线 (如20日均线)
    pub medium: Option<f64>,
    /// 长期移动平均线 (如60日均线)
    pub long: Option<f64>,
}

/// RSI指标
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RSIIndicator {
    /// 时间戳
    pub timestamp: NaiveDateTime,
    /// RSI值 (通常14周期)
    pub value: Option<f64>,
}

/// MACD指标
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MACDIndicator {
    /// 时间戳
    pub timestamp: NaiveDateTime,
    /// MACD线
    pub macd: Option<f64>,
    /// 信号线
    pub signal: Option<f64>,
    /// 柱状图
    pub histogram: Option<f64>,
}

/// Bollinger Bands指标
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BollingerBands {
    /// 时间戳
    pub timestamp: NaiveDateTime,
    /// 中轨 (通常20日均线)
    pub middle: Option<f64>,
    /// 上轨 (中轨 + 2倍标准差)
    pub upper: Option<f64>,
    /// 下轨 (中轨 - 2倍标准差)
    pub lower: Option<f64>,
}

/// 指标数据集合
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndicatorData {
    /// 移动平均线
    pub ma: Vec<MAIndicator>,
    /// RSI
    pub rsi: Vec<RSIIndicator>,
    /// MACD
    pub macd: Vec<MACDIndicator>,
    /// Bollinger Bands
    pub bollinger_bands: Vec<BollingerBands>,
}

/// 交易信号
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradingSignal {
    /// 时间戳
    pub timestamp: NaiveDateTime,
    /// 信号类型 (买入/卖出/持有)
    pub signal_type: SignalType,
    /// 信号来源 (如MA交叉, RSI超买等)
    pub source: String,
    /// 价格
    pub price: f64,
}

/// 信号类型枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SignalType {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "hold")]
    Hold,
}

/// 回测参数
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BacktestParams {
    /// 交易品种
    pub symbol: String,
    /// 时间周期
    pub timeframe: String,
    /// 起始时间
    pub start_date: NaiveDateTime,
    /// 结束时间
    pub end_date: NaiveDateTime,
    /// 初始资金
    pub initial_capital: f64,
    /// 策略参数
    pub strategy_params: serde_json::Value,
}

/// 回测结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BacktestResult {
    /// 回测ID
    pub id: String,
    /// 交易品种
    pub symbol: String,
    /// 时间周期
    pub timeframe: String,
    /// 起始时间
    pub start_date: NaiveDateTime,
    /// 结束时间
    pub end_date: NaiveDateTime,
    /// 初始资金
    pub initial_capital: f64,
    /// 最终资金
    pub final_capital: f64,
    /// 总收益率
    pub total_return: f64,
    /// 年化收益率
    pub annual_return: f64,
    /// 最大回撤
    pub max_drawdown: f64,
    /// 夏普比率
    pub sharpe_ratio: f64,
    /// 交易次数
    pub trade_count: usize,
    /// 盈利交易次数
    pub winning_trades: usize,
    /// 亏损交易次数
    pub losing_trades: usize,
    /// 胜率
    pub win_rate: f64,
    /// 平均盈利
    pub average_win: f64,
    /// 平均亏损
    pub average_loss: f64,
    /// 盈亏比
    pub profit_factor: f64,
    /// 交易记录
    pub trades: Vec<TradeRecord>,
}

/// 交易记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeRecord {
    /// 交易ID
    pub id: String,
    /// 交易时间
    pub timestamp: NaiveDateTime,
    /// 交易类型 (买入/卖出)
    pub trade_type: TradeType,
    /// 价格
    pub price: f64,
    /// 数量
    pub quantity: f64,
    /// 金额
    pub amount: f64,
    /// 手续费
    pub fee: f64,
    /// 剩余资金
    pub remaining_capital: f64,
    /// 持仓数量
    pub position: f64,
}

/// 交易类型枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TradeType {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}