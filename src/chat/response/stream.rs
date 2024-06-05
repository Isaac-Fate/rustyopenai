use std::{ pin::Pin, task::{ Context, Poll } };
use bytes::{ Bytes, BytesMut };
use futures::{ Stream, StreamExt };
use regex::Regex;
use lazy_static::lazy_static;
use crate::{ Result, Error };
use super::ChatCompletionChunk;

/// The content of the termination data chunk,
/// indicating the end of the stream.
const TERMINATION_DATA_CHUNK: &str = "[DONE]";

lazy_static! {
    static ref DATA_CHUNK_RE: Regex = Regex::new(r#"^data: (.*)\n\n"#).unwrap();
}

pub struct ChatCompletionStream {
    stream: Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>>>>,
    buffer: BytesMut,
}

impl ChatCompletionStream {
    pub fn new<S: 'static + Stream<Item = reqwest::Result<Bytes>>>(stream: S) -> Self {
        Self { stream: Box::pin(stream), buffer: BytesMut::new() }
    }

    fn extract_first_chunk(
        &mut self,
        done_receiving: bool
    ) -> Poll<Option<Result<ChatCompletionChunk>>> {
        // Convert buffer to string
        let buffer = self.buffer.clone();
        let buffer_str = String::from_utf8_lossy(&buffer);

        println!("buffer_str: {}", buffer_str);

        // Match a data chunk from the start of the buffer
        if let Some(captures) = DATA_CHUNK_RE.captures(&buffer_str) {
            // Get the first match
            let data_chunk_match = captures.get(1).expect("failed to get data chunk match");

            // Get the matched data chunk as str
            let data_chunk = data_chunk_match.as_str();

            // Check if the data chunk is the termination data chunk
            if data_chunk == TERMINATION_DATA_CHUNK {
                return Poll::Ready(None);
            }

            // Update the start position of the buffer
            let buffer_start = data_chunk_match.end() + 2;

            // Update buffer by removing the matched data chunk
            self.buffer = self.buffer.split_off(buffer_start);

            // Parse to a chat completion chunk
            let chat_completion_chunk: ChatCompletionChunk = serde_json
                ::from_str(data_chunk)
                .unwrap();

            Poll::Ready(Some(Ok(chat_completion_chunk)))
        } else if done_receiving {
            // Poll::Ready(Some(Err(Error::ApiKeyNotSet)))
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}

impl Stream for ChatCompletionStream {
    type Item = Result<ChatCompletionChunk>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        println!("!!! poll_next is called");

        match self.stream.poll_next_unpin(cx) {
            Poll::Ready(Some(Ok(bytes))) => {
                // Add newly received bytes to the buffer
                self.buffer.extend(&bytes);

                println!("bytes: {:?}", bytes);

                self.extract_first_chunk(false)
            }
            Poll::Ready(Some(Err(error))) => {
                // Poll::Ready(Some(Err(Error::ApiKeyNotSet)))
                eprintln!("error: {}", error);
                Poll::Ready(None)
            }
            Poll::Ready(None) => {
                // The buffer is empty
                if self.buffer.is_empty() {
                    Poll::Ready(None)
                } else {
                    // The buffer is not empty yet,
                    // which means there are still bytes to be processed
                    self.extract_first_chunk(true)
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
