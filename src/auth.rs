//! Site identity: challenge-response sign-in with delegation gate plus docs gate.
//!
//! Sign-in runs once on the main page. The visitor presents a did:peer
//! identity, signs the open challenge with the identity secret inside the
//! browser, and the server returns a bearer token kept in browser storage.
//! The models page and the settings page recognize that same token instead
//! of running their own sign-in. The docs page recognizes it too, through
//! the approved list seeded at boot from allowed_dids.json.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::collections::{HashMap, HashSet};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{LazyLock, Mutex};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(target_arch = "wasm32"))]
static SESSIONS: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(not(target_arch = "wasm32"))]
static CHALLENGES: LazyLock<Mutex<HashMap<String, (String, u64)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(not(target_arch = "wasm32"))]
static PENDING: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

#[cfg(not(target_arch = "wasm32"))]
static APPROVED: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

/// Called once at server boot with the owner-edited approved list.
#[cfg(feature = "ssr")]
pub fn seed_approved(dids: Vec<String>) {
    if let Ok(mut approved) = APPROVED.lock() {
        approved.extend(dids);
    }
}

#[cfg(not(target_arch = "wasm32"))]
const NONCE_MINUTES: u64 = 5;

#[cfg(not(target_arch = "wasm32"))]
fn now_epoch() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Format check: did:peer:2.Vz plus base58 32-byte key.
pub fn did_format_ok(did: &str) -> bool {
    const PREFIX: &str = "did:peer:2.Vz";
    let Some(key_part) = did.strip_prefix(PREFIX) else {
        return false;
    };
    let Ok(bytes) = bs58::decode(key_part).into_vec() else {
        return false;
    };
    bytes.len() == 32
}

#[cfg(not(target_arch = "wasm32"))]
fn new_token() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn new_nonce() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Namespace under which model-updates delegations live.
#[cfg(feature = "ssr")]
fn updates_namespace() -> [u8; 32] {
    exocore::protocol_internal::derive_namespace("model-updates")
}

/// A freshly generated identity. The secret is shown once, at creation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewIdentity {
    pub did: String,
    pub secret: String,
    pub token: String,
}

/// What the gate sees behind a token.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GateView {
    pub did: String,
    pub model_updates: bool,
    pub admin: bool,
}

/// What the docs gate sees behind a token.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocView {
    pub did: String,
    pub approved: bool,
}

/// One waiting identity for the owner tools page.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingIdentity {
    pub did: String,
}

/// Issue a one-time challenge for the presented identity.
#[server]
pub async fn request_nonce(did: String) -> Result<String, ServerFnError> {
    let did = did.trim().to_string();
    if !did_format_ok(&did) {
        return Err(ServerFnError::ServerError("That is not a did:peer identity. It looks like did:peer:2.Vz…".into()));
    }
    let nonce = new_nonce();
    let expiry = now_epoch() + NONCE_MINUTES * 60;
    match CHALLENGES.lock() {
        Ok(mut c) => {
            c.insert(did, (nonce.clone(), expiry));
        }
        Err(_) => return Err(ServerFnError::ServerError("challenge store locked".into())),
    }
    Ok(nonce)
}

