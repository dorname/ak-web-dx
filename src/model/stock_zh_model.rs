use serde::{Deserialize, Serialize};

/// 中国股票数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZnStocks {
    /// 日期
    #[serde(rename = "日期")]
    pub date: Option<String>,
    /// 股票编码
    #[serde(rename = "股票代码")]
    pub stockcode: String,
    /// 开盘价格
    #[serde(rename = "开盘")]
    pub openprice: Option<f64>,
    /// 收盘价格
    #[serde(rename = "收盘")]
    pub closeprice: Option<f64>,
    /// 最高价格
    #[serde(rename = "最高")]
    pub highprice: Option<f64>,
    /// 最低价格
    #[serde(rename = "最低")]
    pub lowprice: Option<f64>,
    /// 成交量
    #[serde(rename = "成交量")]
    pub volume: Option<f64>,
    /// 成交额
    #[serde(rename = "成交额")]
    pub turnover: Option<f64>,
    /// 振幅
    #[serde(rename = "振幅")]
    pub priceamplitude: Option<f64>,
    /// 涨跌幅
    #[serde(rename = "涨跌幅")]
    pub pricechangerate: Option<f64>,
    /// 涨跌率
    #[serde(rename = "涨跌率")]
    pub pricechangeamount: Option<f64>,
    /// 换手率
    #[serde(rename = "换手率")]
    pub turnoverrate: Option<f64>,
}

