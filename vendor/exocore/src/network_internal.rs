// =============================================================================
// network_internal.rs — Iroh Node Lifecycle & P2P Engine
// =============================================================================
//
// This module manages the Iroh stack for network connectivity.
//
// Architecture:
//   IrohNode (The persistent engine)
//     ├── endpoint  — The connection (NAT traversal / QUIC)
//     ├── blobs     — Storage (message payloads)
//     ├── gossip    — Gossip (broadcasts alerts to peers)
//     └── topics    — A map of active subscriptions
//
// Capability management (Meadowcap):
//   Peers provide signed tokens to verify permission levels
//   (Read/Write/Admin) in memory.
//
// See also: docs/spec/11_meadowcap_capabilities.md
// =============================================================================

use futures_lite::StreamExt;
use iroh::PublicKey;
use iroh_gossip::api as gossip_api;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;

// (Removed unused crate::runtime import)
/// Static in-memory storage of verified access capabilities.
/// Maps [Namespace Hash] -> { [Delegatee DID] -> PermissionLevel }
pub static CAPABILITY_STORE: Lazy<
    RwLock<HashMap<[u8; 32], HashMap<String, crate::protocol_internal::PermissionLevel>>>,
> = Lazy::new(|| RwLock::new(HashMap::new()));

async fn save_capabilities() {
    let store = CAPABILITY_STORE.read().await;
    let path = std::path::Path::new("exostorage").join("capabilities.json");
    if let Ok(json) = serde_json::to_string(&*store) {
        let _ = std::fs::write(path, json);
    }
}

async fn load_capabilities() {
    let path = std::path::Path::new("exostorage").join("capabilities.json");
    if let Ok(json) = std::fs::read_to_string(path) {
        if let Ok(data) = serde_json::from_str::<
            HashMap<[u8; 32], HashMap<String, crate::protocol_internal::PermissionLevel>>,
        >(&json)
        {
            let mut store = CAPABILITY_STORE.write().await;
            *store = data;
            tracing::info!("Restored {} capability namespaces from disk", store.len());
        }
    }
}

async fn dispatch_alpn(
    alpn: &[u8],
    conn: iroh::endpoint::Connection,
    remote_id: iroh::EndpointId,
    gossip: &iroh_gossip::net::Gossip,
    blobs: &iroh_blobs::api::Store,
) -> Result<(), String> {
    if alpn == iroh_gossip::ALPN {
        gossip.handle_connection(conn).await.map_err(|e| e.to_string())?;
        return Ok(());
    }
    if alpn == iroh_blobs::ALPN {
        iroh_blobs::provider::handle_connection(
            conn,
            blobs.clone(),
            iroh_blobs::provider::events::EventSender::DEFAULT,
        )
        .await;
        return Ok(());
    }

    match alpn {
        crate::alpn::exotalk::GOSSIP
        | crate::alpn::exonomy::GOSSIP
        | crate::alpn::exocracy::GOSSIP
        | crate::alpn::republet::GOSSIP
        | crate::alpn::conscia::GOSSIP => {
            gossip.handle_connection(conn).await.map_err(|e| e.to_string())?;
        }
        crate::alpn::exotalk::BLOBS
        | crate::alpn::exonomy::BLOBS
        | crate::alpn::exocracy::BLOBS
        | crate::alpn::republet::BLOBS
        | crate::alpn::conscia::BLOBS => {
            iroh_blobs::provider::handle_connection(
                conn,
                blobs.clone(),
                iroh_blobs::provider::events::EventSender::DEFAULT,
            )
            .await;
        }
        crate::alpn::exotalk::STREAM => {
            tracing::info!("Exotalk stream connection from {:?}", remote_id);
        }
        crate::alpn::conscia::AUTH => {
            tracing::info!("Conscia auth connection from {:?}", remote_id);
        }
        _ => {
            tracing::warn!("Unhandled ALPN: {:?}", String::from_utf8_lossy(alpn));
        }
    }
    Ok(())
}

/// Tracker for the associated relay
static ASSOCIATED_RELAY: Lazy<RwLock<Option<PublicKey>>> = Lazy::new(|| RwLock::new(None));

