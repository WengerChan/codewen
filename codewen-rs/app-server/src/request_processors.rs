use crate::bespoke_event_handling::apply_bespoke_event_handling;
use crate::command_exec::CommandExecManager;
use crate::command_exec::StartCommandExecParams;
use crate::config_manager::ConfigManager;
use crate::error_code::INPUT_TOO_LARGE_ERROR_CODE;
use crate::error_code::invalid_params;
use crate::models::supported_models;
use crate::outgoing_message::ConnectionId;
use crate::outgoing_message::ConnectionRequestId;
use crate::outgoing_message::OutgoingMessageSender;
use crate::outgoing_message::RequestContext;
use crate::outgoing_message::ThreadScopedOutgoingMessageSender;
use crate::skills_watcher::SkillsWatcher;
use crate::thread_status::ThreadWatchManager;
use crate::thread_status::resolve_thread_status;
use chrono::Duration as ChronoDuration;
use chrono::SecondsFormat;
use codewen_analytics::AnalyticsEventsClient;
use codewen_analytics::AnalyticsJsonRpcError;
use codewen_analytics::InputError;
use codewen_analytics::TurnSteerRequestError;
use codewen_app_server_protocol::Account;
use codewen_app_server_protocol::AccountLoginCompletedNotification;
use codewen_app_server_protocol::AccountTokenUsageDailyBucket;
use codewen_app_server_protocol::AccountTokenUsageSummary;
use codewen_app_server_protocol::AccountUpdatedNotification;
use codewen_app_server_protocol::AddCreditsNudgeCreditType;
use codewen_app_server_protocol::AddCreditsNudgeEmailStatus;
use codewen_app_server_protocol::AdditionalContextEntry;
use codewen_app_server_protocol::AdditionalContextKind;
use codewen_app_server_protocol::AppListUpdatedNotification;
use codewen_app_server_protocol::AppSummary;
use codewen_app_server_protocol::AppTemplateSummary;
use codewen_app_server_protocol::AppTemplateUnavailableReason;
use codewen_app_server_protocol::AppsInstalledParams;
use codewen_app_server_protocol::AppsInstalledResponse;
use codewen_app_server_protocol::AppsListParams;
use codewen_app_server_protocol::AppsListResponse;
use codewen_app_server_protocol::AppsReadParams;
use codewen_app_server_protocol::AppsReadResponse;
use codewen_app_server_protocol::AskForApproval;
use codewen_app_server_protocol::AuthMode;
use codewen_app_server_protocol::CancelLoginAccountParams;
use codewen_app_server_protocol::CancelLoginAccountResponse;
use codewen_app_server_protocol::CancelLoginAccountStatus;
use codewen_app_server_protocol::ClientInfo;
use codewen_app_server_protocol::ClientRequest;
use codewen_app_server_protocol::ClientResponsePayload;
use codewen_app_server_protocol::CodewenErrorInfo;
use codewen_app_server_protocol::CollaborationModeListParams;
use codewen_app_server_protocol::CollaborationModeListResponse;
use codewen_app_server_protocol::CommandExecParams;
use codewen_app_server_protocol::CommandExecResizeParams;
use codewen_app_server_protocol::CommandExecTerminateParams;
use codewen_app_server_protocol::CommandExecWriteParams;
use codewen_app_server_protocol::ConfigWarningNotification;
use codewen_app_server_protocol::ConsumeAccountRateLimitResetCreditOutcome;
use codewen_app_server_protocol::ConsumeAccountRateLimitResetCreditParams;
use codewen_app_server_protocol::ConsumeAccountRateLimitResetCreditResponse;
use codewen_app_server_protocol::ConversationGitInfo;
use codewen_app_server_protocol::ConversationSummary;
use codewen_app_server_protocol::DeprecationNoticeNotification;
use codewen_app_server_protocol::DynamicToolFunctionSpec;
use codewen_app_server_protocol::DynamicToolNamespaceTool;
use codewen_app_server_protocol::DynamicToolSpec;
use codewen_app_server_protocol::EnvironmentAddParams;
use codewen_app_server_protocol::EnvironmentAddResponse;
use codewen_app_server_protocol::EnvironmentInfoParams;
use codewen_app_server_protocol::EnvironmentInfoResponse;
use codewen_app_server_protocol::EnvironmentShellInfo;
use codewen_app_server_protocol::EnvironmentStatusKind;
use codewen_app_server_protocol::EnvironmentStatusParams;
use codewen_app_server_protocol::EnvironmentStatusResponse;
use codewen_app_server_protocol::ExperimentalFeature as ApiExperimentalFeature;
use codewen_app_server_protocol::ExperimentalFeatureListParams;
use codewen_app_server_protocol::ExperimentalFeatureListResponse;
use codewen_app_server_protocol::ExperimentalFeatureStage as ApiExperimentalFeatureStage;
use codewen_app_server_protocol::FeedbackUploadParams;
use codewen_app_server_protocol::FeedbackUploadResponse;
use codewen_app_server_protocol::GetAccountParams;
use codewen_app_server_protocol::GetAccountRateLimitsResponse;
use codewen_app_server_protocol::GetAccountResponse;
use codewen_app_server_protocol::GetAccountTokenUsageResponse;
use codewen_app_server_protocol::GetAuthStatusParams;
use codewen_app_server_protocol::GetAuthStatusResponse;
use codewen_app_server_protocol::GetConversationSummaryParams;
use codewen_app_server_protocol::GetConversationSummaryResponse;
use codewen_app_server_protocol::GetWorkspaceMessagesResponse;
use codewen_app_server_protocol::GitDiffToRemoteParams;
use codewen_app_server_protocol::GitDiffToRemoteResponse;
use codewen_app_server_protocol::GitInfo as ApiGitInfo;
use codewen_app_server_protocol::HookMetadata;
use codewen_app_server_protocol::HooksListParams;
use codewen_app_server_protocol::HooksListResponse;
use codewen_app_server_protocol::InitializeParams;
use codewen_app_server_protocol::InitializeResponse;
use codewen_app_server_protocol::InstalledApp;
use codewen_app_server_protocol::JSONRPCErrorError;
use codewen_app_server_protocol::ListMcpServerStatusParams;
use codewen_app_server_protocol::ListMcpServerStatusResponse;
use codewen_app_server_protocol::LoginAccountParams;
use codewen_app_server_protocol::LoginAccountResponse;
use codewen_app_server_protocol::LoginApiKeyParams;
use codewen_app_server_protocol::LoginAppBrand;
use codewen_app_server_protocol::LogoutAccountResponse;
use codewen_app_server_protocol::MarketplaceAddParams;
use codewen_app_server_protocol::MarketplaceAddResponse;
use codewen_app_server_protocol::MarketplaceInterface;
use codewen_app_server_protocol::MarketplaceRemoveParams;
use codewen_app_server_protocol::MarketplaceRemoveResponse;
use codewen_app_server_protocol::MarketplaceUpgradeErrorInfo;
use codewen_app_server_protocol::MarketplaceUpgradeParams;
use codewen_app_server_protocol::MarketplaceUpgradeResponse;
use codewen_app_server_protocol::McpResourceReadParams;
use codewen_app_server_protocol::McpResourceReadResponse;
use codewen_app_server_protocol::McpServerOauthLoginCompletedNotification;
use codewen_app_server_protocol::McpServerOauthLoginParams;
use codewen_app_server_protocol::McpServerOauthLoginResponse;
use codewen_app_server_protocol::McpServerRefreshResponse;
use codewen_app_server_protocol::McpServerStatus;
use codewen_app_server_protocol::McpServerStatusDetail;
use codewen_app_server_protocol::McpServerToolCallParams;
use codewen_app_server_protocol::McpServerToolCallResponse;
use codewen_app_server_protocol::MemoryResetResponse;
use codewen_app_server_protocol::MockExperimentalMethodParams;
use codewen_app_server_protocol::MockExperimentalMethodResponse;
use codewen_app_server_protocol::ModelListParams;
use codewen_app_server_protocol::ModelListResponse;
use codewen_app_server_protocol::PermissionProfileListParams;
use codewen_app_server_protocol::PermissionProfileListResponse;
use codewen_app_server_protocol::PermissionProfileSummary;
use codewen_app_server_protocol::PluginDetail;
use codewen_app_server_protocol::PluginInstallParams;
use codewen_app_server_protocol::PluginInstallResponse;
use codewen_app_server_protocol::PluginInstalledParams;
use codewen_app_server_protocol::PluginInstalledResponse;
use codewen_app_server_protocol::PluginInterface;
use codewen_app_server_protocol::PluginListMarketplaceKind;
use codewen_app_server_protocol::PluginListParams;
use codewen_app_server_protocol::PluginListResponse;
use codewen_app_server_protocol::PluginMarketplaceEntry;
use codewen_app_server_protocol::PluginReadParams;
use codewen_app_server_protocol::PluginReadResponse;
use codewen_app_server_protocol::PluginShareCheckoutParams;
use codewen_app_server_protocol::PluginShareCheckoutResponse;
use codewen_app_server_protocol::PluginShareContext;
use codewen_app_server_protocol::PluginShareDeleteParams;
use codewen_app_server_protocol::PluginShareDeleteResponse;
use codewen_app_server_protocol::PluginShareDiscoverability;
use codewen_app_server_protocol::PluginShareListItem;
use codewen_app_server_protocol::PluginShareListParams;
use codewen_app_server_protocol::PluginShareListResponse;
use codewen_app_server_protocol::PluginSharePrincipal;
use codewen_app_server_protocol::PluginSharePrincipalType;
use codewen_app_server_protocol::PluginShareSaveParams;
use codewen_app_server_protocol::PluginShareSaveResponse;
use codewen_app_server_protocol::PluginShareTarget;
use codewen_app_server_protocol::PluginShareUpdateDiscoverability;
use codewen_app_server_protocol::PluginShareUpdateTargetsParams;
use codewen_app_server_protocol::PluginShareUpdateTargetsResponse;
use codewen_app_server_protocol::PluginSkillReadParams;
use codewen_app_server_protocol::PluginSkillReadResponse;
use codewen_app_server_protocol::PluginSource;
use codewen_app_server_protocol::PluginSummary;
use codewen_app_server_protocol::PluginUninstallParams;
use codewen_app_server_protocol::PluginUninstallResponse;
use codewen_app_server_protocol::RateLimitResetCredit;
use codewen_app_server_protocol::RateLimitResetCreditStatus;
use codewen_app_server_protocol::RateLimitResetCreditsSummary;
use codewen_app_server_protocol::RateLimitResetType;
use codewen_app_server_protocol::RequestId;
use codewen_app_server_protocol::ReviewDelivery as ApiReviewDelivery;
use codewen_app_server_protocol::ReviewStartParams;
use codewen_app_server_protocol::ReviewStartResponse;
use codewen_app_server_protocol::ReviewTarget as ApiReviewTarget;
use codewen_app_server_protocol::SandboxMode;
use codewen_app_server_protocol::SendAddCreditsNudgeEmailParams;
use codewen_app_server_protocol::SendAddCreditsNudgeEmailResponse;
use codewen_app_server_protocol::ServerNotification;
use codewen_app_server_protocol::ServerRequestResolvedNotification;
use codewen_app_server_protocol::SkillSummary;
use codewen_app_server_protocol::SkillsConfigWriteParams;
use codewen_app_server_protocol::SkillsConfigWriteResponse;
use codewen_app_server_protocol::SkillsExtraRootsSetParams;
use codewen_app_server_protocol::SkillsExtraRootsSetResponse;
use codewen_app_server_protocol::SkillsListParams;
use codewen_app_server_protocol::SkillsListResponse;
use codewen_app_server_protocol::SortDirection;
use codewen_app_server_protocol::Thread;
use codewen_app_server_protocol::ThreadApproveGuardianDeniedActionParams;
use codewen_app_server_protocol::ThreadApproveGuardianDeniedActionResponse;
use codewen_app_server_protocol::ThreadArchiveParams;
use codewen_app_server_protocol::ThreadArchiveResponse;
use codewen_app_server_protocol::ThreadArchivedNotification;
use codewen_app_server_protocol::ThreadBackgroundTerminal;
use codewen_app_server_protocol::ThreadBackgroundTerminalsCleanParams;
use codewen_app_server_protocol::ThreadBackgroundTerminalsCleanResponse;
use codewen_app_server_protocol::ThreadBackgroundTerminalsListParams;
use codewen_app_server_protocol::ThreadBackgroundTerminalsListResponse;
use codewen_app_server_protocol::ThreadBackgroundTerminalsTerminateParams;
use codewen_app_server_protocol::ThreadBackgroundTerminalsTerminateResponse;
use codewen_app_server_protocol::ThreadClosedNotification;
use codewen_app_server_protocol::ThreadCompactStartParams;
use codewen_app_server_protocol::ThreadCompactStartResponse;
use codewen_app_server_protocol::ThreadDecrementElicitationParams;
use codewen_app_server_protocol::ThreadDecrementElicitationResponse;
use codewen_app_server_protocol::ThreadDeleteParams;
use codewen_app_server_protocol::ThreadDeleteResponse;
use codewen_app_server_protocol::ThreadDeletedNotification;
use codewen_app_server_protocol::ThreadForkParams;
use codewen_app_server_protocol::ThreadForkResponse;
use codewen_app_server_protocol::ThreadGoal;
use codewen_app_server_protocol::ThreadGoalClearParams;
use codewen_app_server_protocol::ThreadGoalClearResponse;
use codewen_app_server_protocol::ThreadGoalClearedNotification;
use codewen_app_server_protocol::ThreadGoalGetParams;
use codewen_app_server_protocol::ThreadGoalGetResponse;
use codewen_app_server_protocol::ThreadGoalSetParams;
use codewen_app_server_protocol::ThreadGoalSetResponse;
use codewen_app_server_protocol::ThreadGoalStatus;
use codewen_app_server_protocol::ThreadGoalUpdatedNotification;
use codewen_app_server_protocol::ThreadHistoryBuilder;
#[cfg(test)]
use codewen_app_server_protocol::ThreadHistoryMode;
use codewen_app_server_protocol::ThreadIncrementElicitationParams;
use codewen_app_server_protocol::ThreadIncrementElicitationResponse;
use codewen_app_server_protocol::ThreadInjectItemsParams;
use codewen_app_server_protocol::ThreadInjectItemsResponse;
use codewen_app_server_protocol::ThreadItem;
use codewen_app_server_protocol::ThreadItemEntry;
use codewen_app_server_protocol::ThreadItemsListParams;
use codewen_app_server_protocol::ThreadItemsListResponse;
use codewen_app_server_protocol::ThreadListCwdFilter;
use codewen_app_server_protocol::ThreadListParams;
use codewen_app_server_protocol::ThreadListResponse;
use codewen_app_server_protocol::ThreadLoadedListParams;
use codewen_app_server_protocol::ThreadLoadedListResponse;
use codewen_app_server_protocol::ThreadMemoryModeSetParams;
use codewen_app_server_protocol::ThreadMemoryModeSetResponse;
use codewen_app_server_protocol::ThreadMetadataGitInfoUpdateParams;
use codewen_app_server_protocol::ThreadMetadataUpdateParams;
use codewen_app_server_protocol::ThreadMetadataUpdateResponse;
use codewen_app_server_protocol::ThreadNameUpdatedNotification;
use codewen_app_server_protocol::ThreadReadParams;
use codewen_app_server_protocol::ThreadReadResponse;
use codewen_app_server_protocol::ThreadRealtimeAppendAudioParams;
use codewen_app_server_protocol::ThreadRealtimeAppendAudioResponse;
use codewen_app_server_protocol::ThreadRealtimeAppendSpeechParams;
use codewen_app_server_protocol::ThreadRealtimeAppendSpeechResponse;
use codewen_app_server_protocol::ThreadRealtimeAppendTextParams;
use codewen_app_server_protocol::ThreadRealtimeAppendTextResponse;
use codewen_app_server_protocol::ThreadRealtimeListVoicesResponse;
use codewen_app_server_protocol::ThreadRealtimeStartParams;
use codewen_app_server_protocol::ThreadRealtimeStartResponse;
use codewen_app_server_protocol::ThreadRealtimeStartTransport;
use codewen_app_server_protocol::ThreadRealtimeStopParams;
use codewen_app_server_protocol::ThreadRealtimeStopResponse;
use codewen_app_server_protocol::ThreadResumeInitialTurnsPageParams;
use codewen_app_server_protocol::ThreadResumeParams;
use codewen_app_server_protocol::ThreadResumeResponse;
use codewen_app_server_protocol::ThreadRollbackParams;
use codewen_app_server_protocol::ThreadSearchOccurrence;
use codewen_app_server_protocol::ThreadSearchOccurrencesParams;
use codewen_app_server_protocol::ThreadSearchOccurrencesResponse;
use codewen_app_server_protocol::ThreadSearchParams;
use codewen_app_server_protocol::ThreadSearchResponse;
use codewen_app_server_protocol::ThreadSearchResult;
use codewen_app_server_protocol::ThreadSearchTextRange;
use codewen_app_server_protocol::ThreadSetNameParams;
use codewen_app_server_protocol::ThreadSetNameResponse;
use codewen_app_server_protocol::ThreadSettings;
use codewen_app_server_protocol::ThreadSettingsUpdateParams;
use codewen_app_server_protocol::ThreadSettingsUpdateResponse;
use codewen_app_server_protocol::ThreadShellCommandParams;
use codewen_app_server_protocol::ThreadShellCommandResponse;
use codewen_app_server_protocol::ThreadSortKey;
use codewen_app_server_protocol::ThreadSourceKind;
use codewen_app_server_protocol::ThreadStartParams;
use codewen_app_server_protocol::ThreadStartResponse;
use codewen_app_server_protocol::ThreadStartedNotification;
use codewen_app_server_protocol::ThreadStatus;
use codewen_app_server_protocol::ThreadTurnsListParams;
use codewen_app_server_protocol::ThreadTurnsListResponse;
use codewen_app_server_protocol::ThreadUnarchiveParams;
use codewen_app_server_protocol::ThreadUnarchiveResponse;
use codewen_app_server_protocol::ThreadUnarchivedNotification;
use codewen_app_server_protocol::ThreadUnsubscribeParams;
use codewen_app_server_protocol::ThreadUnsubscribeResponse;
use codewen_app_server_protocol::ThreadUnsubscribeStatus;
use codewen_app_server_protocol::Turn;
use codewen_app_server_protocol::TurnEnvironmentParams;
use codewen_app_server_protocol::TurnError;
use codewen_app_server_protocol::TurnInterruptParams;
use codewen_app_server_protocol::TurnInterruptResponse;
use codewen_app_server_protocol::TurnItemsView;
use codewen_app_server_protocol::TurnStartParams;
use codewen_app_server_protocol::TurnStartResponse;
use codewen_app_server_protocol::TurnStatus;
use codewen_app_server_protocol::TurnSteerParams;
use codewen_app_server_protocol::TurnSteerResponse;
use codewen_app_server_protocol::UserInput as V2UserInput;
use codewen_app_server_protocol::WindowsSandboxReadiness;
use codewen_app_server_protocol::WindowsSandboxReadinessResponse;
use codewen_app_server_protocol::WindowsSandboxSetupCompletedNotification;
use codewen_app_server_protocol::WindowsSandboxSetupMode;
use codewen_app_server_protocol::WindowsSandboxSetupStartParams;
use codewen_app_server_protocol::WindowsSandboxSetupStartResponse;
use codewen_app_server_protocol::WorkspaceMessage;
use codewen_app_server_protocol::WorkspaceMessageType;
use codewen_arg0::Arg0DispatchPaths;
use codewen_backend_client::AddCreditsNudgeCreditType as BackendAddCreditsNudgeCreditType;
use codewen_backend_client::Client as BackendClient;
use codewen_backend_client::CodewenWorkspaceMessage as BackendWorkspaceMessage;
use codewen_backend_client::CodewenWorkspaceMessageType as BackendWorkspaceMessageType;
use codewen_backend_client::CodewenWorkspaceMessagesResponse as BackendWorkspaceMessagesResponse;
use codewen_backend_client::ConsumeRateLimitResetCreditCode as BackendConsumeRateLimitResetCreditCode;
use codewen_backend_client::RateLimitResetCreditDetails as BackendRateLimitResetCreditDetails;
use codewen_backend_client::RateLimitResetCreditsDetails as BackendRateLimitResetCreditsDetails;
use codewen_backend_client::RequestError as BackendRequestError;
use codewen_backend_client::TokenUsageProfile;
use codewen_chatgpt::connectors;
use codewen_chatgpt::workspace_settings;
use codewen_config::CloudConfigBundleLoadError;
use codewen_config::CloudConfigBundleLoadErrorCode;
use codewen_config::ConfigLayerStack;
use codewen_config::loader::project_trust_key;
use codewen_config::types::McpServerTransportConfig;
use codewen_connectors::AppInfo;
use codewen_core::CodewenThread;
use codewen_core::CodewenThreadSettingsOverrides;
use codewen_core::ForkSnapshot;
use codewen_core::McpManager;
use codewen_core::NewThread;
#[cfg(test)]
use codewen_core::SessionMeta;
use codewen_core::StartThreadOptions;
use codewen_core::SteerInputError;
use codewen_core::ThreadConfigSnapshot;
use codewen_core::ThreadManager;
use codewen_core::config::Config;
use codewen_core::config::ConfigOverrides;
use codewen_core::config::NetworkProxyAuditMetadata;
use codewen_core::config::edit::ConfigEdit;
use codewen_core::config::edit::ConfigEditsBuilder;
use codewen_core::connectors::AccessibleConnectorsStatus;
use codewen_core::exec::ExecCapturePolicy;
use codewen_core::exec::ExecExpiration;
use codewen_core::exec::ExecParams;
use codewen_core::exec_env::create_env;
use codewen_core::path_utils;
#[cfg(test)]
use codewen_core::read_head_for_summary;
use codewen_core::sandboxing::SandboxPermissions;
use codewen_core::truncate_rollout_after_turn_id;
use codewen_core::truncate_rollout_before_turn_id;
use codewen_core::windows_sandbox::WindowsSandboxLevelExt;
use codewen_core::windows_sandbox::WindowsSandboxSetupMode as CoreWindowsSandboxSetupMode;
use codewen_core::windows_sandbox::WindowsSandboxSetupRequest;
use codewen_core::windows_sandbox::sandbox_setup_is_complete;
use codewen_core_plugins::PluginInstallError as CorePluginInstallError;
use codewen_core_plugins::PluginInstallRequest;
use codewen_core_plugins::PluginReadRequest;
use codewen_core_plugins::PluginUninstallError as CorePluginUninstallError;
use codewen_core_plugins::PluginsManager;
use codewen_core_plugins::loader::load_plugin_apps;
use codewen_core_plugins::loader::load_plugin_mcp_servers;
use codewen_core_plugins::manifest::PluginManifestInterface;
use codewen_core_plugins::marketplace::MarketplaceError;
use codewen_core_plugins::marketplace::MarketplacePluginSource;
use codewen_core_plugins::marketplace_add::MarketplaceAddError;
use codewen_core_plugins::marketplace_add::MarketplaceAddRequest;
use codewen_core_plugins::marketplace_add::add_marketplace as add_marketplace_to_codewen_home;
use codewen_core_plugins::marketplace_remove::MarketplaceRemoveError;
use codewen_core_plugins::marketplace_remove::MarketplaceRemoveRequest as CoreMarketplaceRemoveRequest;
use codewen_core_plugins::marketplace_remove::remove_marketplace;
use codewen_core_plugins::remote::RemoteMarketplace;
use codewen_core_plugins::remote::RemoteMarketplaceSource;
use codewen_core_plugins::remote::RemotePluginCatalogError;
use codewen_core_plugins::remote::RemotePluginDetail as RemoteCatalogPluginDetail;
use codewen_core_plugins::remote::RemotePluginServiceConfig;
use codewen_core_plugins::remote::RemotePluginShareContext as RemoteCatalogPluginShareContext;
use codewen_core_plugins::remote::RemotePluginShareSummary as RemoteCatalogPluginShareSummary;
use codewen_core_plugins::remote::RemotePluginSummary as RemoteCatalogPluginSummary;
use codewen_exec_server::EnvironmentManager;
use codewen_exec_server::EnvironmentObservedStatus;
use codewen_exec_server::LOCAL_ENVIRONMENT_ID;
use codewen_exec_server::LOCAL_FS;
use codewen_features::FEATURES;
use codewen_features::Feature;
use codewen_features::Stage;
use codewen_feedback::CodewenFeedback;
use codewen_feedback::FeedbackAttachmentPath;
use codewen_feedback::FeedbackUploadOptions;
use codewen_git_utils::git_diff_to_remote;
use codewen_git_utils::resolve_root_git_project_for_trust;
use codewen_login::AuthManager;
use codewen_login::CODEX_OPEN_APP_URL;
use codewen_login::CodewenAuth;
use codewen_login::LoginSuccessPage;
use codewen_login::LoginSuccessPageBrand;
use codewen_login::ServerOptions as LoginServerOptions;
use codewen_login::ShutdownHandle;
use codewen_login::complete_device_code_login;
use codewen_login::login_with_api_key;
use codewen_login::login_with_bedrock_api_key;
use codewen_login::oauth_client_id;
use codewen_login::request_device_code;
use codewen_login::run_login_server;
use codewen_mcp::McpRuntimeContext;
use codewen_mcp::McpServerStatusSnapshot;
use codewen_mcp::McpSnapshotDetail;
use codewen_mcp::collect_mcp_server_status_snapshot_with_detail;
use codewen_mcp::discover_supported_scopes_with_http_client;
use codewen_mcp::read_mcp_resource as read_mcp_resource_without_thread;
use codewen_mcp::resolve_oauth_scopes;
use codewen_memories_write::clear_memory_roots_contents;
use codewen_model_provider::create_model_provider;
use codewen_models_manager::collaboration_mode_presets::builtin_collaboration_mode_presets;
use codewen_protocol::ThreadId;
use codewen_protocol::config_types::CollaborationMode;
use codewen_protocol::config_types::ForcedLoginMethod;
use codewen_protocol::config_types::Personality;
use codewen_protocol::config_types::ReasoningSummary;
use codewen_protocol::config_types::TrustLevel;
use codewen_protocol::config_types::WindowsSandboxLevel;
use codewen_protocol::error::CodewenErr;
use codewen_protocol::error::Result as CodewenResult;
#[cfg(test)]
use codewen_protocol::items::TurnItem;
use codewen_protocol::models::ResponseItem;
use codewen_protocol::openai_models::ReasoningEffort;
#[cfg(test)]
use codewen_protocol::permissions::FileSystemSandboxPolicy;
use codewen_protocol::protocol::AgentStatus;
use codewen_protocol::protocol::ConversationAudioParams;
use codewen_protocol::protocol::ConversationSpeechParams;
use codewen_protocol::protocol::ConversationStartParams;
use codewen_protocol::protocol::ConversationStartTransport;
use codewen_protocol::protocol::ConversationTextParams;
use codewen_protocol::protocol::EventMsg;
#[cfg(test)]
use codewen_protocol::protocol::GitInfo as CoreGitInfo;
use codewen_protocol::protocol::InitialHistory;
use codewen_protocol::protocol::McpAuthStatus as CoreMcpAuthStatus;
use codewen_protocol::protocol::Op;
use codewen_protocol::protocol::RealtimeVoicesList;
use codewen_protocol::protocol::ResumedHistory;
use codewen_protocol::protocol::ReviewDelivery as CoreReviewDelivery;
use codewen_protocol::protocol::ReviewRequest;
use codewen_protocol::protocol::ReviewTarget as CoreReviewTarget;
use codewen_protocol::protocol::RolloutItem;
use codewen_protocol::protocol::SessionConfiguredEvent;
#[cfg(test)]
use codewen_protocol::protocol::SessionMetaLine;
use codewen_protocol::protocol::TurnEnvironmentSelection;
use codewen_protocol::protocol::TurnEnvironmentSelections;
use codewen_protocol::protocol::W3cTraceContext;
use codewen_protocol::protocol::strip_user_message_prefix;
use codewen_protocol::user_input::MAX_USER_INPUT_TEXT_CHARS;
use codewen_protocol::user_input::UserInput as CoreInputItem;
use codewen_rmcp_client::perform_oauth_login_return_url_with_http_client;
use codewen_rollout::is_persisted_rollout_item;
use codewen_rollout::state_db::StateDbHandle;
use codewen_rollout::state_db::reconcile_rollout;
use codewen_state::ThreadMetadata;
use codewen_state::log_db::LogDbLayer;
use codewen_thread_store::ArchiveThreadParams as StoreArchiveThreadParams;
use codewen_thread_store::DeleteThreadsParams as StoreDeleteThreadsParams;
use codewen_thread_store::GitInfoPatch as StoreGitInfoPatch;
use codewen_thread_store::ListItemsParams as StoreListItemsParams;
use codewen_thread_store::ListThreadsParams as StoreListThreadsParams;
use codewen_thread_store::ListTurnsParams as StoreListTurnsParams;
use codewen_thread_store::LoadThreadHistoryParams as StoreLoadThreadHistoryParams;
use codewen_thread_store::LocalThreadStore;
use codewen_thread_store::ReadThreadByRolloutPathParams as StoreReadThreadByRolloutPathParams;
use codewen_thread_store::ReadThreadParams as StoreReadThreadParams;
use codewen_thread_store::SearchThreadOccurrencesParams as StoreSearchThreadOccurrencesParams;
use codewen_thread_store::SearchThreadsParams as StoreSearchThreadsParams;
use codewen_thread_store::SortDirection as StoreSortDirection;
use codewen_thread_store::StoredThread;
use codewen_thread_store::StoredTurn;
use codewen_thread_store::StoredTurnItemsView;
use codewen_thread_store::StoredTurnStatus;
use codewen_thread_store::ThreadMetadataPatch as StoreThreadMetadataPatch;
use codewen_thread_store::ThreadRelationFilter as StoreThreadRelationFilter;
use codewen_thread_store::ThreadSortKey as StoreThreadSortKey;
use codewen_thread_store::ThreadStore;
use codewen_thread_store::ThreadStoreError;
use codewen_utils_absolute_path::AbsolutePathBuf;
use codewen_utils_pty::DEFAULT_OUTPUT_BYTES_CAP;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as IoError;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tokio_util::sync::DropGuard;
use tokio_util::task::TaskTracker;
use toml::Value as TomlValue;
use tracing::Instrument;
use tracing::error;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

