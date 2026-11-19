//! Models Leptos site: verification dashboard with live operations.
//!
//! Round data flows from the boltstore `models` database over HTTP.
//! Delegations flow from the Willow capabilities namespace, read directly
//! by the gate after sign-in. Menu visibility follows delegations.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Boltstore reachability for server functions.
#[cfg(feature = "ssr")]
fn store_base() -> String {
    std::env::var("MODELS_STORE").unwrap_or_else(|_| "http://127.0.0.1:8080".into())
}

/// One registry row as served for the dashboard.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelRow {
    pub model_id: String,
    pub provider: String,
    pub context_window: Option<i64>,
    pub last_tested: Option<String>,
    pub is_active: bool,
}

/// Registry rows for the dashboard views.
#[server]
pub async fn registry_rows() -> Result<Vec<ModelRow>, ServerFnError> {
    let key = match std::env::var("MODELS_DASH_KEY") {
        Ok(k) => k,
        Err(_) => return Err(ServerFnError::ServerError("dashboard key unset".into())),
    };
    let resp = match reqwest::Client::new()
        .post(format!("{}/api/databases/models/query", store_base()))
        .bearer_auth(key)
        .json(&serde_json::json!({
            "sql": "SELECT model_id, provider, context_window, last_tested, is_active FROM model_registry ORDER BY provider, model_id"
        }))
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    let body: serde_json::Value = match resp.json().await {
        Ok(b) => b,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };
    let rows = body.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default();
    Ok(rows
        .iter()
        .map(|r| ModelRow {
            model_id: r.get("model_id").and_then(|v| v.as_str()).unwrap_or_default().into(),
            provider: r.get("provider").and_then(|v| v.as_str()).unwrap_or_default().into(),
            context_window: r.get("context_window").and_then(|v| v.as_i64()),
            last_tested: r.get("last_tested").and_then(|v| v.as_str()).map(|s| s.chars().take(10).collect()),
            is_active: r.get("is_active").and_then(|v| v.as_i64()).unwrap_or(0) == 1,
        })
        .collect())
}
