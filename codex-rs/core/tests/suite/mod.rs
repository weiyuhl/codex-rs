// Aggregates all former standalone integration tests as modules.

#[cfg(not(target_os = "windows"))]
mod abort_tasks;
mod additional_context;
mod agent_execution;
mod agent_websocket;
mod agents_md;
mod apply_patch_cli;
#[cfg(not(target_os = "windows"))]
mod approvals;
mod audio_truncation;
mod auto_review;
mod catalog_permission_messages;
mod cli_stream;
mod client;
mod client_websockets;
mod code_mode;
mod code_mode_elicitation;
mod codex_delegate;
mod collaboration_instructions;
mod compact;
mod compact_remote;
mod compact_resume_fork;
mod current_time_reminder;
mod deprecation_notice;
mod exec;
mod exec_policy;
#[cfg(not(target_os = "windows"))]
mod extension_sandbox;
mod external_auth;
mod fork_thread;
#[cfg(not(target_os = "windows"))]
mod guardian_review;
#[cfg(not(target_os = "windows"))]
mod hooks;
#[cfg(not(target_os = "windows"))]
mod hooks_mcp;
mod image_rollout;
mod items;
mod json_result;
mod live_cli;
mod mcp_auth_elicitation;
mod mcp_auth_refresh;
#[cfg(unix)]
mod mcp_refresh_cleanup;
mod mcp_tool_cache;
mod mcp_tool_exposure;
mod mcp_turn_metadata;
mod model_overrides;
mod model_runtime_selectors;
mod model_switching;
mod model_visible_layout;
mod models_cache_ttl;
mod models_etag_responses;
mod multi_agent_mode;
mod multi_agent_resume;
#[cfg(unix)]
mod multi_exec_server_sandbox;
mod network_approval;
mod openai_file_mcp;
mod otel;
mod override_updates;
mod pending_input;
mod permissions_messages;
mod personality;
mod plugins;
mod prompt_cache_key;
mod prompt_caching;
mod prompt_debug_tests;
mod quota_exceeded;
mod realtime_conversation;
mod realtime_initial_items;
mod remote_env;
mod remote_models;
mod request_compression;
#[cfg(not(target_os = "windows"))]
mod request_permissions;
#[cfg(not(target_os = "windows"))]
mod request_permissions_tool;
mod request_plugin_install;
mod request_user_input;
mod responses_api_proxy_headers;
mod responses_lite;
mod resume;
mod resume_warning;
mod review;
mod rmcp_client;
mod rollout_budget;
mod rollout_list_find;
mod safety_buffering;
mod safety_check_downgrade;
mod search_tool;
mod shell_command;
mod shell_serialization;
mod skill_approval;
mod skills;
mod skills_extension;
mod spawn_agent_description;
mod sqlite_state;
mod stream_error_allows_next_turn;
mod stream_no_completed;
mod subagent_notifications;
mod token_budget;
mod tool_harness;
mod tool_parallelism;
mod tools;
mod truncation;
mod turn_state;
mod unified_exec;
mod unified_exec_process_events;
#[cfg(unix)]
mod unified_exec_zsh_fork_approvals;
mod unstable_features_warning;
mod user_notification;
mod view_image;
mod web_search;
mod websocket_fallback;
mod window_headers;
mod workspace_roots;