#[cfg(test)]
use codewen_app_server_protocol::ServerRequest;

mod account_processor;
mod apps_processor;
mod bedrock_auth;
mod catalog_processor;
mod command_exec_processor;
mod config_processor;
mod environment_processor;
mod feedback_doctor_report;
mod feedback_processor;
mod fs_processor;
mod git_processor;
mod initialize_processor;
mod marketplace_processor;
mod mcp_processor;
mod plugins;
mod process_exec_processor;
mod remote_control_processor;
mod search;
mod thread_fork_goal;
mod thread_processor;
mod token_usage_replay;
mod turn_processor;
mod windows_sandbox_processor;

pub(crate) use account_processor::AccountRequestProcessor;
pub(crate) use apps_processor::AppsRequestProcessor;
pub(crate) use catalog_processor::CatalogRequestProcessor;
pub(crate) use command_exec_processor::CommandExecRequestProcessor;
pub(crate) use config_processor::ConfigRequestProcessor;
pub(crate) use environment_processor::EnvironmentRequestProcessor;
pub(crate) use feedback_processor::FeedbackRequestProcessor;
pub(crate) use fs_processor::FsRequestProcessor;
pub(crate) use git_processor::GitRequestProcessor;
pub(crate) use initialize_processor::InitializeRequestProcessor;
pub(crate) use marketplace_processor::MarketplaceRequestProcessor;
pub(crate) use mcp_processor::McpRequestProcessor;
pub(crate) use plugins::PluginRequestProcessor;
pub(crate) use process_exec_processor::ProcessExecRequestProcessor;
pub(crate) use remote_control_processor::RemoteControlRequestProcessor;
pub(crate) use search::SearchRequestProcessor;
pub(crate) use thread_goal_processor::ThreadGoalRequestProcessor;
pub(crate) use thread_processor::ThreadRequestProcessor;
pub(crate) use turn_processor::TurnRequestProcessor;
pub(crate) use windows_sandbox_processor::WindowsSandboxRequestProcessor;

