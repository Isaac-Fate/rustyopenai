use std::time::Duration;
use futures::StreamExt;
use rustyopenai::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client
    // let client = OpenAIClient::builder().timeout(Duration::from_millis(5000)).build()?;
    let client = OpenAIClient::new()?;

    // Build the request body
    let request_body = ChatRequestBody::builder(
        "gpt-3.5-turbo",
        vec![
            system_message!("You are a helpful assistant."),
            // user_message!("What is the tallest building in Hong Kong?"),
            user_message!("Explain rust stream in details")
        ]
    ).build();

    // Get the stream
    let mut chat_completion_stream = create_chat_completion_stream(
        &client,
        &request_body,
        true
    ).await?;

    while let Some(chunk) = chat_completion_stream.next().await {
        // println!("{:#?}", chunk);
        let choices = chunk.unwrap().choices;

        let choice = match choices.first() {
            Some(choice) => choice,
            None => {
                continue;
            }
        };

        print!("{}", choice.delta.content.clone().unwrap_or("".to_string()));
    }

    Ok(())
}