/// Global store of pending join requests (node IDs).
pub static PENDING_REQUESTS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(Vec::new()));

/// Cached secret key for the node (used for signing delegations)
static NODE_SECRET: Lazy<RwLock<Option<String>>> = Lazy::new(|| RwLock::new(None));

/// Cached DID for the node (used for discovering and signing)
// EDUCATIONAL CONTEXT: Identity Propagation
/// When the daemon boots, it reads its DID from config.toml and
/// stores it here. Every subsystem that needs to sign, delegate, or identify
/// itself reads from this source of truth instead of constructing ad-hoc identity strings.
static NODE_DID: Lazy<RwLock<Option<String>>> = Lazy::new(|| RwLock::new(None));

/// Returns the node's actual DID for external consumption (e.g., HTTP API).
pub async fn get_node_did() -> Option<String> {
    NODE_DID.read().await.clone()
}

pub struct IrohNode {
    endpoint: iroh::Endpoint,
    blobs: iroh_blobs::api::Store,
    gossip: iroh_gossip::net::Gossip,
    topics: Arc<RwLock<HashMap<[u8; 32], gossip_api::GossipSender>>>,
}

static NODE: Lazy<RwLock<Option<Arc<IrohNode>>>> = Lazy::new(|| RwLock::new(None));
static NODE_PROFILE_ROOT: Lazy<RwLock<Option<std::path::PathBuf>>> =
    Lazy::new(|| RwLock::new(None));
static INGRESS_PAUSED: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(false));
static EGRESS_PAUSED: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(false));
static PEERS: Lazy<RwLock<HashMap<iroh::EndpointId, ()>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
static TOPIC_LATEST_BLOB_HASH: Lazy<RwLock<HashMap<[u8; 32], String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn get_latest_blob_hash(namespace: [u8; 32]) -> Option<String> {
    TOPIC_LATEST_BLOB_HASH.read().await.get(&namespace).cloned()
}