/// Answer the challenge with the identity secret's signature over the nonce.
/// The secret itself never crosses the wire.
#[server]
pub async fn answer_nonce(did: String, nonce: String, signature_b58: String) -> Result<String, ServerFnError> {
    let did = did.trim().to_string();
    let expected = match CHALLENGES.lock().map(|mut c| c.remove(&did)) {
        Ok(v) => v,
        Err(_) => return Err(ServerFnError::ServerError("challenge store locked".into())),
    };
    let Some((want, expiry)) = expected else {
        return Err(ServerFnError::ServerError("No open challenge for that identity. Request one first.".into()));
    };
    if want != nonce || now_epoch() > expiry {
        return Err(ServerFnError::ServerError("Challenge expired or mismatched. Request a fresh one.".into()));
    }
    if !did_format_ok(&did) {
        return Err(ServerFnError::ServerError("That is not a did:peer identity.".into()));
    }
    let key_part = did.strip_prefix("did:peer:2.Vz").unwrap_or_default();
    let pk_bytes = match bs58::decode(key_part).into_vec() {
        Ok(b) => b,
        Err(_) => return Err(ServerFnError::ServerError("Identity key does not decode.".into())),
    };
    let pk_arr: [u8; 32] = match pk_bytes.try_into() {
        Ok(a) => a,
        Err(_) => return Err(ServerFnError::ServerError("Identity key is not 32 bytes.".into())),
    };
    let verifying_key = match ed25519_dalek::VerifyingKey::from_bytes(&pk_arr) {
        Ok(k) => k,
        Err(_) => return Err(ServerFnError::ServerError("Identity key rejected.".into())),
    };
    let sig_bytes = match bs58::decode(signature_b58.trim()).into_vec() {
        Ok(b) => b,
        Err(_) => return Err(ServerFnError::ServerError("Signature does not decode.".into())),
    };
    let sig_arr: [u8; 64] = match sig_bytes.try_into() {
        Ok(a) => a,
        Err(_) => return Err(ServerFnError::ServerError("Signature is not 64 bytes.".into())),
    };
    let signature = ed25519_dalek::Signature::from_bytes(&sig_arr);
    use ed25519_dalek::Verifier;
    if verifying_key.verify(nonce.as_bytes(), &signature).is_err() {
        return Err(ServerFnError::ServerError("Signature check failed. The secret does not match the identity.".into()));
    }
    let token = new_token();
    match SESSIONS.lock() {
        Ok(mut s) => {
            s.insert(token.clone(), did);
        }
        Err(_) => return Err(ServerFnError::ServerError("sessions locked".into())),
    }
    Ok(token)
}

/// Generate a peer identity on the spot and continue signed in with it.
/// The new identity holds no delegations: the owner grants them.
#[server]
pub async fn create_identity() -> Result<NewIdentity, ServerFnError> {
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    let signing_key = SigningKey::generate(&mut OsRng);
    let public_key = signing_key.verifying_key();
    let did = format!("did:peer:2.Vz{}", bs58::encode(public_key.as_bytes()).into_string());
    let secret = bs58::encode(signing_key.to_bytes()).into_string();

    let token = new_token();
    match SESSIONS.lock() {
        Ok(mut sessions) => {
            sessions.insert(token.clone(), did.clone());
        }
        Err(_) => return Err(ServerFnError::ServerError("sessions locked".into())),
    }
    if let Ok(mut p) = PENDING.lock() {
        p.insert(did.clone());
    }
    Ok(NewIdentity { did, secret, token })
}

/// Drop a session token.
#[server]
pub async fn sign_out(token: String) -> Result<(), ServerFnError> {
    match SESSIONS.lock() {
        Ok(mut sessions) => {
            sessions.remove(&token);
            Ok(())
        }
        Err(_) => Err(ServerFnError::ServerError("sessions locked".into())),
    }
}

#[cfg(target_arch = "wasm32")]
const TOKEN_KEY: &str = "exo_session";

