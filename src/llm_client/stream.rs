use crate::core::StreamChunk;
use crate::core::interfaces::LLMStreamError;

pub async fn parse_sse_stream(_response: reqwest::Response) -> impl futures::Stream<Item = Result<StreamChunk, LLMStreamError>> {
    futures::stream::empty()
}

pub fn aggregate_stream_chunks(_chunks: Vec<StreamChunk>) -> AggregatedResult {
    AggregatedResult::default()
}

#[derive(Default)]
pub struct AggregatedResult {
    pub content: String,
    pub tool_calls: Vec<crate::core::LLMToolCall>,
    pub finish_reason: Option<String>,
}