use crate::error_code::internal_error;
use crate::error_code::invalid_request;
use crate::filters::compute_source_filters;
use crate::filters::source_kind_matches;
use crate::thread_state::ConnectionCapabilities;
use crate::thread_state::ThreadListenerCommand;
use crate::thread_state::ThreadState;
use crate::thread_state::ThreadStateManager;
use token_usage_replay::restored_token_usage_turn_id;
use token_usage_replay::send_thread_token_usage_update_to_connection;

fn resolve_request_cwd(cwd: Option<PathBuf>) -> Result<Option<AbsolutePathBuf>, JSONRPCErrorError> {
    cwd.map(|cwd| {
        AbsolutePathBuf::relative_to_current_dir(path_utils::normalize_for_native_workdir(cwd))
            .map_err(|err| invalid_request(format!("invalid cwd: {err}")))
    })
    .transpose()
}

fn resolve_turn_environment_selections(
    thread_manager: &ThreadManager,
    environments: Option<Vec<TurnEnvironmentParams>>,
) -> Result<Option<Vec<TurnEnvironmentSelection>>, JSONRPCErrorError> {
    let Some(environments) = environments else {
        return Ok(None);
    };
    let mut selections = Vec::with_capacity(environments.len());
    for environment in environments {
        let environment_id = environment.environment_id;
        let cwd = environment
            .cwd
            .to_inferred_path_uri()
            .ok_or_else(|| {
                invalid_request(format!(
                    "invalid cwd for environment `{environment_id}`: path `{}` does not use absolute POSIX or Windows path syntax",
                    environment.cwd
                ))
            })?;
        let workspace_roots = environment
            .runtime_workspace_roots
            .map(|roots| {
                let mut resolved_roots = Vec::new();
                for root in roots {
                    let root = root.to_inferred_path_uri().ok_or_else(|| {
                        invalid_request(format!(
                            "invalid runtime workspace root for environment `{environment_id}`: path `{root}` does not use absolute POSIX or Windows path syntax"
                        ))
                    })?;
                    if !resolved_roots.contains(&root) {
                        resolved_roots.push(root);
                    }
                }
                Ok::<_, JSONRPCErrorError>(resolved_roots)
            })
            .transpose()?
            .unwrap_or_else(|| vec![cwd.clone()]);
        selections.push(TurnEnvironmentSelection {
            environment_id,
            cwd,
            workspace_roots,
        });
    }
    thread_manager
        .validate_environment_selections(&selections)
        .map_err(environment_selection_error)?;
    Ok(Some(selections))
}

