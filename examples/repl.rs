use std::io::Write;

use rustyopenai::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create the editor
    let mut editor = rustyline::DefaultEditor::new().unwrap();

    // Create an OpenAI client
    let client = OpenAIClient::new()?;

    loop {
        let line = editor.readline(">> ");
        match line {
            Ok(line) => {
                match line.as_str() {
                    // Quit the REPL
                    r"\quit" | r"\q" => {
                        break;
                    }

                    line => {
                        // Add the line to the history
                        editor.add_history_entry(line).unwrap();

                        // Query the OpenAI API and print the response
                        print_response_content(&client, line).await?;

                        // Print a new line to prevent that the next prompt is on the same line
                        println!();
                        std::io::stdout().flush().unwrap();
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}

async fn print_response_content<S: AsRef<str>>(client: &OpenAIClient, prompt: S) -> Result<()> {
    // Build the request body
    let request_body = ChatRequestBody::builder(
        "gpt-3.5-turbo",
        vec![system_message!("You are a helpful assistant."), user_message!(prompt.as_ref())]
    ).build();

    // Get the chat completion stream
    let mut stream = create_chat_completion_stream(client, &request_body, false).await?;

    // Print the content of each chunk
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(chunk) => {
                print!(
                    "{}",
                    chunk.choices.first().unwrap().delta.content.clone().unwrap_or("".to_string())
                );
                std::io::stdout().flush().unwrap();
            }
            Err(error) => {
                eprintln!("{}", error);
                break;
            }
        }
    }

    Ok(())
}
