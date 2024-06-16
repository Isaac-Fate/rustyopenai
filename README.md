# Rusty OpenAI

A Rust crate for interfacing with the OpenAI API.

## Chat

### Complete Response

```rust
use rustyopenai::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client
    let client = OpenAIClient::new()?;

    // Build the request body
    let request_body = ChatRequestBody::builder(
        "gpt-3.5-turbo",
        vec![system_message!("You are a helpful assistant."), user_message!("Hello?")]
    ).build();

    // Send the request
    let chat_completion = create_chat_completion(&client, &request_body).await?;

    // Print the struct
    println!("{:#?}", chat_completion);

    Ok(())
}
```

```
ChatCompletion {
    id: "chatcmpl-9ZLDewTAvpKeglNziiTbd6KJorDIt",
    created: 1718210074,
    model: "gpt-3.5-turbo-0125",
    system_fingerprint: None,
    choices: [
        ChatCompletionChoice {
            finish_reason: Stop,
            index: 0,
            message: ChatCompletionMessage {
                content: Some(
                    "Hello! How can I assist you today?",
                ),
                tool_calls: None,
            },
        },
    ],
    usage: ChatCompletionTokenUsage {
        completion_tokens: 9,
        prompt_tokens: 19,
        total_tokens: 28,
    },
}
```

### Streaming

## Embeddings

## Models

### Listing Models

### Retrieving a Model
