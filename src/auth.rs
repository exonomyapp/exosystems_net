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

#[cfg(feature = "ssr")]
use crate::identity_store;

#[cfg(not(target_arch = "wasm32"))]
static CHALLENGES: LazyLock<Mutex<HashMap<String, (String, u64)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(not(target_arch = "wasm32"))]
static PENDING: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

#[cfg(not(target_arch = "wasm32"))]
static STORE: LazyLock<Mutex<Option<sqlx::SqlitePool>>> =
    LazyLock::new(|| Mutex::new(None));

/// Called once at server boot. Opens the SQLite identity store and runs
/// migrations. Replaces the allowed_dids.json seed.
#[cfg(feature = "ssr")]
pub async fn init_store() -> Result<(), sqlx::Error> {
    let pool = identity_store::open().await?;
    if let Ok(mut store) = STORE.lock() {
        *store = Some(pool);
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
const NONCE_MINUTES: u64 = 5;

#[cfg(not(target_arch = "wasm32"))]
fn now_epoch() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Get the SQLite identity store pool, if opened at boot.
#[cfg(feature = "ssr")]
fn store_pool() -> Option<sqlx::SqlitePool> {
    STORE.lock().ok().and_then(|s| s.clone())
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

/// Namespace under which Models delegations live.
#[cfg(feature = "ssr")]
fn updates_namespace() -> [u8; 32] {
    exocore::protocol_internal::derive_namespace("models")
}

/// A freshly generated identity. The secret is shown once, at creation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewIdentity {
    pub did: String,
    pub secret: String,
    pub token: String,
}

/// One row from the petitions table. Serialize/Deserialize are always
/// derived so the type crosses the #[server] boundary; sqlx::FromRow is
/// only derived with the ssr feature.
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PetitionRow {
    pub id: String,
    pub did: String,
    pub display_name: Option<String>,
    pub created_at: String,
    pub status: String,
}

/// What the gate sees behind a token.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GateView {
    pub did: String,
    pub models: bool,
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
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    let known = identity_store::get_identity(&pool, &did).await.map(|r| r.is_some()).unwrap_or(false);
    let nonce = new_nonce();
    let expiry = now_epoch() + NONCE_MINUTES * 60;
    match CHALLENGES.lock() {
        Ok(mut c) => {
            c.insert(did.clone(), (nonce.clone(), expiry));
        }
        Err(_) => return Err(ServerFnError::ServerError("challenge store locked".into())),
    }
    if known {
        Ok(nonce)
    } else {
        Ok(nonce)
    }
}

/// Result of a challenge-response sign-in round.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignInResult {
    pub token: String,
    pub needs_approval: bool,
}

/// Answer the challenge with the identity secret's signature over the nonce.
/// The secret itself never crosses the wire.
#[server]
pub async fn answer_nonce(did: String, nonce: String, signature_b58: String) -> Result<SignInResult, ServerFnError> {
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
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    if let Err(e) = identity_store::insert_session(&pool, &token, &did).await {
        return Err(ServerFnError::ServerError(format!("could not store session: {e}")));
    }
    let known = identity_store::get_identity(&pool, &did).await.map(|r| r.is_some()).unwrap_or(false);
    if !known {
        // Unknown identity that proved secret ownership: requires conscire
        // approval from the node owner before sign-in is complete.
        match PENDING.lock() {
            Ok(mut p) => { p.insert(did.clone()); }
            Err(_) => return Err(ServerFnError::ServerError("pending store locked".into())),
        }
        return Ok(SignInResult { token, needs_approval: true });
    }
    Ok(SignInResult { token, needs_approval: false })
}

