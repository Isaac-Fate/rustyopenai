use std::collections::HashMap;
use serde::{ Deserialize, Serialize };
use rustyopenai::prelude::*;

const ENDPOINT: &str =
    "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies/eur.json";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    for (amount, source, target) in [
        (100.0, CurrencyCode::EUR, CurrencyCode::CNY),
        (100.0, CurrencyCode::CNY, CurrencyCode::EUR),
        (100.0, CurrencyCode::HKD, CurrencyCode::CNY),
        (100.0, CurrencyCode::CNY, CurrencyCode::HKD),
    ] {
        print_currency_exchange_result(amount, source, target).await?;
    }

    // Create an OpenAI client
    let client = OpenAIClient::new()?;

    // Build the request body
    let request_body = ChatRequestBody::builder(
        "gpt-3.5-turbo",
        vec![
            system_message!("You are a helpful assistant."),
            user_message!("What is 100 Hong Kong dollars in Chinese Yuan?")
        ]
    )
        .tools(
            vec![
                function!(
                    "exchange_currency",
                    description = "Converts a currency into another currency.",
                    parameters = function_parameters! {
                    "amount": json!({"type": "number", "description": "The amount to convert."}),
                    "source": json!({"type": "string", "enum": ["eur", "cny", "hkd", "usd"], "description": "The currency to convert from."}),
                    "target": json!({"type": "string", "enum": ["eur", "cny", "hkd", "usd"], "description": "The currency to convert to."})
                }
                )
            ]
        )
        .build();

    // Send the request
    let completion = create_chat_completion(&client, &request_body).await?;

    // Print the response
    println!("{:#?}", completion);

    // Extract the called function
    let function = completion.choices
        .first()
        .unwrap()
        .message.tool_calls.clone()
        .unwrap()
        .first()
        .unwrap()
        .clone().function;

    assert_eq!(function.name, "exchange_currency");

    // Get the arguments
    let arguments = function.arguments;
    let amount = arguments.get("amount").unwrap().as_f64().unwrap();
    let source: CurrencyCode = serde_json::from_value(arguments.get("source").unwrap().to_owned())?;
    let target: CurrencyCode = serde_json::from_value(arguments.get("target").unwrap().to_owned())?;

    // Call the function
    let target_amount = exchange_currency(amount, source, target).await?;

    // Print the result
    println!("question: What is 100 Hong Kong dollars in Chinese Yuan?");
    println!("answer: {}", target_amount);

    Ok(())
}

async fn print_currency_exchange_result(
    amount: f64,
    source: CurrencyCode,
    target: CurrencyCode
) -> Result<()> {
    // Convert the amount
    let target_amount = exchange_currency(amount, source, target).await?;

    // Print the result
    println!("{} {:?} -> {} {:?}", amount, source, target_amount, target);

    Ok(())
}

async fn exchange_currency(amount: f64, source: CurrencyCode, target: CurrencyCode) -> Result<f64> {
    // Get the exchange rates
    let exchange_rates = get_currency_exchange_rates_relative_target_euro().await?;

    // Calculate the exchange rate
    let exchange_rate = calculate_currency_exchange_rate(&exchange_rates, source, target)?;

    // Convert the amount
    Ok(amount * exchange_rate)
}

fn calculate_currency_exchange_rate(
    exchange_rates: &HashMap<CurrencyCode, f64>,
    source: CurrencyCode,
    target: CurrencyCode
) -> Result<f64> {
    // Get relative rates
    let euro_to_source_currency_rate = exchange_rates
        .get(&source)
        .ok_or(Error::MissingCurrencyCode(source))?;
    let euro_to_target_currency_rate = exchange_rates
        .get(&target)
        .ok_or(Error::MissingCurrencyCode(target))?;

    // Calculate the exchange rate
    Ok(euro_to_target_currency_rate / euro_to_source_currency_rate)
}

async fn get_currency_exchange_rates_relative_target_euro() -> Result<HashMap<CurrencyCode, f64>> {
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
    #[error("failed to get the response source API: {source}")] GetResponse {
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

    // There is a symbol 🐕 source the API response,
    // which I am not aware of.
    #[serde(other)]
    Unknown,
}
