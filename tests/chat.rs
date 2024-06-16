use rustyopenai::prelude::*;

#[tokio::test]
async fn test_create_chat_completion() -> Result<()> {
    // Create a client
    let client = OpenAIClient::new()?;

    // Build the request body
    let request_body = ChatRequestBody::builder(
        "gpt-3.5-turbo",
        vec![system_message!("You are a helpful assistant."), user_message!("Hello?")]
    ).build();

    // Send the request
    let chat_completion = create_chat_completion(&client, &request_body).await;

    // Unwrap the response
    assert!(chat_completion.is_ok());
    let chat_completion = chat_completion?;

    println!("{:#?}", chat_completion);

    Ok(())
}