/// Generate a peer identity on the spot and continue signed in with it.
/// The new identity holds no delegations: the owner grants them.
/// The display name and avatar arrive with the creation flow and persist
/// in the Willow user list below.
#[server]
pub async fn create_identity(display_name: String, avatar: String) -> Result<NewIdentity, ServerFnError> {
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    let display_name = display_name.trim().to_string();
    if display_name.is_empty() {
        return Err(ServerFnError::ServerError("Give the identity a display name.".into()));
    }
    if avatar.len() > 262144 {
        return Err(ServerFnError::ServerError("That avatar image is too large.".into()));
    }
    if users_empty().await && site_flags().await.disable_new_users {
        // First identity bootstraps the mesh even while creation is paused.
    } else if site_flags().await.disable_new_users && !users_empty().await {
        return Err(ServerFnError::ServerError("Adding identities is paused by the owner.".into()));
    }

    let signing_key = SigningKey::generate(&mut OsRng);
    let public_key = signing_key.verifying_key();
    let did = format!("did:peer:2.Vz{}", bs58::encode(public_key.as_bytes()).into_string());
    let secret = bs58::encode(signing_key.to_bytes()).into_string();

    let token = new_token();
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    if let Err(e) = identity_store::insert_session(&pool, &token, &did).await {
        return Err(ServerFnError::ServerError(format!("could not store session: {e}")));
    }
    if let Err(e) = identity_store::insert_identity(
        &pool,
        &did,
        "exotalk",
        None,
        Some(&display_name),
        if avatar.is_empty() { None } else { Some(&avatar) },
    ).await {
        return Err(ServerFnError::ServerError(format!("could not store identity: {e}")));
    }
    Ok(NewIdentity { did, secret, token })
}

