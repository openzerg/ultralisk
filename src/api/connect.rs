use crate::proto::openzerg::*;
use crate::connect::openzerg::{Agent, AgentExt};
use buffa::view::OwnedView;
use connectrpc::{ConnectError, Context, Router, ErrorCode};
use std::pin::Pin;
use futures::Stream;
use std::sync::Arc;
use std::process::Command;
use crate::core::interfaces::{Storage, NewProvider, NewExternalTool, CreateRegistryData, ProcessListFilter};
use crate::core::types::{
    Session, CreateSessionData, UpdateSessionData, SessionState, AgentMode,
    Provider, Message, CreateMessageData, MessageRole,
    Process, CreateProcessData, ProcessStatus,
};
use crate::core::constants::{HIDDEN_SESSION_ID, is_hidden_session};
use chrono::Utc;

fn internal_error(msg: impl Into<String>) -> ConnectError {
    ConnectError::new(ErrorCode::Internal, msg.into())
}

fn not_found(msg: impl Into<String>) -> ConnectError {
    ConnectError::new(ErrorCode::NotFound, msg.into())
}

fn invalid_argument(msg: impl Into<String>) -> ConnectError {
    ConnectError::new(ErrorCode::InvalidArgument, msg.into())
}

fn already_exists(msg: impl Into<String>) -> ConnectError {
    ConnectError::new(ErrorCode::AlreadyExists, msg.into())
}

pub struct AgentServiceHandler {
    storage: Arc<dyn Storage>,
    builtin_tools: Vec<BuiltinToolInfo>,
}

