use serde::{Deserialize, Serialize};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{
///         data::{Continent, Country, Currency, Data},
///         ApiClient, Entity,
///     };
///
///     #[tokio::test]
///     async fn test_list_all_data() {
///         let client = ApiClient::from_env().unwrap();
///         let data: Vec<Data> = client.list_all(Entity::Data).await.unwrap();
///         assert!(!data.is_empty());
///     }
///     #[tokio::test]
///     async fn test_list_all_currencies() {
///         let client = ApiClient::from_env().unwrap();
///         let currencies: Vec<Currency> = client.list_all(Entity::Currency).await.unwrap();
///         assert!(!currencies.is_empty());
///     }
///     #[tokio::test]
///     async fn test_list_all_countries() {
///         let client = ApiClient::from_env().unwrap();
///         let countries: Vec<Country> = client.list_all(Entity::Country).await.unwrap();
///         assert!(!countries.is_empty());
///     }
///     #[tokio::test]
///     async fn test_list_all_continents() {
///         let client = ApiClient::from_env().unwrap();
///         let continents: Vec<Continent> = client.list_all(Entity::Continent).await.unwrap();
///         assert!(!continents.is_empty());
///     }
///     #[tokio::test]
///     async fn test_retrieve_current_currency() {
///         let client = ApiClient::from_env().unwrap();
///         let current_currency: Currency = client.retrieve_current_currency().await.unwrap();
///         assert_eq!(current_currency.code, crate::models::data::CurrencyISO::RUB);
///     }
/// }
/// ```
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
    pub code: CurrencyISO,
    /// Full name of currency.
    pub name: String,
    /// Currency symbol.
    pub symbol: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub enum CurrencyISO {
    AED,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BYN,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BRL,
    BSD,
    BTC,
    BTN,
    BWP,
    BYR,
    BZD,
    CAD,
    CDF,
    CHF,
    CLP,
    CNY,
    COP,
    CRC,
    CUC,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EGP,
    ERN,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GGP,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    IMP,
    INR,
    IQD,
    IRR,
    IRT,
    ISK,
    JEP,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KPW,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LYD,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRO,
    MRU,
    MUR,
    MVR,
    MWK,
    MXN,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PRB,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SDG,
    SEK,
    SGD,
    SHP,
    SLL,
    SOS,
    SRD,
    SSP,
    STD,
    STN,
    SYP,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    #[default]
    USD,
    UYU,
    UZS,
    VEF,
    VES,
    VND,
    VUV,
    WST,
    XAF,
    XCD,
    XOF,
    XPF,
    YER,
    ZAR,
    ZMW,
}