/// Drop a session token.
#[server]
pub async fn sign_out(token: String) -> Result<(), ServerFnError> {
    let pool = match store_pool() {
        Some(p) => p,
        None => return Ok(()),
    };
    let _ = identity_store::delete_session(&pool, &token).await;
    Ok(())
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

#[cfg(target_arch = "wasm32")]
const SAVED_SECRET_KEY: &str = "exo_saved_secret";

/// Store a saved secret for an identity on this device.
/// Stored in browser localStorage at the user's explicit choice.
#[cfg(target_arch = "wasm32")]
pub fn store_saved_secret(did: &str, secret: &str) {
    let key = format!("{SAVED_SECRET_KEY}:{did}");
    let _ = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .map(|s| s.set_item(&key, secret));
}

/// Read a saved secret for an identity from this device.
/// Returns None when none was saved.
#[cfg(target_arch = "wasm32")]
pub fn read_saved_secret(did: &str) -> Option<String> {
    let key = format!("{SAVED_SECRET_KEY}:{did}");
    web_sys::window()?
        .local_storage()
        .ok()?
        .unwrap()
        .get_item(&key)
        .ok()?
}

/// Clear a saved secret for an identity from this device.
#[cfg(target_arch = "wasm32")]
pub fn clear_saved_secret(did: &str) {
    let key = format!("{SAVED_SECRET_KEY}:{did}");
    let _ = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .map(|s| s.remove_item(&key));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn store_saved_secret(_did: &str, _secret: &str) {}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_saved_secret(_did: &str) -> Option<String> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
pub fn clear_saved_secret(_did: &str) {}

/// Persist a small named boolean choice (e.g. a checkbox) in browser
/// storage so it survives reloads. Server builds get inert stubs.
#[cfg(target_arch = "wasm32")]
pub fn store_flag(key: &str, value: bool) {
    let _ = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .map(|s| s.set_item(key, if value { "true" } else { "false" }));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn store_flag(_key: &str, _value: bool) {}

/// Read back a choice stored by `store_flag`. False when absent.
#[cfg(target_arch = "wasm32")]
pub fn read_flag(key: &str) -> bool {
    web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|s| s.get_item(key).ok().flatten())
        .map(|v| v == "true")
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_flag(_key: &str) -> bool {
    false
}

/// Everything below runs server-side against the capability store.
/// The operator tools and the console's privileged sections need the Admin
/// delegation. The owner named in OWNER_DID holds them on the bootstrap
/// path, while the delegation store is empty.
#[cfg(feature = "ssr")]
async fn caller_did(token: &str) -> Option<String> {
    let pool = store_pool()?;
    identity_store::get_session(&pool, token).await.ok().flatten()
}

#[cfg(feature = "ssr")]
async fn level_of(did: &str) -> Option<exocore::protocol_internal::PermissionLevel> {
    let pool = store_pool()?;
    let ns_hex = bs58::encode(updates_namespace()).into_string();
    match identity_store::get_delegation(&pool, &ns_hex, did).await.ok().flatten() {
        Some(d) => match d.permission.as_str() {
            "read" => Some(exocore::protocol_internal::PermissionLevel::Read),
            "write" => Some(exocore::protocol_internal::PermissionLevel::Write),
            "admin" => Some(exocore::protocol_internal::PermissionLevel::Admin),
            _ => None,
        },
        None => {
            if bootstrap_owner(&did).await {
                Some(exocore::protocol_internal::PermissionLevel::Admin)
            } else {
                None
            }
        }
    }
}

#[cfg(feature = "ssr")]
async fn bootstrap_owner(did: &str) -> bool {
    if !is_owner_did(did) {
        return false;
    }
    let pool = match store_pool() {
        Some(p) => p,
        None => return true,
    };
    identity_store::delegations_empty(&pool).await.unwrap_or(true)
}

/// The operator tools and the console's privileged sections need the Admin
/// delegation. The owner named in OWNER_DID holds them on the bootstrap
/// path, while the delegation store is empty.
#[cfg(feature = "ssr")]
pub async fn require_admin(token: &str) -> Result<(), ServerFnError> {
    let Some(did) = caller_did(token).await else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    if matches!(
        level_of(&did).await,
        Some(exocore::protocol_internal::PermissionLevel::Admin)
    ) {
        return Ok(());
    }
    if bootstrap_owner(&did).await {
        return Ok(());
    }
    Err(ServerFnError::ServerError(
        "This section needs the Admin delegation.".into(),
    ))
}

/// Gate check behind a token: who is it, and does a delegation cover models?
#[server]
pub async fn gate_check(token: String) -> Result<Option<GateView>, ServerFnError> {
    let Some(did) = caller_did(&token).await else {
        return Ok(None);
    };
    let models = matches!(
        level_of(&did).await,
        Some(exocore::protocol_internal::PermissionLevel::Read)
            | Some(exocore::protocol_internal::PermissionLevel::Write)
            | Some(exocore::protocol_internal::PermissionLevel::Admin)
    );
    let admin = matches!(
        level_of(&did).await,
        Some(exocore::protocol_internal::PermissionLevel::Admin)
    ) || bootstrap_owner(&did).await;
    if !models {
        if let Ok(mut p) = PENDING.lock() {
            p.insert(did.clone());
        }
    }
    Ok(Some(GateView { did, models: models, admin }))
}

/// Docs gate behind a token: who is it, and is it known to the store?
#[server]
pub async fn docs_content(token: String) -> Result<DocView, ServerFnError> {
    let Some(did) = caller_did(&token).await else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    let known = identity_store::get_identity(&pool, &did).await.map(|r| r.is_some()).unwrap_or(false);
    Ok(DocView { did, approved: known })
}

/// Waiting identities for the owner tools page. Caller must hold Admin.
#[server]
pub async fn pending_list(token: String) -> Result<Vec<PendingIdentity>, ServerFnError> {
    let Some(did) = caller_did(&token).await else {
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

/// Record a petition for access from an identity that proved secret ownership
/// but is not yet known to this node. The owner must approve before sign-in
/// is complete.
#[server]
pub async fn petition_access(did: String, display_name: String) -> Result<(), ServerFnError> {
    let did = did.trim().to_string();
    if !did_format_ok(&did) {
        return Err(ServerFnError::ServerError("That is not a did:peer identity.".into()));
    }
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    let id = format!("pet:{}:{}", did, now_epoch());
    let _ = identity_store::upsert_petition(&pool, &id, &did, if display_name.is_empty() { None } else { Some(&display_name) }).await;
    if let Ok(mut p) = PENDING.lock() {
        p.insert(did.clone());
    }
    Ok(())
}

/// All pending petitions for the owner tools page. Caller must hold Admin.
#[server]
pub async fn list_petitions(token: String) -> Result<Vec<PetitionRow>, ServerFnError> {
    let Some(did) = caller_did(&token).await else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    if !matches!(level_of(&did).await, Some(exocore::protocol_internal::PermissionLevel::Admin)) {
        return Err(ServerFnError::ServerError("Owner tools need the Admin delegation.".into()));
    }
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    identity_store::list_petitions(&pool).await
        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("could not list petitions: {e}")) })
}

/// Approve a petition: add the identity to the store and grant it access.
/// Caller must hold Admin, except the bootstrapped owner on an empty store.
#[server]
pub async fn approve_petition(token: String, did: String) -> Result<(), ServerFnError> {
    let Some(caller) = caller_did(&token).await else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let did = did.trim().to_string();
    let admin = matches!(level_of(&caller).await, Some(exocore::protocol_internal::PermissionLevel::Admin));
    let bootstrap = match std::env::var("OWNER_DID") {
        Ok(owner) => owner.split(',').any(|o| o.trim() == caller),
        Err(_) => false,
    };
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    let empty = match identity_store::list_identities(&pool).await {
        Ok(list) => list.is_empty(),
        Err(_) => true,
    };
    if !(admin || (bootstrap && empty)) {
        return Err(ServerFnError::ServerError("Owner tools need the Admin delegation.".into()));
    }
    // Add the identity to the store as known
    if identity_store::get_identity(&pool, &did).await.map(|r| r.is_some()).unwrap_or(false) {
        return Err(ServerFnError::ServerError("Identity is already known.".into()));
    }
    identity_store::insert_identity(&pool, &did, "exotalk", None, None, None).await
        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("could not approve identity: {e}")) })?;
    identity_store::resolve_petition(&pool, &did, "approved").await
        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("could not resolve petition: {e}")) })?;
    if let Ok(mut p) = PENDING.lock() {
        p.remove(&did);
    }
    Ok(())
}