pub async fn get_willow_head(namespace: [u8; 32]) -> Result<Option<String>, String> {
    let Some(profile_root) = NODE_PROFILE_ROOT.read().await.clone() else {
        return Ok(None);
    };
    let head_path = willow_head_path(&profile_root, namespace);
    let value = match std::fs::read_to_string(head_path) {
        Ok(head) => head.trim().to_string(),
        Err(_) => return Ok(None),
    };
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

pub async fn set_willow_head(namespace: [u8; 32], head_hash: String) -> Result<(), String> {
    let Some(profile_root) = NODE_PROFILE_ROOT.read().await.clone() else {
        return Err("Iroh node not initialized".to_string());
    };
    let head_path = willow_head_path(&profile_root, namespace);
    if let Some(parent) = head_path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let temp_path = head_path.with_extension("tmp");
    std::fs::write(&temp_path, head_hash.as_bytes()).map_err(|error| error.to_string())?;
    std::fs::rename(&temp_path, &head_path).map_err(|error| error.to_string())?;
    Ok(())
}

fn willow_head_path(profile_root: &std::path::Path, namespace: [u8; 32]) -> std::path::PathBuf {
    let safe_namespace = bs58::encode(namespace).into_string();
    profile_root
        .join("willow_heads")
        .join(format!("{safe_namespace}.head"))
}

pub async fn set_ingress_paused(paused: bool) {
    let mut mode = INGRESS_PAUSED.write().await;
    *mode = paused;
}

pub async fn is_ingress_paused() -> bool {
    let mode = INGRESS_PAUSED.read().await;
    *mode
}

pub async fn set_egress_paused(paused: bool) {
    let mut mode = EGRESS_PAUSED.write().await;
    *mode = paused;
}

pub async fn is_egress_paused() -> bool {
    let mode = EGRESS_PAUSED.read().await;
    *mode
}

pub async fn set_associated_relay(node_id: String) -> Result<(), String> {
    let key = PublicKey::from_str(&node_id).map_err(|e| e.to_string())?;
    {
        let mut p = ASSOCIATED_RELAY.write().await;
        *p = Some(key);
    }

    let node_opt = NODE.read().await;
    if let Some(node) = node_opt.as_ref() {
        let mut addr = iroh::EndpointAddr::new(key);
        if let Ok(relay) = "https://euw1-1.relay.iroh.network./".parse() {
            addr = addr.with_relay_url(relay);
        }
        let ep = node.endpoint.clone();
        tokio::spawn(async move {
            tracing::info!("Dialing associated relay node: {}", key);
            if let Ok(conn) = ep.connect(addr, iroh_gossip::ALPN).await {
                let mut peers = PEERS.write().await;
                peers.insert(conn.remote_id(), ());
            }
        });
    }

    Ok(())
}

/// Forces an immediate cryptographic connection with the associated relay node.
pub async fn initiate_connection() -> Result<(), String> {
    let target = {
        let p = ASSOCIATED_RELAY.read().await;
        *p
    };

    if let Some(key) = target {
        let node_opt = NODE.read().await;
        if let Some(node) = node_opt.as_ref() {
            let mut addr = iroh::EndpointAddr::new(key);
            if let Ok(relay) = "https://euw1-1.relay.iroh.network./".parse() {
                addr = addr.with_relay_url(relay);
            }
            let ep = node.endpoint.clone();
            tokio::spawn(async move {
                tracing::info!("Manually triggered dial to associated relay node: {}", key);
                if let Ok(conn) = ep.connect(addr, iroh_gossip::ALPN).await {
                    let mut peers = PEERS.write().await;
                    peers.insert(conn.remote_id(), ());
                }
            });
            Ok(())
        } else {
            Err("Network not initialized".to_string())
        }
    } else {
        Err("No associated relay node set".to_string())
    }
}

pub async fn get_relay_status() -> (Option<String>, bool, u32) {
    let target = {
        let p = ASSOCIATED_RELAY.read().await;
        *p
    };
    let node_opt = NODE.read().await;

    if node_opt.is_some() {
        let peers = PEERS.read().await;
        let peer_count = peers.len() as u32;
        let is_online = target.map(|t| peers.contains_key(&t)).unwrap_or(false);
        (target.map(|t| t.to_string()), is_online, peer_count)
    } else {
        (target.map(|t| t.to_string()), false, 0)
    }
}

pub async fn get_peer_list() -> Vec<(String, Vec<String>)> {
    let node_lock = NODE.read().await;
    if let Some(n) = node_lock.as_ref() {
        let peer_ids: Vec<iroh::EndpointId> = {
            let peers = PEERS.read().await;
            peers.keys().cloned().collect()
        };
        let mut out = Vec::with_capacity(peer_ids.len());
        for peer_id in peer_ids {
            let mut addrs = Vec::new();
            if let Some(info) = n.endpoint.remote_info(peer_id).await {
                addrs = info.addrs().map(|a| a.addr().to_string()).collect();
            }
            out.push((peer_id.to_string(), addrs));
        }
        out
    } else {
        vec![]
    }
}

pub async fn join_conversation_topic(namespace: [u8; 32]) -> Result<(), String> {
    let node_lock = NODE.read().await;
    let n = node_lock.as_ref().ok_or("Network not initialized")?;

    let topic_id = iroh_gossip::proto::TopicId::from(namespace);

    {
        let topics = n.topics.read().await;
        if topics.contains_key(&namespace) {
            return Ok(());
        }
    }

    let topic = n
        .gossip
        .subscribe(topic_id, vec![])
        .await
        .map_err(|e| e.to_string())?;
    let (sender, mut receiver) = topic.split();

    {
        let mut topics = n.topics.write().await;
        topics.insert(namespace, sender);
    }

    tokio::spawn(async move {
        while let Some(Ok(event)) = receiver.next().await {
            if is_ingress_paused().await {
                continue; // Ignore gossip processing if ingress is paused
            }

            if let gossip_api::Event::Received(msg) = event {
                let msg_content = msg.content.clone();
                if msg_content.len() == 32 {
                    let bytes: [u8; 32] = match msg_content.to_vec().try_into() {
                        Ok(value) => value,
                        Err(_) => continue,
                    };
                    let hash = iroh_blobs::Hash::from(bytes);
                    let hash_string = hash.to_string();
                    TOPIC_LATEST_BLOB_HASH
                        .write()
                        .await
                        .insert(namespace, hash_string.clone());
                    let _ = set_willow_head(namespace, hash_string).await;
                    continue;
                }
                if let Ok(json_str) = String::from_utf8(msg_content.to_vec()) {
                    // 1. Handle Join Requests
                    if json_str.contains("node_id") && json_str.contains("timestamp_ms") {
                        if let Ok(req) =
                            serde_json::from_str::<crate::protocol_internal::JoinRequest>(&json_str)
                        {
                            let mut pending = PENDING_REQUESTS.write().await;
                            if !pending.contains(&req.node_id) {
                                pending.push(req.node_id.clone());
                                tracing::info!("New JoinRequest queued for {}", req.node_id);
                            }
                            continue;
                        }
                    }

                    if json_str.contains("delegator")
                        && json_str.contains("delegatee")
                        && json_str.contains("access_level")
                    {
                        if let Ok(cap) =
                            serde_json::from_str::<crate::protocol_internal::Capability>(&json_str)
                        {
                            if cap.verify() {
                                let mut store = CAPABILITY_STORE.write().await;
                                let ns_map =
                                    store.entry(cap.namespace).or_insert_with(HashMap::new);
                                ns_map.insert(cap.delegatee.clone(), cap.access_level.clone());
                                tracing::info!(
                                    "Capability token stored and verified for {}",
                                    cap.delegatee
                                );
                                drop(store);
                                save_capabilities().await;
                            } else {
                                tracing::warn!("Invalid capability token dropped");
                            }
                            continue;
                        }
                    }

                    if json_str.contains("delegator")
                        && json_str.contains("namespace")
                        && !json_str.contains("access_level")
                    {
                        if let Ok(tombstone) = serde_json::from_str::<
                            crate::protocol_internal::RevocationTombstone,
                        >(&json_str)
                        {
                            let mut store = CAPABILITY_STORE.write().await;
                            if let Some(ns_map) = store.get_mut(&tombstone.namespace) {
                                if ns_map.remove(&tombstone.delegatee).is_some() {
                                    tracing::info!(
                                        "Capability revoked via gossip tombstone for {}",
                                        tombstone.delegatee
                                    );
                                    drop(store);
                                    save_capabilities().await;
                                }
                            }
                            continue;
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

pub async fn get_pending_requests() -> Vec<String> {
    PENDING_REQUESTS.read().await.clone()
}

pub async fn get_all_capabilities() -> HashMap<String, String> {
    let store = CAPABILITY_STORE.read().await;
    let mut roles = HashMap::new();
    for ns_map in store.values() {
        for (did, level) in ns_map {
            roles.insert(did.clone(), format!("{:?}", level));
        }
    }
    roles
}

pub async fn authorize_node(node_id: String, role: String) -> Result<(), String> {
    let secret = {
        let s = NODE_SECRET.read().await;
        s.clone().ok_or("Node secret not set")?
    };

    let level = match role.to_lowercase().as_str() {
        "admin" | "owner" => crate::protocol_internal::PermissionLevel::Admin,
        "write" | "writer" | "delegate" => crate::protocol_internal::PermissionLevel::Write,
        _ => crate::protocol_internal::PermissionLevel::Read,
    };

    // Use a default global namespace for mesh-level governance
    let namespace = crate::protocol_internal::derive_namespace("mesh_governance");

    let delegator_did = get_node_did().await.ok_or("Node DID not set")?;

    let cap = crate::protocol_internal::delegate_capability(
        &delegator_did,
        &node_id,
        namespace,
        level.clone(),
        &secret,
    )?;

    // Store locally
    {
        let mut store = CAPABILITY_STORE.write().await;
        let ns_map = store.entry(namespace).or_insert_with(HashMap::new);
        ns_map.insert(node_id.clone(), level);
    }
    save_capabilities().await;

    // Broadcast
    let json = serde_json::to_string(&cap).map_err(|e| e.to_string())?;
    broadcast_raw_gossip(namespace, json).await?;

    // Remove from pending if present
    {
        let mut pending = PENDING_REQUESTS.write().await;
        pending.retain(|id| id != &node_id);
    }

    Ok(())
}

pub async fn revoke_node(node_id: String) -> Result<(), String> {
    let namespace = crate::protocol_internal::derive_namespace("mesh_governance");

    // 1. Remove locally
    {
        let mut store = CAPABILITY_STORE.write().await;
        if let Some(ns_map) = store.get_mut(&namespace) {
            ns_map.remove(&node_id);
        }
    }
    save_capabilities().await;

    // 2. Broadcast Tombstone
    let delegator_did = get_node_did().await.ok_or("Node DID not set")?;

    let tombstone = crate::protocol_internal::RevocationTombstone {
        delegator: delegator_did,
        delegatee: node_id.clone(),
        namespace,
        signature: vec![], // In this version, we don't sign tombstones yet
    };

    let json = serde_json::to_string(&tombstone).map_err(|e| e.to_string())?;
    broadcast_raw_gossip(namespace, json).await?;

    tracing::info!("Revocation broadcasted for node: {}", node_id);
    Ok(())
}

pub async fn request_access() -> Result<(), String> {
    let node_lock = NODE.read().await;
    let n = node_lock.as_ref().ok_or("Network not initialized")?;
    let node_id = n.endpoint.id().to_string();

    let namespace = crate::protocol_internal::derive_namespace("mesh_governance");

    let req = crate::protocol_internal::JoinRequest {
        node_id: node_id.clone(),
        timestamp_ms: chrono::Utc::now().timestamp_millis(),
    };

    let json = serde_json::to_string(&req).map_err(|e| e.to_string())?;

    // Ensure we are subscribed to the topic before broadcasting
    join_conversation_topic(namespace).await?;

    broadcast_raw_gossip(namespace, json).await?;
    tracing::info!("Access request broadcasted for node: {}", node_id);

    Ok(())
}

async fn broadcast_raw_gossip(namespace: [u8; 32], content: String) -> Result<(), String> {
    let node_lock = NODE.read().await;
    let n = node_lock.as_ref().ok_or("Network not initialized")?;
    let topics = n.topics.read().await;
    let sender = topics.get(&namespace).ok_or("Not subscribed to topic")?;
    sender
        .broadcast(content.into_bytes().into())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn shutdown_network() {
    let mut node_lock = NODE.write().await;
    if let Some(n) = node_lock.take() {
        tracing::info!("Shutting down active Iroh node...");
        // Explicitly close the endpoint to terminate the accept loop and release sockets
        let _ = n.endpoint.close().await;
        // In Iroh, dropping the Endpoint and Gossip instances cleans up the background tasks
        drop(n);
    }
}

pub async fn start_iroh_node(did: String, secret_key_b58: String, port_mapping: bool) -> Result<(), String> {
    {
        if NODE.read().await.is_some() {
            return Ok(());
        }
    }

    {
        let mut s = NODE_SECRET.write().await;
        *s = Some(secret_key_b58.clone());
    }

    {
        let mut d = NODE_DID.write().await;
        *d = Some(did.clone());
    }

    load_capabilities().await;

    // Profile-specific storage path
    let safe_did = did.replace(":", "_");
    let profile_root = std::path::PathBuf::from("exostorage")
        .join("profiles")
        .join(&safe_did);
    {
        let mut root = NODE_PROFILE_ROOT.write().await;
        *root = Some(profile_root.clone());
    }
    let storage_path = profile_root.join("blobs");

    std::fs::create_dir_all(&storage_path)
        .map_err(|e| format!("Failed to create storage dir: {}", e))?;

    // Add a small retry loop to allow the previous instance to release the file lock
    let mut blobs_opt = None;
    for i in 0..10 {
        match iroh_blobs::store::fs::FsStore::load(&storage_path).await {
            Ok(s) => {
                blobs_opt = Some(s);
                break;
            }
            Err(e) => {
                if i == 9 {
                    return Err(format!("Failed to load blob store after retries: {}", e));
                }
                tracing::warn!(
                    "Blob store locked, retrying in 100ms... (attempt {})",
                    i + 1
                );
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    }
    let blobs = iroh_blobs::api::Store::from(blobs_opt.unwrap());

    // Bind the Iroh Node ID to the user's secret key
    let sk_bytes = bs58::decode(&secret_key_b58)
        .into_vec()
        .map_err(|e| format!("Invalid secret key b58: {}", e))?;
    let sk_arr: [u8; 32] = sk_bytes
        .try_into()
        .map_err(|_| "Secret key length mismatch".to_string())?;
    let secret_key = iroh::SecretKey::from_bytes(&sk_arr);

    let mut endpoint_builder = iroh::Endpoint::builder(iroh::endpoint::presets::N0);
    endpoint_builder = endpoint_builder
        .secret_key(secret_key)
        .alpns(crate::alpn::all_alpns().iter().map(|a| a.to_vec()).collect());
    
    if !port_mapping {
        endpoint_builder = endpoint_builder.portmapper_config(iroh::endpoint::PortmapperConfig::Disabled);
    }

    let endpoint = endpoint_builder
        .bind()
        .await
        .map_err(|e| format!("Failed to bind Iroh endpoint: {}", e))?;

    let gossip = iroh_gossip::net::Gossip::builder().spawn(endpoint.clone());

    let endpoint_clone = endpoint.clone();
    let gossip_clone = gossip.clone();
    let blobs_clone = blobs.clone();

    tokio::spawn(async move {
        while let Some(incoming) = endpoint_clone.accept().await {
            if is_ingress_paused().await {
                tracing::info!("Ingress paused, ignoring incoming connection");
                continue;
            }

            let mut accepting = match incoming.accept() {
                Ok(a) => a,
                Err(e) => {
                    tracing::error!("Failed to accept incoming: {}", e);
                    continue;
                }
            };
            let alpn = match accepting.alpn().await {
                Ok(a) => a,
                Err(e) => {
                    tracing::error!("Failed to read ALPN: {}", e);
                    continue;
                }
            };

            let conn = match accepting.await {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!("Failed to connect: {}", e);
                    continue;
                }
            };
            let remote_id = conn.remote_id();

            {
                let mut peers = PEERS.write().await;
                peers.insert(remote_id, ());
            }

            let gossip = gossip_clone.clone();
            let blobs = blobs_clone.clone();
            tokio::spawn(async move {
                if let Err(e) = dispatch_alpn(&alpn, conn, remote_id, &gossip, &blobs).await {
                    tracing::error!("ALPN handler error: {}", e);
                }
            });
        }
    });

    let node = Arc::new(IrohNode {
        endpoint,
        blobs,
        gossip,
        topics: Arc::new(RwLock::new(HashMap::new())),
    });

    let mut node_lock = NODE.write().await;
    *node_lock = Some(node.clone());

    // Handshake initialization
    let p = ASSOCIATED_RELAY.read().await;
    if let Some(key) = *p {
        let mut addr = iroh::EndpointAddr::new(key);
        if let Ok(relay) = "https://euw1-1.relay.iroh.network./".parse() {
            addr = addr.with_relay_url(relay);
        }
        let ep = node.endpoint.clone();
        tokio::spawn(async move {
            tracing::info!("Boot dial to associated relay node: {}", key);
            if let Ok(conn) = ep.connect(addr, iroh_gossip::ALPN).await {
                let mut peers = PEERS.write().await;
                peers.insert(conn.remote_id(), ());
            }
        });
    }

    Ok(())
}

pub async fn get_node_id() -> String {
    let node = NODE.read().await;
    if let Some(n) = node.as_ref() {
        n.endpoint.id().to_string()
    } else {
        "Not initialized".to_string()
    }
}

pub async fn save_blob(bytes: Vec<u8>) -> Result<String, String> {
    let node = NODE.read().await;
    let n = node.as_ref().ok_or("Network not initialized")?;

    let outcome = n.blobs.add_bytes(bytes).await.map_err(|e| e.to_string())?;
    Ok(outcome.hash.to_string())
}

pub async fn load_blob(hash_str: String) -> Result<Vec<u8>, String> {
    let node = NODE.read().await;
    let n = node.as_ref().ok_or("Network not initialized")?;

    let hash = hash_str
        .parse::<iroh_blobs::Hash>()
        .map_err(|e| e.to_string())?;

    let mut reader = n.blobs.reader(hash);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await.map_err(|e| e.to_string())?;
    Ok(buf)
}

/// Broadcasts a message hash across the gossip mesh.
pub async fn broadcast_message_hash(namespace: [u8; 32], hash_str: String) -> Result<(), String> {
    if is_egress_paused().await {
        tracing::warn!("Egress paused, skipping gossip broadcast");
        return Ok(());
    }

    let node_lock = NODE.read().await;
    let n = node_lock.as_ref().ok_or("Network not initialized")?;

    let topics = n.topics.read().await;
    let sender = topics
        .get(&namespace)
        .ok_or("Not subscribed to this conversation topic")?;

    let hash = hash_str
        .parse::<iroh_blobs::Hash>()
        .map_err(|e| e.to_string())?;
    sender
        .broadcast(hash.as_bytes().to_vec().into())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_stats() -> HashMap<String, String> {
    let node = NODE.read().await;
    let mut stats = HashMap::new();
    if let Some(n) = node.as_ref() {
        stats.insert("node_id".to_string(), n.endpoint.id().to_string());
        stats.insert("status".to_string(), "Online".to_string());

        // 1. Local Bind (LAN)
        let sockets = n.endpoint.bound_sockets();
        if let Some(addr) = sockets
            .iter()
            .find(|a| a.is_ipv4())
            .or_else(|| sockets.first())
        {
            stats.insert("local_port".to_string(), addr.port().to_string());
        }

        // 2. Relay Connectivity (Internet)
        let home_relay = n
            .endpoint
            .addr()
            .relay_urls()
            .next()
            .map(|url| url.to_string())
            .unwrap_or_else(|| "None".to_string());
        stats.insert("home_relay".to_string(), home_relay);

        // 3. Mesh Peering
        let peers_count = PEERS.read().await.len();
        stats.insert("active_peers".to_string(), peers_count.to_string());

        // 4. Gossip Subscriptions
        let topics = n.topics.read().await;
        stats.insert("topics_count".to_string(), topics.len().to_string());

        // EDUCATIONAL CONTEXT: We renamed 'blob_count' to 'storage_status' to elevate
        // the API from an implementation-detail (blobs) to a functional concept (storage).
        stats.insert("storage_status".to_string(), "Active".to_string());
    } else {
        stats.insert("status".to_string(), "Offline".to_string());
    }
    stats
}
#[cfg(test)]
mod diagnostics {
    use super::*;
    #[tokio::test]
    async fn print_local_node_id() {
        let secret_key_b58 = "CVh7w1QW9GdThiRskD5SYtCyzGLteP4BC6CXR94CF26u";
        let sk_bytes = bs58::decode(&secret_key_b58).into_vec().unwrap();
        let sk_arr: [u8; 32] = sk_bytes.try_into().unwrap();
        let secret_key = iroh::SecretKey::from_bytes(&sk_arr);
        println!("DIAGNOSTIC_NODE_ID: {}", secret_key.public().to_string());
    }
    #[tokio::test]
    async fn check_status() {
        let stats = get_stats().await;
        println!("NETWORK_STATS: {:?}", stats);
        let (id, online, peers) = get_relay_status().await;
        println!(
            "RELAY_STATUS: id={:?} online={} peers={}",
            id, online, peers
        );
    }

    #[tokio::test]
    async fn test_send_actual_request() -> Result<(), String> {
        let did = "did:peer:exocracy_verify".to_string();
        let secret = "CVh7w1QW9GdThiRskD5SYtCyzGLteP4BC6CXR94CF26u".to_string(); // Valid different key
        start_iroh_node(did, secret, true).await?;

        let stats = get_stats().await;
        let local_id = stats.get("node_id").unwrap();
        println!("LOCAL_NODE_ID_FOR_VERIFY: {}", local_id);

        // Connect to LOCAL relay
        let remote_id = "2f4300ae2c116d3c0f87cea35cc0254900a217558878d55010e435e30b0cc9b4";
        set_associated_relay(remote_id.to_string()).await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        for i in 0..10 {
            println!("Sending attempt {}...", i);
            request_access().await?;
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }

        Ok(())
    }
}
