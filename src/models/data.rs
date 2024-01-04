use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    /// Data resource ID.
    pub slug: String,
    /// Data resource description.
    pub description: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Continent {
    /// 2 character continent code
    pub code: String,
    /// Full name of continent.
    pub name: String,
    /// List of countries on this continent.
    pub countries: Vec<Country>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    /// ISO3166 alpha-2 country code    
    pub code: String,
    /// Default ISO4127 alpha-3 currency code for the country.
    pub currency_code: Option<String>,
    /// Currency symbol position for this country.
    pub currency_pos: Option<String>,
    /// Decimal separator for displayed prices for this country.
    pub decimal_sep: Option<String>,
    /// The unit lengths are defined in for this country.
    pub dimension_unit: Option<String>,
    /// Full name of country.
    pub name: String,
    /// Number of decimal points shown in displayed prices for this country.
    pub num_decimals: Option<i32>,
    /// List of states in this country. See Continents - Countries - States properties    
    pub states: Vec<State>,
    /// Thousands separator for displayed prices in this country.
    pub thousand_sep: Option<String>,
    /// The unit weights are defined in for this country.
    pub weight_unit: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    /// State code.
    pub code: serde_json::Value,
    /// Full name of state.
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// ISO4217 currency code.
    pub code: String,
    /// Full name of currency.
    pub name: String,
    /// Currency symbol.
    pub symbol: String,
}
