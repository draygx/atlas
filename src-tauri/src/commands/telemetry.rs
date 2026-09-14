//! No-op telemetry commands stub for İmece.
use std::sync::Arc;
use tauri::{AppHandle, State};
use crate::telemetry::{OrgIdentity, TelemetryClient, TelemetryConfig};

#[tauri::command]
pub fn telemetry_config(client: State<'_, Arc<TelemetryClient>>) -> TelemetryConfig {
    client.config()
}

#[tauri::command]
pub fn telemetry_set_org(_app: AppHandle, _org_id: Option<String>) {}

pub fn resolve_org(_app: &AppHandle, _org_id: Option<&str>) -> Option<OrgIdentity> {
    None
}
