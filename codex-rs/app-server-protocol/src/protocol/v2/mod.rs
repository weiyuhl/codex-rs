mod shared;

mod account;
mod apps;
mod attestation;
mod collaboration_mode;
mod command_exec;
mod config;
mod current_time;
mod environment;
mod experimental_feature;
mod feedback;
mod fs;
mod hook;
mod item;
mod mcp;
mod model;
mod notification;
mod permissions;
mod plugin;
mod process;
mod realtime;
mod remote_control;
mod review;
mod thread;
mod thread_data;
mod turn;

pub use account::*;
pub use apps::*;
pub use attestation::*;
pub use collaboration_mode::*;
pub use command_exec::*;
pub use config::*;
pub use current_time::*;
pub use environment::*;
pub use experimental_feature::*;
pub use feedback::*;
pub use fs::*;
pub use hook::*;
pub use item::*;
pub use mcp::*;
pub use model::*;
pub use notification::*;
pub use permissions::*;
pub use plugin::*;
pub use process::*;
pub use realtime::*;
pub use remote_control::*;
pub use review::*;
pub use shared::*;
pub use thread::*;
pub use thread_data::*;
pub use turn::*;

pub type WindowsSandboxSetupMode = String;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct InitializeParams;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct InitializeResponse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct GetConversationSummaryParams;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct GetConversationSummaryResponse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct GitDiffToRemoteParams;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct GitDiffToRemoteResponse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct GetAuthStatusParams;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct GetAuthStatusResponse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct InterruptConversationResponse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct ApplyPatchApprovalParams;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct ApplyPatchApprovalResponse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct ExecCommandApprovalParams;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct ExecCommandApprovalResponse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, schemars::JsonSchema, ts_rs::TS)]
pub struct ExecOneOffCommandParams {
    pub command: Vec<String>,
    pub timeout_ms: Option<u64>,
    pub cwd: Option<std::path::PathBuf>,
    pub sandbox_policy: Option<permissions::SandboxPolicy>,
}

#[cfg(test)]
mod tests;
