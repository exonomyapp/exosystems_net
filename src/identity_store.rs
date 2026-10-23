//! SQLite identity store for the Exosystems node.
//!
//! Replaces the JSON files (allowed_dids.json, users.json, capabilities.json,
//! site-flags.json, site-settings.json) that previously tracked identities,
//! profiles, delegations, flags, and settings on disk.
//! Boot and sign-in now read the SQLite store instead.

use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

pub use crate::auth::PetitionRow;

/// Path to the SQLite database holding the node identity store.
const IDENTITY_DB: &str = "exostorage/identity.db";

/// Open (creating if necessary) the identity store database and run migrations.
pub async fn open() -> Result<SqlitePool, sqlx::Error> {
    std::fs::create_dir_all("exostorage")?;
    let opts = SqliteConnectOptions::from_str(IDENTITY_DB)?
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(opts).await?;
    migrate(&pool).await?;
    Ok(pool)
}

/// Run schema migrations against the store.
async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

/// Current unix epoch seconds.
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Look up an identity row by DID. Returns None when unknown to the store.
pub async fn get_identity(pool: &SqlitePool, did: &str) -> Result<Option<IdentityRow>, sqlx::Error> {
    let row: Option<IdentityRow> = sqlx::query_as(
        "SELECT did, kind, created_at, binding_proof_id, display_name, avatar FROM identities WHERE did = ?",
    )
    .bind(did)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Insert a new identity row. Returns an error if the DID already exists.
pub async fn insert_identity(
    pool: &SqlitePool,
    did: &str,
    kind: &str,
    binding_proof_id: Option<&str>,
    display_name: Option<&str>,
    avatar: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO identities (did, kind, created_at, binding_proof_id, display_name, avatar) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(did)
    .bind(kind)
    .bind(now().to_string())
    .bind(binding_proof_id)
    .bind(display_name)
    .bind(avatar)
    .execute(pool)
    .await?;
    Ok(())
}

/// Update an existing identity's display name and avatar.
pub async fn upsert_identity(
    pool: &SqlitePool,
    did: &str,
    display_name: Option<&str>,
    avatar: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE identities SET display_name = COALESCE(?, display_name), avatar = COALESCE(?, avatar) WHERE did = ?",
    )
    .bind(display_name)
    .bind(avatar)
    .bind(did)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete an identity row by DID.
pub async fn delete_identity(pool: &SqlitePool, did: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM identities WHERE did = ?")
        .bind(did)
        .execute(pool)
        .await?;
    Ok(())
}

/// All identities in the store, for the identity chip dropdown.
pub async fn list_identities(pool: &SqlitePool) -> Result<Vec<IdentityRow>, sqlx::Error> {
    let rows: Vec<IdentityRow> = sqlx::query_as(
        "SELECT did, kind, created_at, binding_proof_id, display_name, avatar FROM identities ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Whether the store holds any identity. True on a fresh empty store.
pub async fn identities_empty(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM identities")
        .fetch_one(pool)
        .await?;
    Ok(count == 0)
}

/// Look up a delegation for a delegatee within a namespace.
pub async fn get_delegation(
    pool: &SqlitePool,
    namespace: &str,
    delegatee: &str,
) -> Result<Option<DelegationRow>, sqlx::Error> {
    let row: Option<DelegationRow> = sqlx::query_as(
        "SELECT id, delegator, delegatee, namespace, permission, signature, expires_at FROM delegations WHERE namespace = ? AND delegatee = ?",
    )
    .bind(namespace)
    .bind(delegatee)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Insert or replace a delegation row.
pub async fn upsert_delegation(
    pool: &SqlitePool,
    id: &str,
    delegator: &str,
    delegatee: &str,
    namespace: &str,
    permission: &str,
    signature: &[u8],
    expires_at: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT OR REPLACE INTO delegations (id, delegator, delegatee, namespace, permission, signature, expires_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(id)
    .bind(delegator)
    .bind(delegatee)
    .bind(namespace)
    .bind(permission)
    .bind(signature)
    .bind(expires_at)
    .execute(pool)
    .await?;
    Ok(())
}

/// Remove a delegation row by delegatee within a namespace.
pub async fn delete_delegation(
    pool: &SqlitePool,
    namespace: &str,
    delegatee: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM delegations WHERE namespace = ? AND delegatee = ?")
        .bind(namespace)
        .bind(delegatee)
        .execute(pool)
        .await?;
    Ok(())
}

/// All delegations within a namespace.
pub async fn list_delegations(
    pool: &SqlitePool,
    namespace: &str,
) -> Result<Vec<DelegationRow>, sqlx::Error> {
    let rows: Vec<DelegationRow> = sqlx::query_as(
        "SELECT id, delegator, delegatee, namespace, permission, signature, expires_at FROM delegations WHERE namespace = ?",
    )
    .bind(namespace)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Whether the delegations table is empty.
pub async fn delegations_empty(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM delegations")
        .fetch_one(pool)
        .await?;
    Ok(count == 0)
}

/// Look up a binding proof by DID.
pub async fn get_binding_proof(
    pool: &SqlitePool,
    did: &str,
) -> Result<Option<BindingProofRow>, sqlx::Error> {
    let row: Option<BindingProofRow> = sqlx::query_as(
        "SELECT id, did, proof, created_at FROM binding_proofs WHERE did = ?",
    )
    .bind(did)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Insert a binding proof row.
pub async fn insert_binding_proof(
    pool: &SqlitePool,
    id: &str,
    did: &str,
    proof: &[u8],
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO binding_proofs (id, did, proof, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(id)
    .bind(did)
    .bind(proof)
    .bind(now().to_string())
    .execute(pool)
    .await?;
    Ok(())
}

/// Insert or replace a petition row.
pub async fn upsert_petition(
    pool: &SqlitePool,
    id: &str,
    did: &str,
    display_name: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT OR REPLACE INTO petitions (id, did, display_name, created_at, status) VALUES (?, ?, ?, ?, 'pending')",
    )
    .bind(id)
    .bind(did)
    .bind(display_name)
    .bind(now().to_string())
    .execute(pool)
    .await?;
    Ok(())
}

/// All pending petitions, newest first.
pub async fn list_petitions(pool: &SqlitePool) -> Result<Vec<PetitionRow>, sqlx::Error> {
    let rows: Vec<PetitionRow> = sqlx::query_as(
        "SELECT id, did, display_name, created_at, status FROM petitions WHERE status = 'pending' ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Mark a petition as approved or refused.
pub async fn resolve_petition(
    pool: &SqlitePool,
    did: &str,
    status: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE petitions SET status = ? WHERE did = ? AND status = 'pending'",
    )
    .bind(status)
    .bind(did)
    .execute(pool)
    .await?;
    Ok(())
}

/// Read a site flag by key.
pub async fn get_site_flag(pool: &SqlitePool, key: &str) -> Result<Option<String>, sqlx::Error> {
    let row: Option<String> = sqlx::query_scalar("SELECT value FROM site_flags WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

/// Write a site flag by key.
pub async fn set_site_flag(
    pool: &SqlitePool,
    key: &str,
    value: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT OR REPLACE INTO site_flags (key, value) VALUES (?, ?)",
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

/// Read a site setting for an identity by key.
pub async fn get_site_setting(
    pool: &SqlitePool,
    did: &str,
    key: &str,
) -> Result<Option<String>, sqlx::Error> {
    let row: Option<String> = sqlx::query_scalar("SELECT value FROM site_settings WHERE did = ? AND key = ?")
        .bind(did)
        .bind(key)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

/// Write a site setting for an identity by key.
pub async fn set_site_setting(
    pool: &SqlitePool,
    did: &str,
    key: &str,
    value: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT OR REPLACE INTO site_settings (did, key, value) VALUES (?, ?, ?)",
    )
    .bind(did)
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

/// One row from the identities table.
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct IdentityRow {
    pub did: String,
    pub kind: String,
    pub created_at: String,
    pub binding_proof_id: Option<String>,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
}

/// One row from the delegations table.
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DelegationRow {
    pub id: String,
    pub delegator: String,
    pub delegatee: String,
    pub namespace: String,
    pub permission: String,
    pub signature: Vec<u8>,
    pub expires_at: Option<String>,
}

/// One row from the binding_proofs table.
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BindingProofRow {
    pub id: String,
    pub did: String,
    pub proof: Vec<u8>,
    pub created_at: String,
}