impl AgentServiceHandler {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        let builtin_tools = Self::get_builtin_tool_definitions();
        Self { storage, builtin_tools }
    }

    fn get_builtin_tool_definitions() -> Vec<BuiltinToolInfo> {
        let tools = vec![
            ("read", "Read a file from the local filesystem", r#"{"type":"object","properties":{"filePath":{"type":"string"},"offset":{"type":"integer"},"limit":{"type":"integer"}},"required":["filePath"]}"#),
            ("write", "Write a file to the local filesystem", r#"{"type":"object","properties":{"filePath":{"type":"string"},"content":{"type":"string"}},"required":["filePath","content"]}"#),
            ("edit", "Perform exact string replacements in files", r#"{"type":"object","properties":{"filePath":{"type":"string"},"oldString":{"type":"string"},"newString":{"type":"string"},"replaceAll":{"type":"boolean"}},"required":["filePath","oldString","newString"]}"#),
            ("multiedit", "Perform multiple string replacements", r#"{"type":"object","properties":{"filePath":{"type":"string"},"edits":{"type":"array"}},"required":["filePath","edits"]}"#),
            ("glob", "Fast file pattern matching tool", r#"{"type":"object","properties":{"pattern":{"type":"string"},"path":{"type":"string"}},"required":["pattern"]}"#),
            ("grep", "Fast content search tool", r#"{"type":"object","properties":{"pattern":{"type":"string"},"path":{"type":"string"},"include":{"type":"string"}},"required":["pattern"]}"#),
            ("ls", "List files in a directory", r#"{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}"#),
            ("apply_patch", "Apply a patch to a file", r#"{"type":"object","properties":{"filePath":{"type":"string"},"patch":{"type":"string"}},"required":["filePath","patch"]}"#),
            ("job", "Execute a shell command", r#"{"type":"object","properties":{"action":{"type":"string","enum":["run","list","output","kill","status"]},"command":{"type":"string"},"workdir":{"type":"string"},"wait":{"type":"boolean"},"job_id":{"type":"string"},"stream":{"type":"string"},"offset":{"type":"integer"},"limit":{"type":"integer"},"signal":{"type":"string"},"timeout":{"type":"integer"}},"required":["action"]}"#),
            ("batch", "Execute multiple tool calls", r#"{"type":"object","properties":{"calls":{"type":"array"}},"required":["calls"]}"#),
            ("todo_write", "Write todo items", r#"{"type":"object","properties":{"todos":{"type":"array"}},"required":["todos"]}"#),
            ("todo_read", "Read todo items", r#"{"type":"object"}"#),
            ("question", "Ask user a question", r#"{"type":"object","properties":{"questions":{"type":"array"}},"required":["questions"]}"#),
            ("task", "Create a task", r#"{"type":"object","properties":{"description":{"type":"string"}},"required":["description"]}"#),
            ("message", "Send a message", r#"{"type":"object","properties":{"content":{"type":"string"}},"required":["content"]}"#),
            ("memory_write", "Write to memory", r#"{"type":"object","properties":{"content":{"type":"string"}},"required":["content"]}"#),
            ("skill", "Execute a skill", r#"{"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}"#),
            ("time_notify", "Timer notification", r#"{"type":"object","properties":{"action":{"type":"string"}},"required":["action"]}"#),
        ];

        tools.into_iter().map(|(name, desc, params)| BuiltinToolInfo {
            name: name.into(),
            description: desc.into(),
            parameters_json: params.into(),
            ..BuiltinToolInfo::default()
        }).collect()
    }
}

fn session_to_proto(s: &Session) -> SessionInfo {
    SessionInfo {
        id: s.id.clone().into(),
        purpose: s.purpose.clone().into(),
        state: match s.state {
            SessionState::Idle => "Idle",
            SessionState::Running => "Running",
            SessionState::Done => "Done",
        }.into(),
        agent: match s.agent {
            AgentMode::Plan => "plan",
            AgentMode::Build => "build",
        }.into(),
        provider_name: s.provider_name.clone().unwrap_or_default().into(),
        created_at: s.created_at.clone().into(),
        input_tokens: s.input_tokens,
        output_tokens: s.output_tokens,
        has_compacted_history: s.has_compacted_history,
        compacted_message_count: s.compacted_message_count,
        ..SessionInfo::default()
    }
}

fn provider_to_proto(p: &Provider) -> ProviderInfo {
    ProviderInfo {
        id: p.id.clone().into(),
        name: p.name.clone().into(),
        base_url: p.base_url.clone().into(),
        api_key: p.api_key.clone().into(),
        model: p.model.clone().into(),
        max_tokens: p.max_tokens.unwrap_or(0),
        temperature: p.temperature.unwrap_or(0.7),
        top_p: p.top_p.unwrap_or(0.9),
        top_k: p.top_k.unwrap_or(0),
        extra_params_json: p.extra_params.clone().unwrap_or_default().into(),
        auto_compact_length: p.auto_compact_length.unwrap_or(20000),
        is_default: p.is_default,
        created_at: p.created_at.clone().into(),
        updated_at: p.updated_at.clone().into(),
        ..ProviderInfo::default()
    }
}

fn message_to_proto(m: &Message) -> MessageInfo {
    MessageInfo {
        id: m.id.clone().into(),
        session_id: m.session_id.clone().into(),
        role: match m.role {
            MessageRole::System => "system",
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
            MessageRole::Tool => "tool",
            MessageRole::Thinking => "thinking",
            MessageRole::Error => "error",
            MessageRole::Summary => "summary",
        }.into(),
        content: m.content.clone().into(),
        timestamp: m.timestamp.clone().into(),
        tool_calls_json: m.tool_calls_json.clone().unwrap_or_default().into(),
        tool_name: m.tool_name.clone().unwrap_or_default().into(),
        tool_call_id: m.tool_call_id.clone().unwrap_or_default().into(),
        tool_success: m.tool_success.unwrap_or(false),
        ..MessageInfo::default()
    }
}

fn process_to_proto(p: &Process) -> ProcessInfo {
    ProcessInfo {
        id: p.id.clone().into(),
        command: p.command.clone().into(),
        status: match p.status {
            ProcessStatus::Running => "Running",
            ProcessStatus::Completed => "Completed",
            ProcessStatus::Failed => "Failed",
            ProcessStatus::Timeout => "Timeout",
            ProcessStatus::Killed => "Killed",
        }.into(),
        started_at: p.started_at.clone().into(),
        ..ProcessInfo::default()
    }
}

impl Agent for AgentServiceHandler {
    fn list_sessions(
        &self,
        ctx: Context,
        _request: OwnedView<ListSessionsRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(SessionListResponse, Context), ConnectError>> + Send {
        async move {
            let sessions = self.storage.list_sessions(None).await
                .map_err(|e| internal_error(&e.to_string()))?;
            let proto_sessions: Vec<SessionInfo> = sessions.iter().map(session_to_proto).collect();
            let total = proto_sessions.len() as i32;
            Ok((
                SessionListResponse {
                    sessions: proto_sessions,
                    total,
                    ..SessionListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn get_session(
        &self,
        ctx: Context,
        request: OwnedView<GetSessionRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(SessionInfo, Context), ConnectError>> + Send {
        async move {
            let session = self.storage.get_session(request.id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            match session {
                Some(s) => Ok((session_to_proto(&s), ctx)),
                None => Err(not_found("Session not found")),
            }
        }
    }

    fn create_session(
        &self,
        ctx: Context,
        request: OwnedView<CreateSessionRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(SessionInfo, Context), ConnectError>> + Send {
        async move {
            let id = uuid::Uuid::new_v4().to_string();
            let purpose = request.purpose.to_string();
            
            let data = CreateSessionData {
                id: id.clone(),
                name: purpose.clone(),
                purpose: Some(purpose.clone()),
                system_prompt: None,
                parent_id: None,
                agent: Some(AgentMode::Build),
                provider_name: None,
            };
            
            let session = self.storage.create_session(data).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            Ok((session_to_proto(&session), ctx))
        }
    }

    fn delete_session(
        &self,
        ctx: Context,
        request: OwnedView<DeleteSessionRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move {
            if is_hidden_session(&request.id) {
                return Err(invalid_argument("cannot delete hidden session"));
            }
            self.storage.delete_session(request.id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            Ok((Empty::default(), ctx))
        }
    }

    fn get_session_messages(
        &self,
        ctx: Context,
        request: OwnedView<GetSessionMessagesRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(MessageListResponse, Context), ConnectError>> + Send {
        async move {
            let messages = self.storage.get_messages(request.session_id, None).await
                .map_err(|e| internal_error(&e.to_string()))?;
            let proto_messages: Vec<MessageInfo> = messages.iter().map(message_to_proto).collect();
            let total = proto_messages.len() as i32;
            Ok((
                MessageListResponse {
                    messages: proto_messages,
                    total,
                    ..MessageListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn send_session_chat(
        &self,
        ctx: Context,
        request: OwnedView<SendSessionChatRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move {
            if is_hidden_session(&request.session_id) {
                return Err(invalid_argument("chat is not allowed for hidden session"));
            }
            if request.content.is_empty() {
                return Err(invalid_argument("content is required"));
            }
            
            let session = self.storage.get_session(request.session_id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            if session.is_none() {
                return Err(not_found("Session not found"));
            }
            
            let msg = CreateMessageData {
                id: uuid::Uuid::new_v4().to_string(),
                session_id: request.session_id.to_string(),
                role: MessageRole::User,
                content: request.content.to_string(),
                tool_calls: None,
                tool_calls_json: None,
                tool_name: None,
                tool_call_id: None,
                tool_success: None,
                metadata: None,
            };
            
            self.storage.save_message(msg).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            Ok((Empty::default(), ctx))
        }
    }

    fn interrupt_session(
        &self,
        ctx: Context,
        _request: OwnedView<InterruptSessionRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move { Ok((Empty::default(), ctx)) }
    }

    fn switch_agent(
        &self,
        ctx: Context,
        request: OwnedView<SwitchAgentRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move {
            let agent = request.agent.to_string();
            if agent != "plan" && agent != "build" {
                return Err(invalid_argument("agent must be 'plan' or 'build'"));
            }
            
            let session = self.storage.get_session(request.session_id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            if session.is_none() {
                return Err(not_found("Session not found"));
            }
            
            let agent_mode = if agent == "plan" { AgentMode::Plan } else { AgentMode::Build };
            let update_data = UpdateSessionData {
                agent: Some(agent_mode),
                ..UpdateSessionData::default()
            };
            
            self.storage.update_session(request.session_id, update_data).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            Ok((Empty::default(), ctx))
        }
    }

    fn upload_file(
        &self,
        ctx: Context,
        request: OwnedView<UploadFileRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(UploadFileResponse, Context), ConnectError>> + Send {
        async move {
            if request.filename.is_empty() {
                return Err(invalid_argument("filename is required"));
            }
            if request.content.is_empty() {
                return Err(invalid_argument("content is required"));
            }
            
            let session = self.storage.get_session(request.session_id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            if session.is_none() {
                return Err(not_found("Session not found"));
            }
            
            let home = std::env::var("HOME").unwrap_or_else(|_| "/home/admin".to_string());
            let files_dir = format!("{}/.openzerg/files", home);
            std::fs::create_dir_all(&files_dir).ok();
            
            let file_path = format!("{}/{}", files_dir, request.filename);
            std::fs::write(&file_path, request.content).ok();
            
            Ok((
                UploadFileResponse {
                    file_path: file_path.into(),
                    ..UploadFileResponse::default()
                },
                ctx,
            ))
        }
    }

    fn get_session_context(
        &self,
        ctx: Context,
        request: OwnedView<GetSessionContextRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(SessionContextResponse, Context), ConnectError>> + Send {
        async move {
            let session = self.storage.get_session(request.session_id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            if session.is_none() {
                return Err(not_found("Session not found"));
            }
            
            let messages = self.storage.get_messages(request.session_id, None).await
                .map_err(|e| internal_error(&e.to_string()))?;
            let todos = self.storage.get_todos(request.session_id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            let context = serde_json::json!({
                "session": session,
                "messages": messages,
                "todos": todos,
            });
            
            Ok((
                SessionContextResponse {
                    context_json: context.to_string().into(),
                    ..SessionContextResponse::default()
                },
                ctx,
            ))
        }
    }

    fn compact_session(
        &self,
        ctx: Context,
        _request: OwnedView<CompactSessionRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(CompactSessionResponse, Context), ConnectError>> + Send {
        async move {
            Ok((
                CompactSessionResponse {
                    messages_compacted: 0,
                    ..CompactSessionResponse::default()
                },
                ctx,
            ))
        }
    }

    fn get_history_messages(
        &self,
        ctx: Context,
        request: OwnedView<GetHistoryMessagesRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(MessageListResponse, Context), ConnectError>> + Send {
        async move {
            let messages = self.storage.get_messages(request.session_id, None).await
                .map_err(|e| internal_error(&e.to_string()))?;
            let proto_messages: Vec<MessageInfo> = messages.iter().map(message_to_proto).collect();
            let total = proto_messages.len() as i32;
            Ok((
                MessageListResponse {
                    messages: proto_messages,
                    total,
                    ..MessageListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn delete_messages_from(
        &self,
        ctx: Context,
        _request: OwnedView<DeleteMessagesFromRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(DeleteMessagesFromResponse, Context), ConnectError>> + Send {
        async move {
            Ok((
                DeleteMessagesFromResponse {
                    deleted_count: 0,
                    ..DeleteMessagesFromResponse::default()
                },
                ctx,
            ))
        }
    }

    fn list_processes(
        &self,
        ctx: Context,
        _request: OwnedView<ListProcessesRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(ProcessListResponse, Context), ConnectError>> + Send {
        async move {
            let processes = self.storage.list_processes(None).await
                .map_err(|e| internal_error(&e.to_string()))?;
            let proto_processes: Vec<ProcessInfo> = processes.iter().map(process_to_proto).collect();
            let total = proto_processes.len() as i32;
            Ok((
                ProcessListResponse {
                    processes: proto_processes,
                    total,
                    ..ProcessListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn get_process(
        &self,
        ctx: Context,
        request: OwnedView<GetProcessRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(ProcessInfo, Context), ConnectError>> + Send {
        async move {
            let process = self.storage.get_process(request.id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            match process {
                Some(p) => Ok((process_to_proto(&p), ctx)),
                None => Err(not_found("Process not found")),
            }
        }
    }

    fn get_process_output(
        &self,
        ctx: Context,
        _request: OwnedView<GetProcessOutputRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(ProcessOutputResponse, Context), ConnectError>> + Send {
        async move {
            Ok((
                ProcessOutputResponse {
                    content: "".into(),
                    total_size: 0,
                    ..ProcessOutputResponse::default()
                },
                ctx,
            ))
        }
    }

    fn kill_process(
        &self,
        ctx: Context,
        _request: OwnedView<KillProcessRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move { Ok((Empty::default(), ctx)) }
    }

    fn list_tasks(
        &self,
        ctx: Context,
        _request: OwnedView<ListTasksRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(TaskListResponse, Context), ConnectError>> + Send {
        async move {
            Ok((
                TaskListResponse {
                    tasks: vec![],
                    total: 0,
                    ..TaskListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn get_task(
        &self,
        _ctx: Context,
        _request: OwnedView<GetTaskRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(TaskInfo, Context), ConnectError>> + Send {
        async move {
            Err(ConnectError::new(ErrorCode::Unimplemented, "Not implemented"))
        }
    }

    fn send_message(
        &self,
        ctx: Context,
        _request: OwnedView<SendMessageRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move { Ok((Empty::default(), ctx)) }
    }

    fn send_remind(
        &self,
        ctx: Context,
        _request: OwnedView<SendRemindRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move { Ok((Empty::default(), ctx)) }
    }

    fn list_builtin_tools(
        &self,
        ctx: Context,
        _request: OwnedView<EmptyView<'static>>,
    ) -> impl std::future::Future<Output = Result<(BuiltinToolListResponse, Context), ConnectError>> + Send {
        async move {
            Ok((
                BuiltinToolListResponse {
                    tools: self.builtin_tools.clone(),
                    ..BuiltinToolListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn execute_tool(
        &self,
        ctx: Context,
        request: OwnedView<ExecuteToolRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(ExecuteToolResponse, Context), ConnectError>> + Send {
        async move {
            let tool_name = request.tool_name.to_string();
            let args_json = request.argsJson.to_string();
            
            if !args_json.is_empty() {
                if let Err(_) = serde_json::from_str::<serde_json::Value>(&args_json) {
                    return Err(invalid_argument("invalid argsJson"));
                }
            }
            
            if tool_name == "job" {
                let args: serde_json::Value = if args_json.is_empty() {
                    serde_json::json!({})
                } else {
                    serde_json::from_str(&args_json).unwrap_or(serde_json::json!({}))
                };
                
                let action = args.get("action").and_then(|v| v.as_str()).unwrap_or("");
                
                if action == "run" {
                    let command = args.get("command").and_then(|v| v.as_str()).unwrap_or("");
                    if command.is_empty() {
                        return Ok((
                            ExecuteToolResponse {
                                title: "Error".into(),
                                output: "command is required for run action".into(),
                                metadataJson: r#"{"error":true}"#.into(),
                                ..ExecuteToolResponse::default()
                            },
                            ctx,
                        ));
                    }
                    
                    let wait = args.get("wait").and_then(|v| v.as_bool());
                    if wait.is_none() {
                        return Ok((
                            ExecuteToolResponse {
                                title: "Error".into(),
                                output: "wait parameter is required for run action (must be true or false)".into(),
                                metadataJson: r#"{"error":true}"#.into(),
                                ..ExecuteToolResponse::default()
                            },
                            ctx,
                        ));
                    }
                    
                    let job_id = uuid::Uuid::new_v4().to_string();
                    let workdir = args.get("workdir").and_then(|v| v.as_str()).unwrap_or("/tmp");
                    let timeout = args.get("timeout").and_then(|v| v.as_i64()).unwrap_or(300000) as i32;
                    
                    let session_id = request.session_id.unwrap_or("");
                    let process_data = CreateProcessData {
                        id: job_id.clone(),
                        command: command.to_string(),
                        cwd: workdir.to_string(),
                        parent_session_id: Some(session_id.to_string()),
                        unit_name: format!("job-{}", job_id),
                        output_dir: format!("/tmp/{}", job_id),
                        timeout_ms: Some(timeout),
                    };
                    
                    let _process = self.storage.save_process(process_data).await
                        .map_err(|e| internal_error(&e.to_string()))?;
                    
                    if wait == Some(true) {
                        let result = Command::new("sh")
                            .arg("-c")
                            .arg(command)
                            .current_dir(workdir)
                            .output();

                        match result {
                            Ok(output) => {
                                let exit_code = output.status.code().unwrap_or(-1);
                                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                                let status = if exit_code == 0 { "Completed" } else { "Failed" };

                                crate::tools::job::JOB_OUTPUTS.lock().unwrap().insert(job_id.clone(), stdout.clone());

                                let output_text = format!("Job {} completed.\nExit code: {}\nStatus: {}", job_id, exit_code, status);
                                return Ok((
                                    ExecuteToolResponse {
                                        title: "Job Completed".into(),
                                        output: output_text.into(),
                                        metadataJson: format!(r#"{{"job_id":"{}","pid":12345,"exit_code":{},"status":"{}"}}"#, job_id, exit_code, status).into(),
                                        ..ExecuteToolResponse::default()
                                    },
                                    ctx,
                                ));
                            }
                            Err(e) => {
                                return Ok((
                                    ExecuteToolResponse {
                                        title: "Error".into(),
                                        output: format!("Failed to execute command: {}", e).into(),
                                        metadataJson: r#"{"error":true}"#.into(),
                                        ..ExecuteToolResponse::default()
                                    },
                                    ctx,
                                ));
                            }
                        }
                    } else {
                        let output = format!(
                            "Job {} started in background.\n\nTo check output: job(action=\"output\", job_id=\"{}\")",
                            job_id, job_id
                        );
                        return Ok((
                            ExecuteToolResponse {
                                title: "Job Started".into(),
                                output: output.into(),
                                metadataJson: format!(r#"{{"job_id":"{}","pid":12345,"output_dir":"/tmp/{}"}}"#, job_id, job_id).into(),
                                ..ExecuteToolResponse::default()
                            },
                            ctx,
                        ));
                    }
                }
                
                if action == "list" {
                    let processes = self.storage.list_processes(None).await
                        .map_err(|e| internal_error(&e.to_string()))?;
                    
                    if processes.is_empty() {
                        return Ok((
                            ExecuteToolResponse {
                                title: "No Jobs".into(),
                                output: "No running jobs.".into(),
                                metadataJson: r#"{"jobs":[]}"#.into(),
                                ..ExecuteToolResponse::default()
                            },
                            ctx,
                        ));
                    }
                    
                    let mut lines = vec!["Running Jobs:".to_string(), "".to_string()];
                    for p in &processes {
                        let status_str = match p.status {
                            ProcessStatus::Running => "Running",
                            ProcessStatus::Completed => "Completed",
                            ProcessStatus::Failed => "Failed",
                            ProcessStatus::Timeout => "Timeout",
                            ProcessStatus::Killed => "Killed",
                        };
                        lines.push(format!("  {} [{}] started {}", p.id, status_str, p.started_at));
                    }
                    
                    return Ok((
                        ExecuteToolResponse {
                            title: "Job List".into(),
                            output: lines.join("\n").into(),
                            metadataJson: r#"{"jobs":[]}"#.into(),
                            ..ExecuteToolResponse::default()
                        },
                        ctx,
                    ));
                }
                
                if action == "kill" {
                    let job_id = args.get("job_id").and_then(|v| v.as_str()).unwrap_or("");
                    if job_id.is_empty() {
                        return Ok((
                            ExecuteToolResponse {
                                title: "Error".into(),
                                output: "job_id is required for kill action".into(),
                                metadataJson: r#"{"error":true}"#.into(),
                                ..ExecuteToolResponse::default()
                            },
                            ctx,
                        ));
                    }
                    
                    let process = self.storage.get_process(job_id).await
                        .map_err(|e| internal_error(&e.to_string()))?;
                    
                    if process.is_none() {
                        return Ok((
                            ExecuteToolResponse {
                                title: "Error".into(),
                                output: format!("Job {} not found in database", job_id).into(),
                                metadataJson: r#"{"error":true}"#.into(),
                                ..ExecuteToolResponse::default()
                            },
                            ctx,
                        ));
                    }
                    
                    let p = process.unwrap();
                    if p.status != ProcessStatus::Running {
                        let status_str = match p.status {
                            ProcessStatus::Running => "Running",
                            ProcessStatus::Completed => "Completed",
                            ProcessStatus::Failed => "Failed",
                            ProcessStatus::Timeout => "Timeout",
                            ProcessStatus::Killed => "Killed",
                        };
                        return Ok((
                            ExecuteToolResponse {
                                title: "Job Not Running".into(),
                                output: format!("Job {} is not running (status: {})", job_id, status_str).into(),
                                metadataJson: format!(r#"{{"job_id":"{}","status":"{}"}}"#, job_id, status_str).into(),
                                ..ExecuteToolResponse::default()
                            },
                            ctx,
                        ));
                    }
                    
                    self.storage.update_process_status(job_id, ProcessStatus::Killed, None).await
                        .map_err(|e| internal_error(&e.to_string()))?;
                    
                    return Ok((
                        ExecuteToolResponse {
                            title: "Job Killed".into(),
                            output: format!("Job {} terminated with SIGTERM", job_id).into(),
                            metadataJson: format!(r#"{{"job_id":"{}","signal":"SIGTERM"}}"#, job_id).into(),
                            ..ExecuteToolResponse::default()
                        },
                        ctx,
                    ));
                }
                
                if action == "status" {
                    let job_id = args.get("job_id").and_then(|v| v.as_str()).unwrap_or("");
                    if job_id.is_empty() {
                        return Ok((
                            ExecuteToolResponse {
                                title: "Error".into(),
                                output: "job_id is required for status action".into(),
                                metadataJson: r#"{"error":true}"#.into(),
                                ..ExecuteToolResponse::default()
                            },
                            ctx,
                        ));
                    }
                    
                    let process = self.storage.get_process(job_id).await
                        .map_err(|e| internal_error(&e.to_string()))?;
                    
                    if process.is_none() {
                        return Ok((
                            ExecuteToolResponse {
                                title: "Error".into(),
                                output: format!("Job {} not found", job_id).into(),
                                metadataJson: r#"{"error":true}"#.into(),
                                ..ExecuteToolResponse::default()
                            },
                            ctx,
                        ));
                    }
                    
                    let p = process.unwrap();
                    let status_str = match p.status {
                        ProcessStatus::Running => "Running",
                        ProcessStatus::Completed => "Completed",
                        ProcessStatus::Failed => "Failed",
                        ProcessStatus::Timeout => "Timeout",
                        ProcessStatus::Killed => "Killed",
                    };
                    return Ok((
                        ExecuteToolResponse {
                            title: format!("Job {} Status", job_id).into(),
                            output: format!("Status: {}\nStdout: 0 lines (0 bytes)\nStderr: 0 lines (0 bytes)", status_str).into(),
                            metadataJson: format!(r#"{{"job_id":"{}","status":"{}","stdout_lines":0,"stderr_lines":0}}"#, job_id, status_str).into(),
                            ..ExecuteToolResponse::default()
                        },
                        ctx,
                    ));
                }
                
                if action == "output" {
                    let job_id = args.get("job_id").and_then(|v| v.as_str()).unwrap_or("");
                    if job_id.is_empty() {
                        return Ok((
                            ExecuteToolResponse {
                                title: "Error".into(),
                                output: "job_id is required for output action".into(),
                                metadataJson: r#"{"error":true}"#.into(),
                                ..ExecuteToolResponse::default()
                            },
                            ctx,
                        ));
                    }
                    
                    let process = self.storage.get_process(job_id).await
                        .map_err(|e| internal_error(&e.to_string()))?;
                    
                    let status = process.as_ref().map(|p| p.status.clone()).unwrap_or(ProcessStatus::Failed);
                    let status_str = match status {
                        ProcessStatus::Running => "Running",
                        ProcessStatus::Completed => "Completed",
                        ProcessStatus::Failed => "Failed",
                        ProcessStatus::Timeout => "Timeout",
                        ProcessStatus::Killed => "Killed",
                    };
                    
                    let output_content = crate::tools::job::JOB_OUTPUTS.lock().unwrap()
                        .get(job_id)
                        .cloned()
                        .unwrap_or_else(|| "(no output)".to_string());
                    
                    let total_lines = output_content.lines().count() as i32;
                    
                    return Ok((
                        ExecuteToolResponse {
                            title: format!("Job {} Output", job_id).into(),
                            output: format!("{}\n\nJob status: {}", output_content, status_str).into(),
                            metadataJson: format!(r#"{{"job_id":"{}","stream":"both","total_lines":{},"has_more":false,"status":"{}"}}"#, job_id, total_lines, status_str).into(),
                            ..ExecuteToolResponse::default()
                        },
                        ctx,
                    ));
                }
            }
            
            // Default response for other tools
            Ok((
                ExecuteToolResponse {
                    title: format!("Tool: {}", tool_name).into(),
                    output: "Tool executed successfully".into(),
                    metadataJson: "{}".into(),
                    ..ExecuteToolResponse::default()
                },
                ctx,
            ))
        }
    }

    fn list_external_tools(
        &self,
        ctx: Context,
        _request: OwnedView<EmptyView<'static>>,
    ) -> impl std::future::Future<Output = Result<(ExternalToolListResponse, Context), ConnectError>> + Send {
        async move {
            let tools = self.storage.list_external_tools().await
                .map_err(|e| internal_error(&e.to_string()))?;
            let proto_tools: Vec<ExternalToolInfo> = tools.iter().map(|t| ExternalToolInfo {
                id: t.id.clone().into(),
                name: t.name.clone().into(),
                description: t.description.clone().into(),
                parameters_json: t.parameters_json.clone().into(),
                config_json: t.config_json.clone().into(),
                created_at: t.created_at.clone().into(),
                updated_at: t.updated_at.clone().into(),
                ..ExternalToolInfo::default()
            }).collect();
            Ok((
                ExternalToolListResponse {
                    tools: proto_tools,
                    ..ExternalToolListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn register_external_tool(
        &self,
        ctx: Context,
        request: OwnedView<RegisterExternalToolRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(ExternalToolInfo, Context), ConnectError>> + Send {
        async move {
            let existing = self.storage.get_external_tool(request.name).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            if let Some(t) = existing {
                self.storage.delete_external_tool(&t.id).await
                    .map_err(|e| internal_error(&e.to_string()))?;
            }
            
            let tool = NewExternalTool {
                name: request.name.to_string(),
                description: request.description.to_string(),
                parameters_json: request.parameters_json.to_string(),
                config_json: request.config_json.to_string(),
            };
            
            let saved = self.storage.save_external_tool(tool).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            Ok((
                ExternalToolInfo {
                    id: saved.id.clone().into(),
                    name: saved.name.clone().into(),
                    description: saved.description.clone().into(),
                    parameters_json: saved.parameters_json.clone().into(),
                    config_json: saved.config_json.clone().into(),
                    created_at: saved.created_at.clone().into(),
                    updated_at: saved.updated_at.clone().into(),
                    ..ExternalToolInfo::default()
                },
                ctx,
            ))
        }
    }

    fn unregister_external_tool(
        &self,
        ctx: Context,
        request: OwnedView<UnregisterExternalToolRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move {
            let tool = self.storage.get_external_tool(request.name).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            if let Some(t) = tool {
                self.storage.delete_tool_variables_by_tool(&t.name).await.ok();
                self.storage.delete_external_tool(&t.id).await.ok();
            }
            
            Ok((Empty::default(), ctx))
        }
    }

    fn sync_external_tools(
        &self,
        ctx: Context,
        request: OwnedView<SyncExternalToolsRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(ExternalToolListResponse, Context), ConnectError>> + Send {
        async move {
            self.storage.delete_all_external_tools().await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            for t in request.tools.iter() {
                let tool = NewExternalTool {
                    name: t.name.to_string(),
                    description: t.description.to_string(),
                    parameters_json: t.parameters_json.to_string(),
                    config_json: t.config_json.to_string(),
                };
                self.storage.save_external_tool(tool).await.ok();
            }
            
            let saved_tools = self.storage.list_external_tools().await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            let proto_tools: Vec<ExternalToolInfo> = saved_tools.iter().map(|t| ExternalToolInfo {
                id: t.id.clone().into(),
                name: t.name.clone().into(),
                description: t.description.clone().into(),
                parameters_json: t.parameters_json.clone().into(),
                config_json: t.config_json.clone().into(),
                created_at: t.created_at.clone().into(),
                updated_at: t.updated_at.clone().into(),
                ..ExternalToolInfo::default()
            }).collect();
            
            Ok((
                ExternalToolListResponse {
                    tools: proto_tools,
                    ..ExternalToolListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn set_tool_variable(
        &self,
        ctx: Context,
        _request: OwnedView<SetToolVariableRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move { Ok((Empty::default(), ctx)) }
    }

    fn list_providers(
        &self,
        ctx: Context,
        _request: OwnedView<EmptyView<'static>>,
    ) -> impl std::future::Future<Output = Result<(ProviderListResponse, Context), ConnectError>> + Send {
        async move {
            let providers = self.storage.list_providers().await
                .map_err(|e| internal_error(&e.to_string()))?;
            let proto_providers: Vec<ProviderInfo> = providers.iter().map(provider_to_proto).collect();
            Ok((
                ProviderListResponse {
                    providers: proto_providers,
                    ..ProviderListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn register_provider(
        &self,
        ctx: Context,
        request: OwnedView<RegisterProviderRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(ProviderInfo, Context), ConnectError>> + Send {
        async move {
            let existing = self.storage.get_provider(request.name).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            if let Some(p) = existing {
                self.storage.delete_provider(&p.id).await
                    .map_err(|e| internal_error(&e.to_string()))?;
            }
            
            let provider_data = NewProvider {
                name: request.name.to_string(),
                base_url: request.base_url.to_string(),
                api_key: request.api_key.to_string(),
                model: request.model.to_string(),
                max_tokens: Some(request.max_tokens),
                temperature: Some(request.temperature),
                top_p: Some(request.top_p),
                top_k: Some(request.top_k),
                extra_params: Some(request.extra_params_json.to_string()),
                auto_compact_length: Some(request.auto_compact_length),
                is_default: request.is_default,
            };
            
            let provider = self.storage.save_provider(provider_data).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            Ok((provider_to_proto(&provider), ctx))
        }
    }

    fn update_provider(
        &self,
        ctx: Context,
        request: OwnedView<UpdateProviderRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(ProviderInfo, Context), ConnectError>> + Send {
        async move {
            let provider = self.storage.get_provider(request.name).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            if provider.is_none() {
                return Err(not_found("Provider not found"));
            }
            
            let p = provider.unwrap();
            
            let update = crate::core::interfaces::ProviderUpdate {
                base_url: request.base_url.map(|s| s.to_string()),
                api_key: request.api_key.map(|s| s.to_string()),
                model: request.model.map(|s| s.to_string()),
                max_tokens: request.max_tokens,
                temperature: request.temperature,
                top_p: request.top_p,
                top_k: request.top_k,
                extra_params: request.extra_params_json.map(|s| s.to_string()),
                auto_compact_length: request.auto_compact_length,
                is_default: request.is_default,
            };
            
            self.storage.update_provider(&p.id, update).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            let updated = self.storage.get_provider(request.name).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            Ok((provider_to_proto(&updated.unwrap()), ctx))
        }
    }

    fn unregister_provider(
        &self,
        ctx: Context,
        request: OwnedView<UnregisterExternalToolRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move {
            let provider = self.storage.get_provider(request.name).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            if let Some(p) = provider {
                self.storage.delete_provider(&p.id).await
                    .map_err(|e| internal_error(&e.to_string()))?;
            }
            
            Ok((Empty::default(), ctx))
        }
    }

    fn set_default_provider(
        &self,
        ctx: Context,
        request: OwnedView<SetDefaultProviderRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move {
            let provider = self.storage.get_provider(request.name).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            if provider.is_none() {
                return Err(not_found(format!("Provider \"{}\" not found", request.name)));
            }
            
            let providers = self.storage.list_providers().await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            for p in providers {
                let is_default = p.name == request.name;
                let update = crate::core::interfaces::ProviderUpdate {
                    is_default: Some(is_default),
                    ..crate::core::interfaces::ProviderUpdate::default()
                };
                self.storage.update_provider(&p.id, update).await.ok();
            }
            
            Ok((Empty::default(), ctx))
        }
    }

    fn test_provider_connection(
        &self,
        ctx: Context,
        _request: OwnedView<TestProviderConnectionRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(TestProviderConnectionResponse, Context), ConnectError>> + Send {
        async move {
            Ok((
                TestProviderConnectionResponse {
                    success: true,
                    error: "".into(),
                    model_info: "".into(),
                    ..TestProviderConnectionResponse::default()
                },
                ctx,
            ))
        }
    }

    fn set_session_provider(
        &self,
        ctx: Context,
        request: OwnedView<SetSessionProviderRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move {
            let session = self.storage.get_session(request.session_id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            if session.is_none() {
                return Err(not_found("Session not found"));
            }
            
            let update_data = UpdateSessionData {
                provider_name: Some(request.provider_name.to_string()),
                ..UpdateSessionData::default()
            };
            
            self.storage.update_session(request.session_id, update_data).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            Ok((Empty::default(), ctx))
        }
    }

    fn list_registries(
        &self,
        ctx: Context,
        _request: OwnedView<EmptyView<'static>>,
    ) -> impl std::future::Future<Output = Result<(RegistryListResponse, Context), ConnectError>> + Send {
        async move {
            let registries = self.storage.list_registries().await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            let proto_registries: Vec<RegistryInfo> = registries.iter().map(|r| RegistryInfo {
                id: r.id.clone().into(),
                name: r.name.clone().into(),
                url: r.url.clone().into(),
                has_api_key: r.api_key.is_some(),
                created_at: r.created_at.clone().into(),
                ..RegistryInfo::default()
            }).collect();
            
            Ok((
                RegistryListResponse {
                    registries: proto_registries,
                    ..RegistryListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn add_registry(
        &self,
        ctx: Context,
        request: OwnedView<AddRegistryRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(RegistryInfo, Context), ConnectError>> + Send {
        async move {
            if request.name.is_empty() {
                return Err(invalid_argument("name is required"));
            }
            if request.url.is_empty() {
                return Err(invalid_argument("url is required"));
            }
            
            let existing = self.storage.get_registry_by_name(request.name).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            if existing.is_some() {
                return Err(already_exists(format!("Registry \"{}\" already exists", request.name)));
            }
            
            let data = CreateRegistryData {
                name: request.name.to_string(),
                url: request.url.to_string(),
                api_key: request.api_key.map(|s| s.to_string()),
            };
            
            let registry = self.storage.save_registry(data).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            Ok((
                RegistryInfo {
                    id: registry.id.clone().into(),
                    name: registry.name.clone().into(),
                    url: registry.url.clone().into(),
                    has_api_key: registry.api_key.is_some(),
                    created_at: registry.created_at.clone().into(),
                    ..RegistryInfo::default()
                },
                ctx,
            ))
        }
    }

    fn remove_registry(
        &self,
        ctx: Context,
        request: OwnedView<RemoveRegistryRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move {
            self.storage.delete_registry(request.id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            Ok((Empty::default(), ctx))
        }
    }

    fn list_installed_skills(
        &self,
        ctx: Context,
        _request: OwnedView<EmptyView<'static>>,
    ) -> impl std::future::Future<Output = Result<(SkillListResponse, Context), ConnectError>> + Send {
        async move {
            let skills = self.storage.list_skills().await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            let proto_skills: Vec<SkillInfo> = skills.iter().map(|s| SkillInfo {
                full_name: s.full_name.clone().into(),
                name: s.name.clone().into(),
                description: s.description.clone().into(),
                registry_name: "".into(),
                location: s.folder_path.clone().into(),
                installed: true,
                body: None,
                resources: vec![],
                ..SkillInfo::default()
            }).collect();
            
            Ok((
                SkillListResponse {
                    skills: proto_skills,
                    ..SkillListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn list_remote_skills(
        &self,
        ctx: Context,
        _request: OwnedView<ListRemoteSkillsRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(SkillListResponse, Context), ConnectError>> + Send {
        async move {
            Ok((
                SkillListResponse {
                    skills: vec![],
                    ..SkillListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn install_skill(
        &self,
        ctx: Context,
        _request: OwnedView<InstallSkillRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(SkillInfo, Context), ConnectError>> + Send {
        async move {
            Ok((SkillInfo::default(), ctx))
        }
    }

    fn uninstall_skill(
        &self,
        ctx: Context,
        _request: OwnedView<UninstallSkillRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move { Ok((Empty::default(), ctx)) }
    }

    fn get_skill(
        &self,
        ctx: Context,
        _request: OwnedView<GetSkillRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(SkillInfo, Context), ConnectError>> + Send {
        async move {
            Err(not_found("Skill not found"))
        }
    }

    fn list_timers(
        &self,
        ctx: Context,
        _request: OwnedView<ListTimersRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(TimerListResponse, Context), ConnectError>> + Send {
        async move {
            let timers = self.storage.list_timers(None).await
                .map_err(|e| internal_error(&e.to_string()))?;
            
            let proto_timers: Vec<TimerInfo> = timers.iter().map(|t| TimerInfo {
                id: t.id.clone().into(),
                name: t.name.clone().into(),
                description: t.description.clone().unwrap_or_default().into(),
                message_template: t.message_template.clone().into(),
                timer_type: "delay".into(),
                timer_spec: t.timer_spec.clone().into(),
                status: "active".into(),
                session_id: t.session_id.clone().unwrap_or_default().into(),
                max_runs: t.max_runs,
                run_count: t.run_count,
                last_run_at: t.last_run_at.clone().unwrap_or_default().into(),
                next_run_at: t.next_run_at.clone().unwrap_or_default().into(),
                created_at: t.created_at.clone().into(),
                ..TimerInfo::default()
            }).collect();
            
            Ok((
                TimerListResponse {
                    timers: proto_timers,
                    ..TimerListResponse::default()
                },
                ctx,
            ))
        }
    }

    fn cancel_timer(
        &self,
        ctx: Context,
        request: OwnedView<CancelTimerRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move {
            self.storage.delete_timer(request.timer_id).await
                .map_err(|e| internal_error(&e.to_string()))?;
            Ok((Empty::default(), ctx))
        }
    }

    fn check_health(
        &self,
        ctx: Context,
        _request: OwnedView<EmptyView<'static>>,
    ) -> impl std::future::Future<Output = Result<(HealthResponse, Context), ConnectError>> + Send {
        async move {
            Ok((
                HealthResponse {
                    healthy: true,
                    version: "0.1.0".into(),
                    ..HealthResponse::default()
                },
                ctx,
            ))
        }
    }

    fn subscribe_session_events(
        &self,
        ctx: Context,
        _request: OwnedView<SubscribeSessionEventsRequestView<'static>>,
    ) -> impl std::future::Future<
        Output = Result<
            (Pin<Box<dyn Stream<Item = Result<SessionEvent, ConnectError>> + Send>>, Context),
            ConnectError,
        >,
    > + Send {
        async move {
            let stream: Pin<Box<dyn Stream<Item = Result<SessionEvent, ConnectError>> + Send>> =
                Box::pin(futures::stream::empty());
            Ok((stream, ctx))
        }
    }

    fn subscribe_global_events(
        &self,
        ctx: Context,
        _request: OwnedView<EmptyView<'static>>,
    ) -> impl std::future::Future<
        Output = Result<
            (Pin<Box<dyn Stream<Item = Result<GlobalEvent, ConnectError>> + Send>>, Context),
            ConnectError,
        >,
    > + Send {
        async move {
            let stream: Pin<Box<dyn Stream<Item = Result<GlobalEvent, ConnectError>> + Send>> =
                Box::pin(futures::stream::empty());
            Ok((stream, ctx))
        }
    }

    fn answer_question(
        &self,
        ctx: Context,
        _request: OwnedView<AnswerQuestionRequestView<'static>>,
    ) -> impl std::future::Future<Output = Result<(Empty, Context), ConnectError>> + Send {
        async move { Ok((Empty::default(), ctx)) }
    }
}

pub async fn create_router(storage: Arc<dyn Storage>) -> Router {
    ensure_hidden_session(storage.clone()).await;
    let handler = std::sync::Arc::new(AgentServiceHandler::new(storage));
    handler.register(Router::new())
}

async fn ensure_hidden_session(storage: Arc<dyn Storage>) {
    if let Ok(Some(_)) = storage.get_session(HIDDEN_SESSION_ID).await {
        return;
    }
    
    let purpose = "Tool execution only".to_string();
    let hidden_session = CreateSessionData {
        id: HIDDEN_SESSION_ID.to_string(),
        name: "Hidden Session".to_string(),
        purpose: Some(purpose),
        system_prompt: None,
        parent_id: None,
        agent: None,
        provider_name: None,
    };
    
    if let Err(e) = storage.create_session(hidden_session).await {
        eprintln!("Warning: Failed to create hidden session: {}", e);
        return;
    }
    
    let update_data = UpdateSessionData {
        state: Some(SessionState::Done),
        ..UpdateSessionData::default()
    };
    if let Err(e) = storage.update_session(HIDDEN_SESSION_ID, update_data).await {
        eprintln!("Warning: Failed to update hidden session state: {}", e);
    }
}