/// Browser token stash. Server builds get inert stubs.
#[cfg(target_arch = "wasm32")]
pub fn store_token(token: &str) {
    let _ = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .map(|s| s.set_item(TOKEN_KEY, token));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn store_token(_token: &str) {}

#[cfg(target_arch = "wasm32")]
pub fn read_token() -> Option<String> {
    web_sys::window()?.local_storage().ok()?.unwrap().get_item(TOKEN_KEY).ok()?
}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_token() -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn clear_token() {
    let _ = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .map(|s| s.remove_item(TOKEN_KEY));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn clear_token() {}

#[cfg(feature = "ssr")]
fn snapshot_path() -> std::path::PathBuf {
    std::path::Path::new("exostorage").join("site-delegations.json")
}

#[cfg(feature = "ssr")]
async fn persist_delegations() {
    use exocore::network_internal::CAPABILITY_STORE;
    let store = CAPABILITY_STORE.read().await;
    let _ = std::fs::create_dir_all("exostorage");
    if let Ok(json) = serde_json::to_string(&*store) {
        let _ = std::fs::write(snapshot_path(), json);
    }
}

#[cfg(feature = "ssr")]
async fn restore_delegations() {
    use exocore::network_internal::CAPABILITY_STORE;
    if let Ok(json) = std::fs::read_to_string(snapshot_path()) {
        if let Ok(data) = serde_json::from_str(&json) {
            let mut store = CAPABILITY_STORE.write().await;
            *store = data;
        }
    }
}

#[cfg(feature = "ssr")]
fn caller_did(token: &str) -> Option<String> {
    SESSIONS.lock().ok()?.get(token).cloned()
}

#[cfg(feature = "ssr")]
async fn level_of(did: &str) -> Option<exocore::protocol_internal::PermissionLevel> {
    use exocore::network_internal::CAPABILITY_STORE;
    restore_delegations().await;
    let store = CAPABILITY_STORE.read().await;
    store.get(&updates_namespace())?.get(did).cloned()
}

/// Gate check behind a token: who is it, and does a delegation cover models?
#[server]
pub async fn gate_check(token: String) -> Result<Option<GateView>, ServerFnError> {
    let Some(did) = caller_did(&token) else {
        return Ok(None);
    };
    let models = matches!(
        level_of(&did).await,
        Some(exocore::protocol_internal::PermissionLevel::Read)
            | Some(exocore::protocol_internal::PermissionLevel::Write)
            | Some(exocore::protocol_internal::PermissionLevel::Admin)
    );
    let admin = matches!(level_of(&did).await, Some(exocore::protocol_internal::PermissionLevel::Admin));
    if !models {
        if let Ok(mut p) = PENDING.lock() {
            p.insert(did.clone());
        }
    }
    Ok(Some(GateView { did, model_updates: models, admin }))
}

/// Docs gate behind a token: who is it, and is it on the approved list?
#[server]
pub async fn docs_content(token: String) -> Result<DocView, ServerFnError> {
    let Some(did) = caller_did(&token) else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let approved = APPROVED.lock().map(|a| a.contains(&did)).unwrap_or(false);
    Ok(DocView { did, approved })
}

/// Waiting identities for the owner tools page. Caller must hold Admin.
#[server]
pub async fn pending_list(token: String) -> Result<Vec<PendingIdentity>, ServerFnError> {
    let Some(did) = caller_did(&token) else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    if !matches!(level_of(&did).await, Some(exocore::protocol_internal::PermissionLevel::Admin)) {
        return Err(ServerFnError::ServerError("Owner tools need the Admin delegation.".into()));
    }
    let list = match PENDING.lock() {
        Ok(p) => p.iter().map(|d| PendingIdentity { did: d.clone() }).collect(),
        Err(_) => return Err(ServerFnError::ServerError("pending store locked".into())),
    };
    Ok(list)
}

/// Grant a delegation. Caller must hold Admin, except the bootstrapped owner
/// named in OWNER_DID on an empty store. Signed by the node identity.
#[server]
pub async fn grant_access(token: String, delegatee: String, level: String) -> Result<(), ServerFnError> {
    let Some(did) = caller_did(&token) else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let delegatee = delegatee.trim().to_string();
    if !did_format_ok(&delegatee) {
        return Err(ServerFnError::ServerError("That delegatee is not a did:peer identity.".into()));
    }
    let want = match level.as_str() {
        "read" => exocore::protocol_internal::PermissionLevel::Read,
        "write" => exocore::protocol_internal::PermissionLevel::Write,
        "admin" => exocore::protocol_internal::PermissionLevel::Admin,
        _ => return Err(ServerFnError::ServerError("Level reads read, write, or admin.".into())),
    };
    let admin = matches!(level_of(&did).await, Some(exocore::protocol_internal::PermissionLevel::Admin));
    let bootstrap = match std::env::var("OWNER_DID") {
        Ok(owner) => owner.split(',').any(|o| o.trim() == did),
        Err(_) => false,
    };
    {
        use exocore::network_internal::CAPABILITY_STORE;
        let store = CAPABILITY_STORE.read().await;
        let empty = store.get(&updates_namespace()).map(|m| m.is_empty()).unwrap_or(true);
        if !(admin || (bootstrap && empty)) {
            return Err(ServerFnError::ServerError("Owner tools need the Admin delegation.".into()));
        }
    }
    let node_did = match std::env::var("NODE_DID") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => return Err(ServerFnError::ServerError("Node identity unset on the server.".into())),
    };
    let node_secret = match std::env::var("NODE_SECRET") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => return Err(ServerFnError::ServerError("Node secret unset on the server.".into())),
    };
    let cap = match exocore::protocol_internal::delegate_capability(&node_did, &delegatee, updates_namespace(), want.clone(), &node_secret) {
        Ok(c) => c,
        Err(e) => return Err(ServerFnError::ServerError(e)),
    };
    if !cap.verify() {
        return Err(ServerFnError::ServerError("Fresh delegation failed its own check.".into()));
    }
    {
        use exocore::network_internal::CAPABILITY_STORE;
        let mut store = CAPABILITY_STORE.write().await;
        store.entry(updates_namespace()).or_default().insert(delegatee.clone(), want);
    }
    persist_delegations().await;
    if let Ok(mut p) = PENDING.lock() {
        p.remove(&delegatee);
    }
    Ok(())
}