/// Refuse a petition. Caller must hold Admin.
#[server]
pub async fn refuse_petition(token: String, did: String) -> Result<(), ServerFnError> {
    let Some(caller) = caller_did(&token).await else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let did = did.trim().to_string();
    if !matches!(level_of(&caller).await, Some(exocore::protocol_internal::PermissionLevel::Admin)) {
        return Err(ServerFnError::ServerError("Owner tools need the Admin delegation.".into()));
    }
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    identity_store::resolve_petition(&pool, &did, "refused").await
        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("could not resolve petition: {e}")) })?;
    if let Ok(mut p) = PENDING.lock() {
        p.remove(&did);
    }
    Ok(())
}

/// Grant a delegation. Caller must hold Admin, except the bootstrapped owner
/// named in OWNER_DID on an empty store. Signed by the node identity.
#[server]
pub async fn grant_access(token: String, delegatee: String, level: String) -> Result<(), ServerFnError> {
    let Some(did) = caller_did(&token).await else {
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
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    let ns_hex = bs58::encode(updates_namespace()).into_string();
    if let Err(e) = identity_store::upsert_delegation(
        &pool,
        &format!("{}:{}", ns_hex, delegatee),
        &node_did,
        &delegatee,
        &ns_hex,
        match want {
            exocore::protocol_internal::PermissionLevel::Read => "read",
            exocore::protocol_internal::PermissionLevel::Write => "write",
            exocore::protocol_internal::PermissionLevel::Admin => "admin",
        },
        &cap.signature,
        None,
    ).await {
        return Err(ServerFnError::ServerError(format!("could not persist delegation: {e}")));
    }
    if let Ok(mut p) = PENDING.lock() {
        p.remove(&delegatee);
    }
    Ok(())
}

/// Revoke a delegation. Caller must hold Admin.
#[server]
pub async fn revoke_access(token: String, delegatee: String) -> Result<(), ServerFnError> {
    let Some(did) = caller_did(&token).await else {
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
    let Some(did) = caller_did(&token).await else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let pool = match store_pool() {
        Some(p) => p,
        None => return Ok(VisitorSettings::default()),
    };
    let appearance = identity_store::get_site_setting(&pool, &did, "appearance").await.unwrap_or_default();
    let width = identity_store::get_site_setting(&pool, &did, "width").await.unwrap_or_default();
    Ok(VisitorSettings {
        appearance: appearance.unwrap_or_default(),
        width: width.unwrap_or_default(),
    })
}

/// Save the visitor's server-side settings copy. Signed-in visitors only.
/// The browser copy stays primary; this copy rides the Willow namespace on sync.
#[server]
pub async fn save_settings(token: String, settings: VisitorSettings) -> Result<(), ServerFnError> {
    let Some(did) = caller_did(&token).await else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    if let Err(e) = identity_store::set_site_setting(&pool, &did, "appearance", &settings.appearance).await {
        return Err(ServerFnError::ServerError(format!("could not save settings: {e}")));
    }
    if let Err(e) = identity_store::set_site_setting(&pool, &did, "width", &settings.width).await {
        return Err(ServerFnError::ServerError(format!("could not save settings: {e}")));
    }
    Ok(())
}

/// Willow userland: the source of truth for who can sign in.
///
/// ExoAuth itself only proves secret ownership for a presented identity.
/// Delegations are not part of that flow; they live per user in settings
/// dataland. The user list below is the Willow user namespace carried by
/// this site: every identity that ever signed in or was created here is
/// checked in, with its display name and avatar. The identity chip
/// dropdown reads this list, not the browser's session stash.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub display_name: String,
    pub avatar: String,
    pub created_ms: u64,
}

/// Public identity row for the sign-in and chip dropdowns.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub did: String,
    pub display_name: String,
    pub has_avatar: bool,
}

