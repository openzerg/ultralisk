use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use ::connectrpc::{
    Context, ConnectError, Router, Dispatcher, view_handler_fn,
    view_streaming_handler_fn, view_client_streaming_handler_fn,
    view_bidi_streaming_handler_fn,
};
use ::connectrpc::dispatcher::codegen as __crpc_codegen;
use ::connectrpc::CodecFormat as __CodecFormat;
use buffa::bytes::Bytes as __Bytes;
use ::connectrpc::client::{
    ClientConfig, ClientTransport, CallOptions, call_unary, call_server_stream,
    call_client_stream, call_bidi_stream,
};
use futures::Stream;
use buffa::Message;
use buffa::view::OwnedView;
/// Full service name for this service.
pub const AGENT_SERVICE_NAME: &str = "openzerg.Agent";
/// Server trait for Agent.
///
/// # Implementing handlers
///
/// Handlers receive requests as `OwnedView<FooView<'static>>`, which gives
/// zero-copy borrowed access to fields (e.g. `request.name` is a `&str`
/// into the decoded buffer). The view can be held across `.await` points.
///
/// Implement methods with plain `async fn`; the returned future satisfies
/// the `Send` bound automatically. See the
/// [buffa user guide](https://github.com/anthropics/buffa/blob/main/docs/guide.md#ownedview-in-async-trait-implementations)
/// for zero-copy access patterns and when `to_owned_message()` is needed.
#[allow(clippy::type_complexity)]
pub trait Agent: Send + Sync + 'static {
    /// Session management
    fn list_sessions(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::ListSessionsRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::SessionListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the GetSession RPC.
    fn get_session(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::GetSessionRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::SessionInfo, Context), ConnectError>,
    > + Send;
    /// Handle the CreateSession RPC.
    fn create_session(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::CreateSessionRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::SessionInfo, Context), ConnectError>,
    > + Send;
    /// Handle the DeleteSession RPC.
    fn delete_session(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::DeleteSessionRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Handle the GetSessionMessages RPC.
    fn get_session_messages(
        &self,
        ctx: Context,
        request: OwnedView<
            crate::proto::openzerg::GetSessionMessagesRequestView<'static>,
        >,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::MessageListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the SendSessionChat RPC.
    fn send_session_chat(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::SendSessionChatRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Handle the InterruptSession RPC.
    fn interrupt_session(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::InterruptSessionRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Handle the SwitchAgent RPC.
    fn switch_agent(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::SwitchAgentRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Handle the UploadFile RPC.
    fn upload_file(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::UploadFileRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::UploadFileResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the GetSessionContext RPC.
    fn get_session_context(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::GetSessionContextRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::SessionContextResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the CompactSession RPC.
    fn compact_session(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::CompactSessionRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::CompactSessionResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the GetHistoryMessages RPC.
    fn get_history_messages(
        &self,
        ctx: Context,
        request: OwnedView<
            crate::proto::openzerg::GetHistoryMessagesRequestView<'static>,
        >,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::MessageListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the DeleteMessagesFrom RPC.
    fn delete_messages_from(
        &self,
        ctx: Context,
        request: OwnedView<
            crate::proto::openzerg::DeleteMessagesFromRequestView<'static>,
        >,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::DeleteMessagesFromResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Process management
    fn list_processes(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::ListProcessesRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::ProcessListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the GetProcess RPC.
    fn get_process(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::GetProcessRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::ProcessInfo, Context), ConnectError>,
    > + Send;
    /// Handle the GetProcessOutput RPC.
    fn get_process_output(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::GetProcessOutputRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::ProcessOutputResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the KillProcess RPC.
    fn kill_process(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::KillProcessRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Task management
    fn list_tasks(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::ListTasksRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::TaskListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the GetTask RPC.
    fn get_task(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::GetTaskRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::TaskInfo, Context), ConnectError>,
    > + Send;
    /// Messages
    fn send_message(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::SendMessageRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Handle the SendRemind RPC.
    fn send_remind(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::SendRemindRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Builtin tools
    fn list_builtin_tools(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::EmptyView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::BuiltinToolListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the ExecuteTool RPC.
    fn execute_tool(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::ExecuteToolRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::ExecuteToolResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// External tools
    fn list_external_tools(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::EmptyView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::ExternalToolListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the RegisterExternalTool RPC.
    fn register_external_tool(
        &self,
        ctx: Context,
        request: OwnedView<
            crate::proto::openzerg::RegisterExternalToolRequestView<'static>,
        >,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::ExternalToolInfo, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the UnregisterExternalTool RPC.
    fn unregister_external_tool(
        &self,
        ctx: Context,
        request: OwnedView<
            crate::proto::openzerg::UnregisterExternalToolRequestView<'static>,
        >,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Handle the SyncExternalTools RPC.
    fn sync_external_tools(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::SyncExternalToolsRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::ExternalToolListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the SetToolVariable RPC.
    fn set_tool_variable(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::SetToolVariableRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Providers
    fn list_providers(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::EmptyView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::ProviderListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the RegisterProvider RPC.
    fn register_provider(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::RegisterProviderRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::ProviderInfo, Context), ConnectError>,
    > + Send;
    /// Handle the UpdateProvider RPC.
    fn update_provider(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::UpdateProviderRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::ProviderInfo, Context), ConnectError>,
    > + Send;
    /// Handle the UnregisterProvider RPC.
    fn unregister_provider(
        &self,
        ctx: Context,
        request: OwnedView<
            crate::proto::openzerg::UnregisterExternalToolRequestView<'static>,
        >,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Handle the SetDefaultProvider RPC.
    fn set_default_provider(
        &self,
        ctx: Context,
        request: OwnedView<
            crate::proto::openzerg::SetDefaultProviderRequestView<'static>,
        >,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Handle the TestProviderConnection RPC.
    fn test_provider_connection(
        &self,
        ctx: Context,
        request: OwnedView<
            crate::proto::openzerg::TestProviderConnectionRequestView<'static>,
        >,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::TestProviderConnectionResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Session Provider
    fn set_session_provider(
        &self,
        ctx: Context,
        request: OwnedView<
            crate::proto::openzerg::SetSessionProviderRequestView<'static>,
        >,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Skill Registries
    fn list_registries(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::EmptyView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::RegistryListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the AddRegistry RPC.
    fn add_registry(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::AddRegistryRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::RegistryInfo, Context), ConnectError>,
    > + Send;
    /// Handle the RemoveRegistry RPC.
    fn remove_registry(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::RemoveRegistryRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Skills
    fn list_installed_skills(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::EmptyView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::SkillListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the ListRemoteSkills RPC.
    fn list_remote_skills(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::ListRemoteSkillsRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::SkillListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the InstallSkill RPC.
    fn install_skill(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::InstallSkillRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::SkillInfo, Context), ConnectError>,
    > + Send;
    /// Handle the UninstallSkill RPC.
    fn uninstall_skill(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::UninstallSkillRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Handle the GetSkill RPC.
    fn get_skill(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::GetSkillRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::SkillInfo, Context), ConnectError>,
    > + Send;
    /// Timers
    fn list_timers(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::ListTimersRequestView<'static>>,
    ) -> impl Future<
        Output = Result<
            (crate::proto::openzerg::TimerListResponse, Context),
            ConnectError,
        >,
    > + Send;
    /// Handle the CancelTimer RPC.
    fn cancel_timer(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::CancelTimerRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
    /// Health check
    fn check_health(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::EmptyView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::HealthResponse, Context), ConnectError>,
    > + Send;
    /// Streaming
    fn subscribe_session_events(
        &self,
        ctx: Context,
        request: OwnedView<
            crate::proto::openzerg::SubscribeSessionEventsRequestView<'static>,
        >,
    ) -> impl Future<
        Output = Result<
            (
                Pin<
                    Box<
                        dyn Stream<
                            Item = Result<
                                crate::proto::openzerg::SessionEvent,
                                ConnectError,
                            >,
                        > + Send,
                    >,
                >,
                Context,
            ),
            ConnectError,
        >,
    > + Send;
    /// Handle the SubscribeGlobalEvents RPC.
    fn subscribe_global_events(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::EmptyView<'static>>,
    ) -> impl Future<
        Output = Result<
            (
                Pin<
                    Box<
                        dyn Stream<
                            Item = Result<
                                crate::proto::openzerg::GlobalEvent,
                                ConnectError,
                            >,
                        > + Send,
                    >,
                >,
                Context,
            ),
            ConnectError,
        >,
    > + Send;
    /// Question interaction
    fn answer_question(
        &self,
        ctx: Context,
        request: OwnedView<crate::proto::openzerg::AnswerQuestionRequestView<'static>>,
    ) -> impl Future<
        Output = Result<(crate::proto::openzerg::Empty, Context), ConnectError>,
    > + Send;
}
/// Extension trait for registering a service implementation with a Router.
///
/// This trait is automatically implemented for all types that implement the service trait.
///
/// # Example
///
/// ```rust,ignore
/// use std::sync::Arc;
///
/// let service = Arc::new(MyServiceImpl);
/// let router = service.register(Router::new());
/// ```
pub trait AgentExt: Agent {
    /// Register this service implementation with a Router.
    ///
    /// Takes ownership of the `Arc<Self>` and returns a new Router with
    /// this service's methods registered.
    fn register(self: Arc<Self>, router: Router) -> Router;
}
impl<S: Agent> AgentExt for S {
    fn register(self: Arc<Self>, router: Router) -> Router {
        router
            .route_view(
                AGENT_SERVICE_NAME,
                "ListSessions",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.list_sessions(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "GetSession",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.get_session(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "CreateSession",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.create_session(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "DeleteSession",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.delete_session(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "GetSessionMessages",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.get_session_messages(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "SendSessionChat",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.send_session_chat(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "InterruptSession",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.interrupt_session(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "SwitchAgent",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.switch_agent(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "UploadFile",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.upload_file(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "GetSessionContext",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.get_session_context(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "CompactSession",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.compact_session(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "GetHistoryMessages",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.get_history_messages(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "DeleteMessagesFrom",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.delete_messages_from(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "ListProcesses",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.list_processes(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "GetProcess",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.get_process(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "GetProcessOutput",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.get_process_output(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "KillProcess",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.kill_process(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "ListTasks",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.list_tasks(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "GetTask",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.get_task(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "SendMessage",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.send_message(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "SendRemind",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.send_remind(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "ListBuiltinTools",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.list_builtin_tools(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "ExecuteTool",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.execute_tool(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "ListExternalTools",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.list_external_tools(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "RegisterExternalTool",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.register_external_tool(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "UnregisterExternalTool",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.unregister_external_tool(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "SyncExternalTools",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.sync_external_tools(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "SetToolVariable",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.set_tool_variable(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "ListProviders",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.list_providers(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "RegisterProvider",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.register_provider(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "UpdateProvider",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.update_provider(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "UnregisterProvider",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.unregister_provider(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "SetDefaultProvider",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.set_default_provider(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "TestProviderConnection",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.test_provider_connection(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "SetSessionProvider",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.set_session_provider(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "ListRegistries",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.list_registries(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "AddRegistry",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.add_registry(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "RemoveRegistry",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.remove_registry(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "ListInstalledSkills",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.list_installed_skills(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "ListRemoteSkills",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.list_remote_skills(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "InstallSkill",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.install_skill(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "UninstallSkill",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.uninstall_skill(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "GetSkill",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.get_skill(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "ListTimers",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.list_timers(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "CancelTimer",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.cancel_timer(ctx, req).await }
                    })
                },
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "CheckHealth",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.check_health(ctx, req).await }
                    })
                },
            )
            .route_view_server_stream(
                AGENT_SERVICE_NAME,
                "SubscribeSessionEvents",
                view_streaming_handler_fn({
                    let svc = Arc::clone(&self);
                    move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.subscribe_session_events(ctx, req).await }
                    }
                }),
            )
            .route_view_server_stream(
                AGENT_SERVICE_NAME,
                "SubscribeGlobalEvents",
                view_streaming_handler_fn({
                    let svc = Arc::clone(&self);
                    move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.subscribe_global_events(ctx, req).await }
                    }
                }),
            )
            .route_view(
                AGENT_SERVICE_NAME,
                "AnswerQuestion",
                {
                    let svc = Arc::clone(&self);
                    view_handler_fn(move |ctx, req| {
                        let svc = Arc::clone(&svc);
                        async move { svc.answer_question(ctx, req).await }
                    })
                },
            )
    }
}
/// Monomorphic dispatcher for `Agent`.
///
/// Unlike `.register(Router)` which type-erases each method into an `Arc<dyn ErasedHandler>` stored in a `HashMap`, this struct dispatches via a compile-time `match` on method name: no vtable, no hash lookup.
///
/// # Example
///
/// ```rust,ignore
/// use connectrpc::ConnectRpcService;
///
/// let server = AgentServer::new(MyImpl);
/// let service = ConnectRpcService::new(server);
/// // hand `service` to axum/hyper as a fallback_service
/// ```
pub struct AgentServer<T> {
    inner: Arc<T>,
}
impl<T: Agent> AgentServer<T> {
    /// Wrap a service implementation in a monomorphic dispatcher.
    pub fn new(service: T) -> Self {
        Self { inner: Arc::new(service) }
    }
    /// Wrap an already-`Arc`'d service implementation.
    pub fn from_arc(inner: Arc<T>) -> Self {
        Self { inner }
    }
}
impl<T> Clone for AgentServer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
impl<T: Agent> Dispatcher for AgentServer<T> {
    #[inline]
    fn lookup(&self, path: &str) -> Option<__crpc_codegen::MethodDescriptor> {
        let method = path.strip_prefix("openzerg.Agent/")?;
        match method {
            "ListSessions" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "GetSession" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "CreateSession" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "DeleteSession" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "GetSessionMessages" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "SendSessionChat" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "InterruptSession" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "SwitchAgent" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "UploadFile" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "GetSessionContext" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "CompactSession" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "GetHistoryMessages" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "DeleteMessagesFrom" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "ListProcesses" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "GetProcess" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "GetProcessOutput" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "KillProcess" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "ListTasks" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "GetTask" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "SendMessage" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "SendRemind" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "ListBuiltinTools" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "ExecuteTool" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "ListExternalTools" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "RegisterExternalTool" => {
                Some(__crpc_codegen::MethodDescriptor::unary(false))
            }
            "UnregisterExternalTool" => {
                Some(__crpc_codegen::MethodDescriptor::unary(false))
            }
            "SyncExternalTools" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "SetToolVariable" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "ListProviders" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "RegisterProvider" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "UpdateProvider" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "UnregisterProvider" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "SetDefaultProvider" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "TestProviderConnection" => {
                Some(__crpc_codegen::MethodDescriptor::unary(false))
            }
            "SetSessionProvider" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "ListRegistries" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "AddRegistry" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "RemoveRegistry" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "ListInstalledSkills" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "ListRemoteSkills" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "InstallSkill" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "UninstallSkill" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "GetSkill" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "ListTimers" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "CancelTimer" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "CheckHealth" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            "SubscribeSessionEvents" => {
                Some(__crpc_codegen::MethodDescriptor::server_streaming())
            }
            "SubscribeGlobalEvents" => {
                Some(__crpc_codegen::MethodDescriptor::server_streaming())
            }
            "AnswerQuestion" => Some(__crpc_codegen::MethodDescriptor::unary(false)),
            _ => None,
        }
    }
    fn call_unary(
        &self,
        path: &str,
        ctx: Context,
        request: __Bytes,
        format: __CodecFormat,
    ) -> __crpc_codegen::UnaryResult {
        let Some(method) = path.strip_prefix("openzerg.Agent/") else {
            return __crpc_codegen::unimplemented_unary(path);
        };
        let _ = (&ctx, &request, &format);
        match method {
            "ListSessions" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::ListSessionsRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.list_sessions(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "GetSession" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::GetSessionRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.get_session(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "CreateSession" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::CreateSessionRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.create_session(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "DeleteSession" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::DeleteSessionRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.delete_session(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "GetSessionMessages" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::GetSessionMessagesRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.get_session_messages(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "SendSessionChat" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::SendSessionChatRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.send_session_chat(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "InterruptSession" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::InterruptSessionRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.interrupt_session(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "SwitchAgent" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::SwitchAgentRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.switch_agent(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "UploadFile" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::UploadFileRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.upload_file(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "GetSessionContext" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::GetSessionContextRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.get_session_context(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "CompactSession" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::CompactSessionRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.compact_session(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "GetHistoryMessages" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::GetHistoryMessagesRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.get_history_messages(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "DeleteMessagesFrom" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::DeleteMessagesFromRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.delete_messages_from(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "ListProcesses" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::ListProcessesRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.list_processes(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "GetProcess" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::GetProcessRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.get_process(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "GetProcessOutput" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::GetProcessOutputRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.get_process_output(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "KillProcess" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::KillProcessRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.kill_process(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "ListTasks" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::ListTasksRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.list_tasks(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "GetTask" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::GetTaskRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.get_task(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "SendMessage" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::SendMessageRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.send_message(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "SendRemind" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::SendRemindRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.send_remind(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "ListBuiltinTools" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::EmptyView,
                    >(request, format)?;
                    let (res, ctx) = svc.list_builtin_tools(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "ExecuteTool" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::ExecuteToolRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.execute_tool(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "ListExternalTools" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::EmptyView,
                    >(request, format)?;
                    let (res, ctx) = svc.list_external_tools(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "RegisterExternalTool" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::RegisterExternalToolRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.register_external_tool(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "UnregisterExternalTool" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::UnregisterExternalToolRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.unregister_external_tool(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "SyncExternalTools" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::SyncExternalToolsRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.sync_external_tools(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "SetToolVariable" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::SetToolVariableRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.set_tool_variable(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "ListProviders" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::EmptyView,
                    >(request, format)?;
                    let (res, ctx) = svc.list_providers(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "RegisterProvider" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::RegisterProviderRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.register_provider(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "UpdateProvider" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::UpdateProviderRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.update_provider(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "UnregisterProvider" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::UnregisterExternalToolRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.unregister_provider(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "SetDefaultProvider" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::SetDefaultProviderRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.set_default_provider(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "TestProviderConnection" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::TestProviderConnectionRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.test_provider_connection(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "SetSessionProvider" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::SetSessionProviderRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.set_session_provider(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "ListRegistries" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::EmptyView,
                    >(request, format)?;
                    let (res, ctx) = svc.list_registries(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "AddRegistry" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::AddRegistryRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.add_registry(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "RemoveRegistry" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::RemoveRegistryRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.remove_registry(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "ListInstalledSkills" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::EmptyView,
                    >(request, format)?;
                    let (res, ctx) = svc.list_installed_skills(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "ListRemoteSkills" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::ListRemoteSkillsRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.list_remote_skills(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "InstallSkill" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::InstallSkillRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.install_skill(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "UninstallSkill" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::UninstallSkillRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.uninstall_skill(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "GetSkill" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::GetSkillRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.get_skill(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "ListTimers" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::ListTimersRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.list_timers(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "CancelTimer" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::CancelTimerRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.cancel_timer(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "CheckHealth" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::EmptyView,
                    >(request, format)?;
                    let (res, ctx) = svc.check_health(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            "AnswerQuestion" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::AnswerQuestionRequestView,
                    >(request, format)?;
                    let (res, ctx) = svc.answer_question(ctx, req).await?;
                    let bytes = __crpc_codegen::encode_response(&res, format)?;
                    Ok((bytes, ctx))
                })
            }
            _ => __crpc_codegen::unimplemented_unary(path),
        }
    }
    fn call_server_streaming(
        &self,
        path: &str,
        ctx: Context,
        request: __Bytes,
        format: __CodecFormat,
    ) -> __crpc_codegen::StreamingResult {
        let Some(method) = path.strip_prefix("openzerg.Agent/") else {
            return __crpc_codegen::unimplemented_streaming(path);
        };
        let _ = (&ctx, &request, &format);
        match method {
            "SubscribeSessionEvents" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::SubscribeSessionEventsRequestView,
                    >(request, format)?;
                    let (resp_stream, ctx) = svc
                        .subscribe_session_events(ctx, req)
                        .await?;
                    Ok((
                        __crpc_codegen::encode_response_stream(resp_stream, format),
                        ctx,
                    ))
                })
            }
            "SubscribeGlobalEvents" => {
                let svc = Arc::clone(&self.inner);
                Box::pin(async move {
                    let req = __crpc_codegen::decode_request_view::<
                        crate::proto::openzerg::EmptyView,
                    >(request, format)?;
                    let (resp_stream, ctx) = svc
                        .subscribe_global_events(ctx, req)
                        .await?;
                    Ok((
                        __crpc_codegen::encode_response_stream(resp_stream, format),
                        ctx,
                    ))
                })
            }
            _ => __crpc_codegen::unimplemented_streaming(path),
        }
    }
    fn call_client_streaming(
        &self,
        path: &str,
        ctx: Context,
        requests: __crpc_codegen::RequestStream,
        format: __CodecFormat,
    ) -> __crpc_codegen::UnaryResult {
        let Some(method) = path.strip_prefix("openzerg.Agent/") else {
            return __crpc_codegen::unimplemented_unary(path);
        };
        let _ = (&ctx, &requests, &format);
        match method {
            _ => __crpc_codegen::unimplemented_unary(path),
        }
    }
    fn call_bidi_streaming(
        &self,
        path: &str,
        ctx: Context,
        requests: __crpc_codegen::RequestStream,
        format: __CodecFormat,
    ) -> __crpc_codegen::StreamingResult {
        let Some(method) = path.strip_prefix("openzerg.Agent/") else {
            return __crpc_codegen::unimplemented_streaming(path);
        };
        let _ = (&ctx, &requests, &format);
        match method {
            _ => __crpc_codegen::unimplemented_streaming(path),
        }
    }
}
/// Client for this service.
///
/// Generic over `T: ClientTransport`. For **gRPC** (HTTP/2), use
/// `Http2Connection` — it has honest `poll_ready` and composes with
/// `tower::balance` for multi-connection load balancing. For **Connect
/// over HTTP/1.1** (or unknown protocol), use `HttpClient`.
///
/// # Example (gRPC / HTTP/2)
///
/// ```rust,ignore
/// use connectrpc::client::{Http2Connection, ClientConfig};
/// use connectrpc::Protocol;
///
/// let uri: http::Uri = "http://localhost:8080".parse()?;
/// let conn = Http2Connection::connect_plaintext(uri.clone()).await?.shared(1024);
/// let config = ClientConfig::new(uri).protocol(Protocol::Grpc);
///
/// let client = AgentClient::new(conn, config);
/// let response = client.list_sessions(request).await?;
/// ```
///
/// # Example (Connect / HTTP/1.1 or ALPN)
///
/// ```rust,ignore
/// use connectrpc::client::{HttpClient, ClientConfig};
///
/// let http = HttpClient::plaintext();  // cleartext http:// only
/// let config = ClientConfig::new("http://localhost:8080".parse()?);
///
/// let client = AgentClient::new(http, config);
/// let response = client.list_sessions(request).await?;
/// ```
///
/// # Working with the response
///
/// Unary calls return [`UnaryResponse<OwnedView<FooView>>`](::connectrpc::client::UnaryResponse).
/// The `OwnedView` derefs to the view, so field access is zero-copy:
///
/// ```rust,ignore
/// let resp = client.list_sessions(request).await?.into_view();
/// let name: &str = resp.name;  // borrow into the response buffer
/// ```
///
/// If you need the owned struct (e.g. to store or pass by value), use
/// [`into_owned()`](::connectrpc::client::UnaryResponse::into_owned):
///
/// ```rust,ignore
/// let owned = client.list_sessions(request).await?.into_owned();
/// ```
#[derive(Clone)]
pub struct AgentClient<T> {
    transport: T,
    config: ClientConfig,
}
impl<T> AgentClient<T>
where
    T: ClientTransport,
    <T::ResponseBody as http_body::Body>::Error: std::fmt::Display,
{
    /// Create a new client with the given transport and configuration.
    pub fn new(transport: T, config: ClientConfig) -> Self {
        Self { transport, config }
    }
    /// Get the client configuration.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }
    /// Get a mutable reference to the client configuration.
    pub fn config_mut(&mut self) -> &mut ClientConfig {
        &mut self.config
    }
    /// Call the ListSessions RPC. Sends a request to /openzerg.Agent/ListSessions.
    pub async fn list_sessions(
        &self,
        request: crate::proto::openzerg::ListSessionsRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SessionListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.list_sessions_with_options(request, CallOptions::default()).await
    }
    /// Call the ListSessions RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn list_sessions_with_options(
        &self,
        request: crate::proto::openzerg::ListSessionsRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SessionListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ListSessions",
                request,
                options,
            )
            .await
    }
    /// Call the GetSession RPC. Sends a request to /openzerg.Agent/GetSession.
    pub async fn get_session(
        &self,
        request: crate::proto::openzerg::GetSessionRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SessionInfoView<'static>>,
        >,
        ConnectError,
    > {
        self.get_session_with_options(request, CallOptions::default()).await
    }
    /// Call the GetSession RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn get_session_with_options(
        &self,
        request: crate::proto::openzerg::GetSessionRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SessionInfoView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "GetSession",
                request,
                options,
            )
            .await
    }
    /// Call the CreateSession RPC. Sends a request to /openzerg.Agent/CreateSession.
    pub async fn create_session(
        &self,
        request: crate::proto::openzerg::CreateSessionRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SessionInfoView<'static>>,
        >,
        ConnectError,
    > {
        self.create_session_with_options(request, CallOptions::default()).await
    }
    /// Call the CreateSession RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn create_session_with_options(
        &self,
        request: crate::proto::openzerg::CreateSessionRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SessionInfoView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "CreateSession",
                request,
                options,
            )
            .await
    }
    /// Call the DeleteSession RPC. Sends a request to /openzerg.Agent/DeleteSession.
    pub async fn delete_session(
        &self,
        request: crate::proto::openzerg::DeleteSessionRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.delete_session_with_options(request, CallOptions::default()).await
    }
    /// Call the DeleteSession RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn delete_session_with_options(
        &self,
        request: crate::proto::openzerg::DeleteSessionRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "DeleteSession",
                request,
                options,
            )
            .await
    }
    /// Call the GetSessionMessages RPC. Sends a request to /openzerg.Agent/GetSessionMessages.
    pub async fn get_session_messages(
        &self,
        request: crate::proto::openzerg::GetSessionMessagesRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::MessageListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.get_session_messages_with_options(request, CallOptions::default()).await
    }
    /// Call the GetSessionMessages RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn get_session_messages_with_options(
        &self,
        request: crate::proto::openzerg::GetSessionMessagesRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::MessageListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "GetSessionMessages",
                request,
                options,
            )
            .await
    }
    /// Call the SendSessionChat RPC. Sends a request to /openzerg.Agent/SendSessionChat.
    pub async fn send_session_chat(
        &self,
        request: crate::proto::openzerg::SendSessionChatRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.send_session_chat_with_options(request, CallOptions::default()).await
    }
    /// Call the SendSessionChat RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn send_session_chat_with_options(
        &self,
        request: crate::proto::openzerg::SendSessionChatRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "SendSessionChat",
                request,
                options,
            )
            .await
    }
    /// Call the InterruptSession RPC. Sends a request to /openzerg.Agent/InterruptSession.
    pub async fn interrupt_session(
        &self,
        request: crate::proto::openzerg::InterruptSessionRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.interrupt_session_with_options(request, CallOptions::default()).await
    }
    /// Call the InterruptSession RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn interrupt_session_with_options(
        &self,
        request: crate::proto::openzerg::InterruptSessionRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "InterruptSession",
                request,
                options,
            )
            .await
    }
    /// Call the SwitchAgent RPC. Sends a request to /openzerg.Agent/SwitchAgent.
    pub async fn switch_agent(
        &self,
        request: crate::proto::openzerg::SwitchAgentRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.switch_agent_with_options(request, CallOptions::default()).await
    }
    /// Call the SwitchAgent RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn switch_agent_with_options(
        &self,
        request: crate::proto::openzerg::SwitchAgentRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "SwitchAgent",
                request,
                options,
            )
            .await
    }
    /// Call the UploadFile RPC. Sends a request to /openzerg.Agent/UploadFile.
    pub async fn upload_file(
        &self,
        request: crate::proto::openzerg::UploadFileRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::UploadFileResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.upload_file_with_options(request, CallOptions::default()).await
    }
    /// Call the UploadFile RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn upload_file_with_options(
        &self,
        request: crate::proto::openzerg::UploadFileRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::UploadFileResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "UploadFile",
                request,
                options,
            )
            .await
    }
    /// Call the GetSessionContext RPC. Sends a request to /openzerg.Agent/GetSessionContext.
    pub async fn get_session_context(
        &self,
        request: crate::proto::openzerg::GetSessionContextRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SessionContextResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.get_session_context_with_options(request, CallOptions::default()).await
    }
    /// Call the GetSessionContext RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn get_session_context_with_options(
        &self,
        request: crate::proto::openzerg::GetSessionContextRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SessionContextResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "GetSessionContext",
                request,
                options,
            )
            .await
    }
    /// Call the CompactSession RPC. Sends a request to /openzerg.Agent/CompactSession.
    pub async fn compact_session(
        &self,
        request: crate::proto::openzerg::CompactSessionRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::CompactSessionResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.compact_session_with_options(request, CallOptions::default()).await
    }
    /// Call the CompactSession RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn compact_session_with_options(
        &self,
        request: crate::proto::openzerg::CompactSessionRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::CompactSessionResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "CompactSession",
                request,
                options,
            )
            .await
    }
    /// Call the GetHistoryMessages RPC. Sends a request to /openzerg.Agent/GetHistoryMessages.
    pub async fn get_history_messages(
        &self,
        request: crate::proto::openzerg::GetHistoryMessagesRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::MessageListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.get_history_messages_with_options(request, CallOptions::default()).await
    }
    /// Call the GetHistoryMessages RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn get_history_messages_with_options(
        &self,
        request: crate::proto::openzerg::GetHistoryMessagesRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::MessageListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "GetHistoryMessages",
                request,
                options,
            )
            .await
    }
    /// Call the DeleteMessagesFrom RPC. Sends a request to /openzerg.Agent/DeleteMessagesFrom.
    pub async fn delete_messages_from(
        &self,
        request: crate::proto::openzerg::DeleteMessagesFromRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::DeleteMessagesFromResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.delete_messages_from_with_options(request, CallOptions::default()).await
    }
    /// Call the DeleteMessagesFrom RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn delete_messages_from_with_options(
        &self,
        request: crate::proto::openzerg::DeleteMessagesFromRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::DeleteMessagesFromResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "DeleteMessagesFrom",
                request,
                options,
            )
            .await
    }
    /// Call the ListProcesses RPC. Sends a request to /openzerg.Agent/ListProcesses.
    pub async fn list_processes(
        &self,
        request: crate::proto::openzerg::ListProcessesRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProcessListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.list_processes_with_options(request, CallOptions::default()).await
    }
    /// Call the ListProcesses RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn list_processes_with_options(
        &self,
        request: crate::proto::openzerg::ListProcessesRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProcessListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ListProcesses",
                request,
                options,
            )
            .await
    }
    /// Call the GetProcess RPC. Sends a request to /openzerg.Agent/GetProcess.
    pub async fn get_process(
        &self,
        request: crate::proto::openzerg::GetProcessRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProcessInfoView<'static>>,
        >,
        ConnectError,
    > {
        self.get_process_with_options(request, CallOptions::default()).await
    }
    /// Call the GetProcess RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn get_process_with_options(
        &self,
        request: crate::proto::openzerg::GetProcessRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProcessInfoView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "GetProcess",
                request,
                options,
            )
            .await
    }
    /// Call the GetProcessOutput RPC. Sends a request to /openzerg.Agent/GetProcessOutput.
    pub async fn get_process_output(
        &self,
        request: crate::proto::openzerg::GetProcessOutputRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProcessOutputResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.get_process_output_with_options(request, CallOptions::default()).await
    }
    /// Call the GetProcessOutput RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn get_process_output_with_options(
        &self,
        request: crate::proto::openzerg::GetProcessOutputRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProcessOutputResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "GetProcessOutput",
                request,
                options,
            )
            .await
    }
    /// Call the KillProcess RPC. Sends a request to /openzerg.Agent/KillProcess.
    pub async fn kill_process(
        &self,
        request: crate::proto::openzerg::KillProcessRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.kill_process_with_options(request, CallOptions::default()).await
    }
    /// Call the KillProcess RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn kill_process_with_options(
        &self,
        request: crate::proto::openzerg::KillProcessRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "KillProcess",
                request,
                options,
            )
            .await
    }
    /// Call the ListTasks RPC. Sends a request to /openzerg.Agent/ListTasks.
    pub async fn list_tasks(
        &self,
        request: crate::proto::openzerg::ListTasksRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::TaskListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.list_tasks_with_options(request, CallOptions::default()).await
    }
    /// Call the ListTasks RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn list_tasks_with_options(
        &self,
        request: crate::proto::openzerg::ListTasksRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::TaskListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ListTasks",
                request,
                options,
            )
            .await
    }
    /// Call the GetTask RPC. Sends a request to /openzerg.Agent/GetTask.
    pub async fn get_task(
        &self,
        request: crate::proto::openzerg::GetTaskRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::TaskInfoView<'static>>,
        >,
        ConnectError,
    > {
        self.get_task_with_options(request, CallOptions::default()).await
    }
    /// Call the GetTask RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn get_task_with_options(
        &self,
        request: crate::proto::openzerg::GetTaskRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::TaskInfoView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "GetTask",
                request,
                options,
            )
            .await
    }
    /// Call the SendMessage RPC. Sends a request to /openzerg.Agent/SendMessage.
    pub async fn send_message(
        &self,
        request: crate::proto::openzerg::SendMessageRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.send_message_with_options(request, CallOptions::default()).await
    }
    /// Call the SendMessage RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn send_message_with_options(
        &self,
        request: crate::proto::openzerg::SendMessageRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "SendMessage",
                request,
                options,
            )
            .await
    }
    /// Call the SendRemind RPC. Sends a request to /openzerg.Agent/SendRemind.
    pub async fn send_remind(
        &self,
        request: crate::proto::openzerg::SendRemindRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.send_remind_with_options(request, CallOptions::default()).await
    }
    /// Call the SendRemind RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn send_remind_with_options(
        &self,
        request: crate::proto::openzerg::SendRemindRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "SendRemind",
                request,
                options,
            )
            .await
    }
    /// Call the ListBuiltinTools RPC. Sends a request to /openzerg.Agent/ListBuiltinTools.
    pub async fn list_builtin_tools(
        &self,
        request: crate::proto::openzerg::Empty,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::BuiltinToolListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.list_builtin_tools_with_options(request, CallOptions::default()).await
    }
    /// Call the ListBuiltinTools RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn list_builtin_tools_with_options(
        &self,
        request: crate::proto::openzerg::Empty,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::BuiltinToolListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ListBuiltinTools",
                request,
                options,
            )
            .await
    }
    /// Call the ExecuteTool RPC. Sends a request to /openzerg.Agent/ExecuteTool.
    pub async fn execute_tool(
        &self,
        request: crate::proto::openzerg::ExecuteToolRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ExecuteToolResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.execute_tool_with_options(request, CallOptions::default()).await
    }
    /// Call the ExecuteTool RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn execute_tool_with_options(
        &self,
        request: crate::proto::openzerg::ExecuteToolRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ExecuteToolResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ExecuteTool",
                request,
                options,
            )
            .await
    }
    /// Call the ListExternalTools RPC. Sends a request to /openzerg.Agent/ListExternalTools.
    pub async fn list_external_tools(
        &self,
        request: crate::proto::openzerg::Empty,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ExternalToolListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.list_external_tools_with_options(request, CallOptions::default()).await
    }
    /// Call the ListExternalTools RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn list_external_tools_with_options(
        &self,
        request: crate::proto::openzerg::Empty,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ExternalToolListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ListExternalTools",
                request,
                options,
            )
            .await
    }
    /// Call the RegisterExternalTool RPC. Sends a request to /openzerg.Agent/RegisterExternalTool.
    pub async fn register_external_tool(
        &self,
        request: crate::proto::openzerg::RegisterExternalToolRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ExternalToolInfoView<'static>>,
        >,
        ConnectError,
    > {
        self.register_external_tool_with_options(request, CallOptions::default()).await
    }
    /// Call the RegisterExternalTool RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn register_external_tool_with_options(
        &self,
        request: crate::proto::openzerg::RegisterExternalToolRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ExternalToolInfoView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "RegisterExternalTool",
                request,
                options,
            )
            .await
    }
    /// Call the UnregisterExternalTool RPC. Sends a request to /openzerg.Agent/UnregisterExternalTool.
    pub async fn unregister_external_tool(
        &self,
        request: crate::proto::openzerg::UnregisterExternalToolRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.unregister_external_tool_with_options(request, CallOptions::default()).await
    }
    /// Call the UnregisterExternalTool RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn unregister_external_tool_with_options(
        &self,
        request: crate::proto::openzerg::UnregisterExternalToolRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "UnregisterExternalTool",
                request,
                options,
            )
            .await
    }
    /// Call the SyncExternalTools RPC. Sends a request to /openzerg.Agent/SyncExternalTools.
    pub async fn sync_external_tools(
        &self,
        request: crate::proto::openzerg::SyncExternalToolsRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ExternalToolListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.sync_external_tools_with_options(request, CallOptions::default()).await
    }
    /// Call the SyncExternalTools RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn sync_external_tools_with_options(
        &self,
        request: crate::proto::openzerg::SyncExternalToolsRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ExternalToolListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "SyncExternalTools",
                request,
                options,
            )
            .await
    }
    /// Call the SetToolVariable RPC. Sends a request to /openzerg.Agent/SetToolVariable.
    pub async fn set_tool_variable(
        &self,
        request: crate::proto::openzerg::SetToolVariableRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.set_tool_variable_with_options(request, CallOptions::default()).await
    }
    /// Call the SetToolVariable RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn set_tool_variable_with_options(
        &self,
        request: crate::proto::openzerg::SetToolVariableRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "SetToolVariable",
                request,
                options,
            )
            .await
    }
    /// Call the ListProviders RPC. Sends a request to /openzerg.Agent/ListProviders.
    pub async fn list_providers(
        &self,
        request: crate::proto::openzerg::Empty,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProviderListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.list_providers_with_options(request, CallOptions::default()).await
    }
    /// Call the ListProviders RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn list_providers_with_options(
        &self,
        request: crate::proto::openzerg::Empty,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProviderListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ListProviders",
                request,
                options,
            )
            .await
    }
    /// Call the RegisterProvider RPC. Sends a request to /openzerg.Agent/RegisterProvider.
    pub async fn register_provider(
        &self,
        request: crate::proto::openzerg::RegisterProviderRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProviderInfoView<'static>>,
        >,
        ConnectError,
    > {
        self.register_provider_with_options(request, CallOptions::default()).await
    }
    /// Call the RegisterProvider RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn register_provider_with_options(
        &self,
        request: crate::proto::openzerg::RegisterProviderRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProviderInfoView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "RegisterProvider",
                request,
                options,
            )
            .await
    }
    /// Call the UpdateProvider RPC. Sends a request to /openzerg.Agent/UpdateProvider.
    pub async fn update_provider(
        &self,
        request: crate::proto::openzerg::UpdateProviderRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProviderInfoView<'static>>,
        >,
        ConnectError,
    > {
        self.update_provider_with_options(request, CallOptions::default()).await
    }
    /// Call the UpdateProvider RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn update_provider_with_options(
        &self,
        request: crate::proto::openzerg::UpdateProviderRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::ProviderInfoView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "UpdateProvider",
                request,
                options,
            )
            .await
    }
    /// Call the UnregisterProvider RPC. Sends a request to /openzerg.Agent/UnregisterProvider.
    pub async fn unregister_provider(
        &self,
        request: crate::proto::openzerg::UnregisterExternalToolRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.unregister_provider_with_options(request, CallOptions::default()).await
    }
    /// Call the UnregisterProvider RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn unregister_provider_with_options(
        &self,
        request: crate::proto::openzerg::UnregisterExternalToolRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "UnregisterProvider",
                request,
                options,
            )
            .await
    }
    /// Call the SetDefaultProvider RPC. Sends a request to /openzerg.Agent/SetDefaultProvider.
    pub async fn set_default_provider(
        &self,
        request: crate::proto::openzerg::SetDefaultProviderRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.set_default_provider_with_options(request, CallOptions::default()).await
    }
    /// Call the SetDefaultProvider RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn set_default_provider_with_options(
        &self,
        request: crate::proto::openzerg::SetDefaultProviderRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "SetDefaultProvider",
                request,
                options,
            )
            .await
    }
    /// Call the TestProviderConnection RPC. Sends a request to /openzerg.Agent/TestProviderConnection.
    pub async fn test_provider_connection(
        &self,
        request: crate::proto::openzerg::TestProviderConnectionRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<
                crate::proto::openzerg::TestProviderConnectionResponseView<'static>,
            >,
        >,
        ConnectError,
    > {
        self.test_provider_connection_with_options(request, CallOptions::default()).await
    }
    /// Call the TestProviderConnection RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn test_provider_connection_with_options(
        &self,
        request: crate::proto::openzerg::TestProviderConnectionRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<
                crate::proto::openzerg::TestProviderConnectionResponseView<'static>,
            >,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "TestProviderConnection",
                request,
                options,
            )
            .await
    }
    /// Call the SetSessionProvider RPC. Sends a request to /openzerg.Agent/SetSessionProvider.
    pub async fn set_session_provider(
        &self,
        request: crate::proto::openzerg::SetSessionProviderRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.set_session_provider_with_options(request, CallOptions::default()).await
    }
    /// Call the SetSessionProvider RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn set_session_provider_with_options(
        &self,
        request: crate::proto::openzerg::SetSessionProviderRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "SetSessionProvider",
                request,
                options,
            )
            .await
    }
    /// Call the ListRegistries RPC. Sends a request to /openzerg.Agent/ListRegistries.
    pub async fn list_registries(
        &self,
        request: crate::proto::openzerg::Empty,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::RegistryListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.list_registries_with_options(request, CallOptions::default()).await
    }
    /// Call the ListRegistries RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn list_registries_with_options(
        &self,
        request: crate::proto::openzerg::Empty,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::RegistryListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ListRegistries",
                request,
                options,
            )
            .await
    }
    /// Call the AddRegistry RPC. Sends a request to /openzerg.Agent/AddRegistry.
    pub async fn add_registry(
        &self,
        request: crate::proto::openzerg::AddRegistryRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::RegistryInfoView<'static>>,
        >,
        ConnectError,
    > {
        self.add_registry_with_options(request, CallOptions::default()).await
    }
    /// Call the AddRegistry RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn add_registry_with_options(
        &self,
        request: crate::proto::openzerg::AddRegistryRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::RegistryInfoView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "AddRegistry",
                request,
                options,
            )
            .await
    }
    /// Call the RemoveRegistry RPC. Sends a request to /openzerg.Agent/RemoveRegistry.
    pub async fn remove_registry(
        &self,
        request: crate::proto::openzerg::RemoveRegistryRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.remove_registry_with_options(request, CallOptions::default()).await
    }
    /// Call the RemoveRegistry RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn remove_registry_with_options(
        &self,
        request: crate::proto::openzerg::RemoveRegistryRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "RemoveRegistry",
                request,
                options,
            )
            .await
    }
    /// Call the ListInstalledSkills RPC. Sends a request to /openzerg.Agent/ListInstalledSkills.
    pub async fn list_installed_skills(
        &self,
        request: crate::proto::openzerg::Empty,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SkillListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.list_installed_skills_with_options(request, CallOptions::default()).await
    }
    /// Call the ListInstalledSkills RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn list_installed_skills_with_options(
        &self,
        request: crate::proto::openzerg::Empty,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SkillListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ListInstalledSkills",
                request,
                options,
            )
            .await
    }
    /// Call the ListRemoteSkills RPC. Sends a request to /openzerg.Agent/ListRemoteSkills.
    pub async fn list_remote_skills(
        &self,
        request: crate::proto::openzerg::ListRemoteSkillsRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SkillListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.list_remote_skills_with_options(request, CallOptions::default()).await
    }
    /// Call the ListRemoteSkills RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn list_remote_skills_with_options(
        &self,
        request: crate::proto::openzerg::ListRemoteSkillsRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SkillListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ListRemoteSkills",
                request,
                options,
            )
            .await
    }
    /// Call the InstallSkill RPC. Sends a request to /openzerg.Agent/InstallSkill.
    pub async fn install_skill(
        &self,
        request: crate::proto::openzerg::InstallSkillRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SkillInfoView<'static>>,
        >,
        ConnectError,
    > {
        self.install_skill_with_options(request, CallOptions::default()).await
    }
    /// Call the InstallSkill RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn install_skill_with_options(
        &self,
        request: crate::proto::openzerg::InstallSkillRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SkillInfoView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "InstallSkill",
                request,
                options,
            )
            .await
    }
    /// Call the UninstallSkill RPC. Sends a request to /openzerg.Agent/UninstallSkill.
    pub async fn uninstall_skill(
        &self,
        request: crate::proto::openzerg::UninstallSkillRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.uninstall_skill_with_options(request, CallOptions::default()).await
    }
    /// Call the UninstallSkill RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn uninstall_skill_with_options(
        &self,
        request: crate::proto::openzerg::UninstallSkillRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "UninstallSkill",
                request,
                options,
            )
            .await
    }
    /// Call the GetSkill RPC. Sends a request to /openzerg.Agent/GetSkill.
    pub async fn get_skill(
        &self,
        request: crate::proto::openzerg::GetSkillRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SkillInfoView<'static>>,
        >,
        ConnectError,
    > {
        self.get_skill_with_options(request, CallOptions::default()).await
    }
    /// Call the GetSkill RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn get_skill_with_options(
        &self,
        request: crate::proto::openzerg::GetSkillRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::SkillInfoView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "GetSkill",
                request,
                options,
            )
            .await
    }
    /// Call the ListTimers RPC. Sends a request to /openzerg.Agent/ListTimers.
    pub async fn list_timers(
        &self,
        request: crate::proto::openzerg::ListTimersRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::TimerListResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.list_timers_with_options(request, CallOptions::default()).await
    }
    /// Call the ListTimers RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn list_timers_with_options(
        &self,
        request: crate::proto::openzerg::ListTimersRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::TimerListResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "ListTimers",
                request,
                options,
            )
            .await
    }
    /// Call the CancelTimer RPC. Sends a request to /openzerg.Agent/CancelTimer.
    pub async fn cancel_timer(
        &self,
        request: crate::proto::openzerg::CancelTimerRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.cancel_timer_with_options(request, CallOptions::default()).await
    }
    /// Call the CancelTimer RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn cancel_timer_with_options(
        &self,
        request: crate::proto::openzerg::CancelTimerRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "CancelTimer",
                request,
                options,
            )
            .await
    }
    /// Call the CheckHealth RPC. Sends a request to /openzerg.Agent/CheckHealth.
    pub async fn check_health(
        &self,
        request: crate::proto::openzerg::Empty,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::HealthResponseView<'static>>,
        >,
        ConnectError,
    > {
        self.check_health_with_options(request, CallOptions::default()).await
    }
    /// Call the CheckHealth RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn check_health_with_options(
        &self,
        request: crate::proto::openzerg::Empty,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::HealthResponseView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "CheckHealth",
                request,
                options,
            )
            .await
    }
    /// Call the SubscribeSessionEvents RPC. Sends a request to /openzerg.Agent/SubscribeSessionEvents.
    pub async fn subscribe_session_events(
        &self,
        request: crate::proto::openzerg::SubscribeSessionEventsRequest,
    ) -> Result<
        ::connectrpc::client::ServerStream<
            T::ResponseBody,
            crate::proto::openzerg::SessionEventView<'static>,
        >,
        ConnectError,
    > {
        self.subscribe_session_events_with_options(request, CallOptions::default()).await
    }
    /// Call the SubscribeSessionEvents RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn subscribe_session_events_with_options(
        &self,
        request: crate::proto::openzerg::SubscribeSessionEventsRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::ServerStream<
            T::ResponseBody,
            crate::proto::openzerg::SessionEventView<'static>,
        >,
        ConnectError,
    > {
        call_server_stream(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "SubscribeSessionEvents",
                request,
                options,
            )
            .await
    }
    /// Call the SubscribeGlobalEvents RPC. Sends a request to /openzerg.Agent/SubscribeGlobalEvents.
    pub async fn subscribe_global_events(
        &self,
        request: crate::proto::openzerg::Empty,
    ) -> Result<
        ::connectrpc::client::ServerStream<
            T::ResponseBody,
            crate::proto::openzerg::GlobalEventView<'static>,
        >,
        ConnectError,
    > {
        self.subscribe_global_events_with_options(request, CallOptions::default()).await
    }
    /// Call the SubscribeGlobalEvents RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn subscribe_global_events_with_options(
        &self,
        request: crate::proto::openzerg::Empty,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::ServerStream<
            T::ResponseBody,
            crate::proto::openzerg::GlobalEventView<'static>,
        >,
        ConnectError,
    > {
        call_server_stream(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "SubscribeGlobalEvents",
                request,
                options,
            )
            .await
    }
    /// Call the AnswerQuestion RPC. Sends a request to /openzerg.Agent/AnswerQuestion.
    pub async fn answer_question(
        &self,
        request: crate::proto::openzerg::AnswerQuestionRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        self.answer_question_with_options(request, CallOptions::default()).await
    }
    /// Call the AnswerQuestion RPC with explicit per-call options. Options override [`ClientConfig`] defaults.
    pub async fn answer_question_with_options(
        &self,
        request: crate::proto::openzerg::AnswerQuestionRequest,
        options: CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            OwnedView<crate::proto::openzerg::EmptyView<'static>>,
        >,
        ConnectError,
    > {
        call_unary(
                &self.transport,
                &self.config,
                "openzerg.Agent",
                "AnswerQuestion",
                request,
                options,
            )
            .await
    }
}