fn resolve_runtime_workspace_roots(workspace_roots: Vec<AbsolutePathBuf>) -> Vec<AbsolutePathBuf> {
    let mut resolved_roots = Vec::new();
    for root in workspace_roots {
        if !resolved_roots.iter().any(|existing| existing == &root) {
            resolved_roots.push(root);
        }
    }
    resolved_roots
}

mod config_errors;
mod request_errors;
mod thread_delete;
mod thread_goal_processor;
mod thread_lifecycle;
mod thread_resume_redaction;
mod thread_summary;

use self::config_errors::*;
use self::request_errors::*;
use self::thread_goal_processor::api_thread_goal_from_state;
use self::thread_lifecycle::*;
use self::thread_resume_redaction::*;
use self::thread_summary::*;

pub(crate) use self::thread_lifecycle::populate_thread_turns_from_history;
pub(crate) use self::thread_processor::thread_from_stored_thread;
#[cfg(test)]
pub(crate) use self::thread_summary::read_summary_from_rollout;
#[cfg(test)]
pub(crate) use self::thread_summary::summary_to_thread;
pub(crate) use self::thread_summary::thread_settings_from_config_snapshot;
pub(crate) use self::thread_summary::thread_settings_from_core_snapshot;

pub(crate) fn build_legacy_api_turns_from_rollout_items(items: &[RolloutItem]) -> Vec<Turn> {
    let mut builder = ThreadHistoryBuilder::new();
    for item in items {
        if is_persisted_rollout_item(item, codewen_protocol::protocol::ThreadHistoryMode::Legacy) {
            builder.handle_rollout_item(item);
        }
    }
    builder.finish()
}