/// Mesh-wide site flags, owner-managed. `disable_new_users` defaults to
/// false: with creation open, signed-in users read and update; creating
/// and deleting identities pauses while the flag is on, for every
/// subscriber of this site, until the owner turns it back off.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteFlags {
    pub disable_new_users: bool,
}

impl Default for SiteFlags {
    fn default() -> Self {
        SiteFlags { disable_new_users: false }
    }
}

#[cfg(feature = "ssr")]
async fn users_empty() -> bool {
    match store_pool() {
        Some(pool) => identity_store::identities_empty(&pool).await.unwrap_or(true),
        None => true,
    }
}

#[cfg(feature = "ssr")]
async fn site_flags() -> SiteFlags {
    let pool = match store_pool() {
        Some(p) => p,
        None => return SiteFlags::default(),
    };
    let disabled = match identity_store::get_site_flag(&pool, "disable_new_users").await {
        Ok(Some(v)) => v == "true",
        _ => false,
    };
    SiteFlags { disable_new_users: disabled }
}

#[cfg(feature = "ssr")]
fn owner_did_list() -> Vec<String> {
    match std::env::var("OWNER_DID") {
        Ok(v) => v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
        Err(_) => vec![],
    }
}

#[cfg(feature = "ssr")]
fn is_owner_did(did: &str) -> bool {
    owner_did_list().iter().any(|o| o == did)
}

#[cfg(feature = "ssr")]
async fn upsert_profile(did: &str, display_name: &str, avatar: &str) -> Result<(), sqlx::Error> {
    let pool = store_pool().ok_or_else(|| sqlx::Error::RowNotFound)?;
    identity_store::upsert_identity(
        &pool,
        did,
        if display_name.is_empty() { None } else { Some(display_name) },
        if avatar.is_empty() { None } else { Some(avatar) },
    ).await
}

/// Identities the local Willow store knows, newest first.
#[server]
pub async fn users_list() -> Result<Vec<PublicUser>, ServerFnError> {
    let pool = match store_pool() {
        Some(p) => p,
        None => return Ok(vec![]),
    };
    identity_store::list_identities(&pool).await
        .map(|list| list.into_iter().map(|u| PublicUser {
            did: u.did,
            display_name: u.display_name.unwrap_or_default(),
            has_avatar: !u.avatar.clone().unwrap_or_default().is_empty(),
        }).collect())
        .map_err(|e| ServerFnError::ServerError(format!("could not list identities: {e}")))
}

/// Avatar image for a known identity. Empty when none was set.
#[server]
pub async fn user_avatar(did: String) -> Result<String, ServerFnError> {
    let pool = match store_pool() {
        Some(p) => p,
        None => return Ok(String::new()),
    };
    identity_store::get_identity(&pool, &did).await
        .map(|r| r.and_then(|i| i.avatar).unwrap_or_default())
        .map_err(|e| ServerFnError::ServerError(format!("could not read avatar: {e}")))
}

/// Update your own display name and avatar. Reading and updating stay
/// open to every signed-in user at all times.
#[server]
pub async fn update_profile(token: String, display_name: String, avatar: String) -> Result<(), ServerFnError> {
    let Some(did) = caller_did(&token).await else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let display_name = display_name.trim().to_string();
    if display_name.is_empty() {
        return Err(ServerFnError::ServerError("The display name cannot stay empty.".into()));
    }
    if avatar.len() > 262144 {
        return Err(ServerFnError::ServerError("That avatar image is too large.".into()));
    }
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    if let Err(e) = identity_store::upsert_identity(
        &pool,
        &did,
        Some(&display_name),
        if avatar.is_empty() { None } else { Some(&avatar) },
    ).await {
        return Err(ServerFnError::ServerError(format!("could not update profile: {e}")));
    }
    Ok(())
}

/// Remove an identity from the identity store. Removing pauses with
/// creation while the owner flag is on, except for the owner.
#[server]
pub async fn delete_identity(token: String, did: String) -> Result<(), ServerFnError> {
    let Some(caller) = caller_did(&token).await else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    let did = did.trim().to_string();
    let owner = is_owner_did(&caller);
    if caller != did && !owner {
        return Err(ServerFnError::ServerError("You can only remove your own identity.".into()));
    }
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    let flags = site_flags().await;
    if flags.disable_new_users && !owner {
        return Err(ServerFnError::ServerError("Removing identities is paused by the owner.".into()));
    }
    if let Err(e) = identity_store::delete_identity(&pool, &did).await {
        return Err(ServerFnError::ServerError(format!("could not remove identity: {e}")));
    }
    let _ = identity_store::delete_sessions_for_did(&pool, &did).await;
    Ok(())
}

