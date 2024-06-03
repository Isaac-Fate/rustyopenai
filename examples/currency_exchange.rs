use std::collections::HashMap;
use serde::{ Deserialize, Serialize };

const ENDPOINT: &str =
    "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies/eur.json";

#[tokio::main]
async fn main() -> Result<()> {
    for (amount, from, to) in [
        (100.0, CurrencyCode::EUR, CurrencyCode::CNY),
        (100.0, CurrencyCode::CNY, CurrencyCode::EUR),
        (100.0, CurrencyCode::HKD, CurrencyCode::CNY),
        (100.0, CurrencyCode::CNY, CurrencyCode::HKD),
    ] {
        print_currency_exchange_result(amount, from, to).await?;
    }

    Ok(())
}

async fn print_currency_exchange_result(
    amount: f64,
    from: CurrencyCode,
    to: CurrencyCode
) -> Result<()> {
    // Convert the amount
    let to_amount = exchange_currency(amount, from, to).await?;

    // Print the result
    println!("{} {:?} -> {} {:?}", amount, from, to_amount, to);

    Ok(())
}

async fn exchange_currency(amount: f64, from: CurrencyCode, to: CurrencyCode) -> Result<f64> {
    // Get the exchange rates
    let exchange_rates = get_currency_exchange_rates_relative_to_euro().await?;

    // Calculate the exchange rate
    let exchange_rate = calculate_currency_exchange_rate(&exchange_rates, from, to)?;

    // Convert the amount
    Ok(amount * exchange_rate)
}

fn calculate_currency_exchange_rate(
    exchange_rates: &HashMap<CurrencyCode, f64>,
    from: CurrencyCode,
    to: CurrencyCode
) -> Result<f64> {
    // Get relative rates
    let euro_2_from_currency_rate = exchange_rates
        .get(&from)
        .ok_or(Error::MissingCurrencyCode(from))?;
    let euro_2_to_currency_rate = exchange_rates.get(&to).ok_or(Error::MissingCurrencyCode(to))?;

    // Calculate the exchange rate
    Ok(euro_2_to_currency_rate / euro_2_from_currency_rate)
}

async fn get_currency_exchange_rates_relative_to_euro() -> Result<HashMap<CurrencyCode, f64>> {
    // Create an HTTP client
    let client = reqwest::Client::new();

    // Get the response
    let response = match client.get(ENDPOINT).send().await {
        Ok(response) => response,
        Err(error) => {
            return Err(Error::GetResponse { source: error });
        }
    };

    // Parse to JSON value
    let value = match response.json::<serde_json::Value>().await {
        Ok(value) => value,
        Err(error) => {
            return Err(Error::ParseResponse { source: error });
        }
    };

    // Get the object value associated with the `eur` key
    let exchange_rates = match value.get("eur") {
        Some(exchange_rates) => exchange_rates.to_owned(),
        None => {
            return Err(Error::MissingEurField);
        }
    };

    // Parse to a `HashMap`
    let exchange_rates: HashMap<CurrencyCode, f64> = match
        serde_json::from_value(exchange_rates.to_owned())
    {
        Ok(exchange_rates) => exchange_rates,
        Err(error) => {
            return Err(Error::ParseToMap { source: error });
        }
    };

    Ok(exchange_rates)
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to get the response from API: {source}")] GetResponse {
        #[source]
        source: reqwest::Error,
    },

    #[error("failed to parse the response: {source}")] ParseResponse {
        #[source]
        source: reqwest::Error,
    },

    #[error("the `eur` filed is not found in the response")]
    MissingEurField,

    #[error("failed to parse to a `HashMap`: {source}")] ParseToMap {
        #[source]
        source: serde_json::Error,
    },

    #[error("currency code {0:?} is not found")] MissingCurrencyCode(CurrencyCode),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CurrencyCode {
    /// Euro.
    EUR,

    /// Renminbi.
    CNY,

    /// Hong Kong dollar.
    HKD,

    /// United State dollar.
    USD,

    // There is a symbol 🐕 from the API response,
    // which I am not aware of.
    #[serde(other)]
    Unknown,
}