/// Revoke a delegation. Caller must hold Admin.
#[server]
pub async fn revoke_access(token: String, delegatee: String) -> Result<(), ServerFnError> {
    let Some(did) = caller_did(&token) else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    if !matches!(level_of(&did).await, Some(exocore::protocol_internal::PermissionLevel::Admin)) {
        return Err(ServerFnError::ServerError("Owner tools need the Admin delegation.".into()));
    }
    {
        use exocore::network_internal::CAPABILITY_STORE;
        let mut store = CAPABILITY_STORE.write().await;
        if let Some(map) = store.get_mut(&updates_namespace()) {
            map.remove(delegatee.trim());
        }
    }
    persist_delegations().await;
    Ok(())
}

/// Dashboard settings for one visitor.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct VisitorSettings {
    pub appearance: String,
    pub width: String,
}

/// Read the visitor's server-side settings copy. Signed-in visitors only.
#[server]
pub async fn read_settings(token: String) -> Result<VisitorSettings, ServerFnError> {
    let Some(did) = caller_did(&token) else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let path = std::path::Path::new("exostorage").join("site-settings.json");
    if let Ok(json) = std::fs::read_to_string(path) {
        if let Ok(map) = serde_json::from_str::<HashMap<String, VisitorSettings>>(&json) {
            if let Some(s) = map.get(&did) {
                return Ok(s.clone());
            }
        }
    }
    Ok(VisitorSettings::default())
}

/// Save the visitor's server-side settings copy. Signed-in visitors only.
/// The browser copy stays primary; this copy rides the Willow namespace on sync.
#[server]
pub async fn save_settings(token: String, settings: VisitorSettings) -> Result<(), ServerFnError> {
    let Some(did) = caller_did(&token) else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let path = std::path::Path::new("exostorage").join("site-settings.json");
    let mut map: HashMap<String, VisitorSettings> = std::fs::read_to_string(&path)
        .ok()
        .and_then(|j| serde_json::from_str(&j).ok())
        .unwrap_or_default();
    map.insert(did, settings);
    let _ = std::fs::create_dir_all("exostorage");
    match serde_json::to_string(&map) {
        Ok(json) => match std::fs::write(path, json) {
            Ok(()) => Ok(()),
            Err(_) => Err(ServerFnError::ServerError("Settings did not save.".into())),
        },
        Err(_) => Err(ServerFnError::ServerError("Settings did not encode.".into())),
    }
}
