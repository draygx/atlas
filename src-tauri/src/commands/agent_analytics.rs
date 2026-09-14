//! No-op agent analytics stub for İmece.
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct UsageSnap;

#[derive(Debug, Default)]
pub struct TurnAcc;

impl TurnAcc {
    pub fn note_tool_call(&mut self, _salt: u64, _tc: &atlas_agent_wire::ToolCall) {}
    pub fn note_usage(&mut self, _usage: &atlas_agent_wire::Usage) {}
    pub fn note_context(&mut self, _used: u64, _size: u64, _cost: f64) {}
    pub fn note_permission_request(&mut self) {}
    pub fn note_permission_resolved(&mut self) {}
    pub fn note_retry(&mut self) {}
    pub fn note_compaction(&mut self) {}
    pub fn note_compression_saved(&mut self, _tokens: u64) {}
    pub fn note_model(&mut self, _model_id: &str) {}
    pub fn note_mode_change(&mut self) {}
    pub fn note_assistant_message(&mut self) {}
    pub fn note_plan_update(&mut self) {}
}

pub struct AnalyticsState {
    salt: u64,
}

impl AnalyticsState {
    pub fn new() -> Self {
        Self { salt: 0 }
    }

    pub fn begin_turn(&self, _session_id: &str, _turn_seq: u64) {}

    pub fn has_turn(&self, _session_id: &str, _turn_seq: u64) -> bool {
        false
    }

    pub fn with_turn(&self, _session_id: &str, _f: impl FnOnce(&mut TurnAcc)) {}

    pub fn forget_session(&self, _session_id: &str) {}

    pub fn session_ref(&self, _session_id: &str) -> String {
        String::new()
    }

    pub fn salt(&self) -> u64 {
        self.salt
    }

    pub fn finish_turn(&self, _session_id: &str, _turn_seq: u64) -> Option<Value> {
        None
    }
}
