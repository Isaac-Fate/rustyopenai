use rustyopenai::*;

#[test]
fn test_openai_client() {
    // Initialize a logger

    // Create a client
    let client = OpenAIClient::builder().build();
    assert!(client.is_ok());
    let client = client.unwrap();
}

#[tokio::test]
async fn test_list_models() {
    // Create a client
    let client = OpenAIClient::builder().build().unwrap();

    // Get the list of models
    let response = list_models(&client).await;
    assert!(response.is_ok());
    let response = response.unwrap();

    println!("{:#?}", response);
}
