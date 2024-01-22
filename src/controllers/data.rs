#[cfg(test)]
mod tests {
    use crate::{
        data::{Continent, Country, Currency, Data},
        ApiClient, Entity,
    };

    #[tokio::test]
    async fn test_list_all_data() {
        let client = ApiClient::from_env().unwrap();
        let data: Vec<Data> = client.list_all(Entity::Data).await.unwrap();
        assert!(!data.is_empty());
    }
    #[tokio::test]
    async fn test_list_all_currencies() {
        let client = ApiClient::from_env().unwrap();
        let currencies: Vec<Currency> = client.list_all(Entity::Currency).await.unwrap();
        assert!(!currencies.is_empty());
    }
    #[tokio::test]
    async fn test_list_all_countries() {
        let client = ApiClient::from_env().unwrap();
        let countries: Vec<Country> = client.list_all(Entity::Country).await.unwrap();
        assert!(!countries.is_empty());
    }
    #[tokio::test]
    async fn test_list_all_continents() {
        let client = ApiClient::from_env().unwrap();
        let continents: Vec<Continent> = client.list_all(Entity::Continent).await.unwrap();
        assert!(!continents.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_current_currency() {
        let client = ApiClient::from_env().unwrap();
        let current_currency: Currency = client.retrieve_current_currency().await.unwrap();
        assert_eq!(current_currency.code, crate::models::data::CurrencyISO::RUB);
    }
}
