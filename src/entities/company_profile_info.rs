use std::sync::Arc;

use crate::API_ENDPOINT;
use crate::TOKEN;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CompanyProfile {
    pub country: String,
    pub currency: String,
    pub exchange: String,
    pub ipo: String,
    pub marketCapitalization: f64,
    pub name: String,
    pub phone: String,
    pub shareOutstanding: f64,
    pub ticker: String,
    pub weburl: String,
    pub logo: String,
    pub finnhubIndustry: String,
}

impl Into<String> for CompanyProfile {
    fn into(self) -> String {
        format!(
            "Country: {}\nCurrency: {}\nExchange: {}\nIPO: {}\nMarket capitalization: {}\nName: {}\nPhone: {}\nShare outstanding: {}\nTicker: {}\nWebURL: {}\nLogo: {}\nFinnhub industry: {}",
            self.country, self.currency, self.exchange, self.ipo, self.marketCapitalization, self.name, self.phone, self.shareOutstanding, self.ticker, self.weburl, self.logo, self.finnhubIndustry
        )
    }
}

impl CompanyProfile {
    pub async fn get(client: Arc<reqwest::blocking::Client>, symbol: &str, isin: Option<&str>, cusip: Option<&str>) -> Result<Self, ()> {
        client.get(&format!("{}{}", API_ENDPOINT, "/stock/profile2")).query(&[("symbol", Some(symbol)), ("isin", isin), ("cusip", cusip), ("token", Some(TOKEN))]).send()
            .map_err(|_| ())?
            .json::<CompanyProfile>().map_err(|_| ())
    }
}