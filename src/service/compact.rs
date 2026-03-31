use crate::core::types::llm::{LLMMessage, LLMMessageRole};
use crate::core::types::message::{CreateMessageData, Message, MessageRole};
use crate::prompts::{get_compact_system_prompt, get_compact_user_prompt};

pub struct CompactResult {
    pub compacted: usize,
    pub summary_id: String,
}

pub fn get_messages_to_compact(messages: &[Message]) -> Vec<Message> {
    let summary_index = messages
        .iter()
        .rposition(|m| m.role == MessageRole::Summary);

    if let Some(idx) = summary_index {
        messages[..idx].to_vec()
    } else {
        messages
            .iter()
            .filter(|m| m.role != MessageRole::Summary)
            .cloned()
            .collect()
    }
}

pub fn convert_messages_to_llm_format(messages: &[Message]) -> Vec<LLMMessage> {
    let mut result: Vec<LLMMessage> = Vec::new();

    for msg in messages {
        if msg.role == MessageRole::Error {
            continue;
        }

        if msg.role == MessageRole::Thinking {
            continue;
        }

        if msg.role == MessageRole::Summary {
            result.push(LLMMessage {
                role: LLMMessageRole::User,
                content: Some("What did we do so far?".to_string()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
                reasoning_content: None,
            });
            result.push(LLMMessage {
                role: LLMMessageRole::Assistant,
                content: Some(msg.content.clone()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
                reasoning_content: None,
            });
            continue;
        }

        let llm_msg = match msg.role {
            MessageRole::User => LLMMessage {
                role: LLMMessageRole::User,
                content: Some(msg.content.clone()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
                reasoning_content: None,
            },
            MessageRole::Assistant => LLMMessage {
                role: LLMMessageRole::Assistant,
                content: Some(msg.content.clone()),
                tool_calls: msg.tool_calls.clone().map(|tcs| {
                    tcs.into_iter()
                        .map(|tc| crate::core::types::llm::LLMToolCall {
                            id: tc.id,
                            call_type: "function".to_string(),
                            function: crate::core::types::llm::LLMToolCallFunction {
                                name: tc.name,
                                arguments: tc.arguments,
                            },
                        })
                        .collect()
                }),
                tool_call_id: None,
                name: None,
                reasoning_content: None,
            },
            MessageRole::Tool => LLMMessage {
                role: LLMMessageRole::Tool,
                content: Some(msg.content.clone()),
                tool_calls: None,
                tool_call_id: msg.tool_call_id.clone(),
                name: None,
                reasoning_content: None,
            },
            _ => continue,
        };

        result.push(llm_msg);
    }

    result
}

pub fn build_compact_context(messages: &[Message]) -> Vec<LLMMessage> {
    let llm_messages = convert_messages_to_llm_format(messages);

    let compact_system_prompt = get_compact_system_prompt();
    let compact_user_prompt = get_compact_user_prompt();

    let mut context: Vec<LLMMessage> = vec![LLMMessage {
        role: LLMMessageRole::System,
        content: Some(compact_system_prompt.to_string()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        reasoning_content: None,
    }];
    context.extend(llm_messages);
    context.push(LLMMessage {
        role: LLMMessageRole::User,
        content: Some(compact_user_prompt.to_string()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        reasoning_content: None,
    });

    context
}

pub fn create_summary_message(session_id: &str, content: &str) -> CreateMessageData {
    CreateMessageData {
        id: uuid::Uuid::new_v4().to_string(),
        session_id: session_id.to_string(),
        role: MessageRole::Summary,
        content: content.to_string(),
        tool_calls: None,
        tool_calls_json: None,
        tool_name: None,
        tool_call_id: None,
        tool_success: None,
        metadata: None,
    }
}
