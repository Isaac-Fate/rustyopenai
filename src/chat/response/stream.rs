use std::{ pin::Pin, task::{ Context, Poll } };
use bytes::{ Bytes, BytesMut };
use futures::{ Stream, StreamExt };
use regex::Regex;
use lazy_static::lazy_static;
use crate::{ Result, Error, ChatApiError };
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

    fn extract_first_chunk(&mut self) -> Option<Result<ExtractedChunk>> {
        // Convert buffer to string
        let buffer = self.buffer.clone();
        let buffer_str = String::from_utf8_lossy(&buffer);

        // Match a data chunk from the start of the buffer
        if let Some(captures) = DATA_CHUNK_RE.captures(&buffer_str) {
            // Get the first match
            let data_chunk_match = match captures.get(1) {
                Some(data_chunk_match) => data_chunk_match,

                // Actually, this is not reachable since
                // there indeed exist some captures
                None => {
                    return Some(Err(Error::ChatApi(ChatApiError::GetFirstMatchingDataChunk)));
                }
            };

            // Get the matched data chunk as str
            let data_chunk = data_chunk_match.as_str();

            // Check if the data chunk is the termination data chunk
            if data_chunk == TERMINATION_DATA_CHUNK {
                return Some(Ok(ExtractedChunk::Termination));
            }

            // Update the start position of the buffer
            let buffer_start = data_chunk_match.end() + 2;

            // Update buffer by removing the matched data chunk
            self.buffer = self.buffer.split_off(buffer_start);

            // Parse to a chat completion chunk
            let chat_completion_chunk: ChatCompletionChunk = match serde_json::from_str(data_chunk) {
                Ok(chat_completion_chunk) => chat_completion_chunk,
                Err(error) => {
                    return Some(
                        Err(
                            Error::ChatApi(ChatApiError::ParseToChatCompletionChunk {
                                source: error,
                            })
                        )
                    );
                }
            };

            Some(Ok(ExtractedChunk::Normal(chat_completion_chunk)))
        } else {
            None
        }
    }
}

enum ExtractedChunk {
    Normal(ChatCompletionChunk),
    Termination,
}

impl Stream for ChatCompletionStream {
    type Item = Result<ChatCompletionChunk>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match self.stream.poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(bytes))) => {
                    // Add newly received bytes to the buffer
                    self.buffer.extend(&bytes);

                    // Extract the first chunk from the buffer
                    match self.extract_first_chunk() {
                        Some(Ok(ExtractedChunk::Normal(chunk))) => {
                            return Poll::Ready(Some(Ok(chunk)));
                        }
                        Some(Ok(ExtractedChunk::Termination)) => {
                            return Poll::Ready(None);
                        }
                        Some(Err(error)) => {
                            return Poll::Ready(Some(Err(error)));
                        }
                        None => {
                            // Continue polling the bytes stream, and
                            // extract the first chunk from the extended buffer
                            continue;
                        }
                    }
                }
                Poll::Ready(Some(Err(error))) => {
                    return Poll::Ready(
                        Some(
                            Err(
                                Error::ChatApi(ChatApiError::ReceiveStreamedBytes { source: error })
                            )
                        )
                    );
                }
                Poll::Ready(None) => {
                    // The buffer is empty
                    if self.buffer.is_empty() {
                        return Poll::Ready(None);
                    } else {
                        // The buffer is not empty yet,
                        // which means there are still bytes to be processed
                        match self.extract_first_chunk() {
                            Some(Ok(ExtractedChunk::Normal(chunk))) => {
                                return Poll::Ready(Some(Ok(chunk)));
                            }
                            Some(Ok(ExtractedChunk::Termination)) => {
                                return Poll::Ready(None);
                            }
                            Some(Err(error)) => {
                                return Poll::Ready(Some(Err(error)));
                            }
                            None => {
                                // Continue polling the bytes stream, and
                                // extract the first chunk the buffer
                                continue;
                            }
                        }
                    }
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}