/// Read the mesh-wide site flags.
#[server]
pub async fn get_flags() -> Result<SiteFlags, ServerFnError> {
    Ok(site_flags().await)
}

/// Set the mesh-wide site flags. Owner identities only.
#[server]
pub async fn set_flags(token: String, flags: SiteFlags) -> Result<(), ServerFnError> {
    let Some(caller) = caller_did(&token).await else {
        return Err(ServerFnError::ServerError("No session. Sign in first.".into()));
    };
    if !is_owner_did(&caller) {
        return Err(ServerFnError::ServerError("Only the owner changes site flags.".into()));
    }
    let pool = match store_pool() {
        Some(p) => p,
        None => return Err(ServerFnError::ServerError("identity store not ready".into())),
    };
    if let Err(e) = identity_store::set_site_flag(&pool, "disable_new_users", if flags.disable_new_users { "true" } else { "false" }).await {
        return Err(ServerFnError::ServerError(format!("could not save flags: {e}")));
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
const SESSIONS_KEY: &str = "exo_sessions";
#[cfg(target_arch = "wasm32")]
const CURRENT_KEY: &str = "exo_current";

/// Browser multi-session stash: every signed-in identity keeps its token
/// keyed by did, so the chip switches between live sessions without
/// asking for secrets again. Server builds get inert stubs.
#[cfg(target_arch = "wasm32")]
fn session_map() -> std::collections::HashMap<String, String> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|s| s.get_item(SESSIONS_KEY).ok())
        .flatten()
        .and_then(|j| serde_json::from_str(&j).ok())
        .unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
fn write_session_map(map: &std::collections::HashMap<String, String>) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
        if let Ok(json) = serde_json::to_string(map) {
            let _ = storage.set_item(SESSIONS_KEY, &json);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn stash_session(did: &str, token: &str) {
    let mut map = session_map();
    map.insert(did.to_string(), token.to_string());
    write_session_map(&map);
    store_token(token);
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
        let _ = storage.set_item(CURRENT_KEY, did);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn stash_session(_did: &str, _token: &str) {}

/// Token for the currently active identity.
#[cfg(target_arch = "wasm32")]
pub fn active_token() -> Option<String> {
    read_token()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn active_token() -> Option<String> {
    None
}

/// Did of the currently active identity, if the stash knows it.
#[cfg(target_arch = "wasm32")]
pub fn active_did() -> Option<String> {
    let token = read_token()?;
    session_map().into_iter().find(|(_, t)| t == &token).map(|(d, _)| d)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn active_did() -> Option<String> {
    None
}

/// Every identity holding a live browser session.
#[cfg(target_arch = "wasm32")]
pub fn live_sessions() -> Vec<(String, String)> {
    session_map().into_iter().collect()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn live_sessions() -> Vec<(String, String)> {
    vec![]
}

/// Switch the active identity to another live session.
#[cfg(target_arch = "wasm32")]
pub fn switch_session(did: &str) -> bool {
    match session_map().get(did).cloned() {
        Some(token) => {
            store_token(&token);
            if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
                let _ = storage.set_item(CURRENT_KEY, did);
            }
            true
        }
        None => false,
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn switch_session(_did: &str) -> bool {
    false
}

/// Drop one identity's browser session. Clears the active token when it
/// was the one in use.
#[cfg(target_arch = "wasm32")]
pub fn drop_session(did: &str) {
    let mut map = session_map();
    let removed = map.remove(did);
    write_session_map(&map);
    if let Some(token) = removed {
        if read_token().as_deref() == Some(token.as_str()) {
            clear_token();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn drop_session(_did: &str) {}

/// Display name for a known identity. Empty when unknown.
#[server]
pub async fn user_display_name(did: String) -> Result<String, ServerFnError> {
    let pool = match store_pool() {
        Some(p) => p,
        None => return Ok(String::new()),
    };
    identity_store::get_identity(&pool, &did).await
        .map(|r| r.and_then(|i| i.display_name).unwrap_or_default())
        .map_err(|e| ServerFnError::ServerError(format!("could not read display name: {e}")))
}
