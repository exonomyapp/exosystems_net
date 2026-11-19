# Local Development Port Assignments

## Base: 52415 (E-X-O = 5-24-15)

All local development services use the 52415+ range. Production deployments use standard ports per their profiles.

---

## Port Map

| Port | Service | Protocol | Notes |
|------|---------|----------|-------|
| **52415** | Exosystems.net Leptos Site | HTTP | Primary web UI (`cargo leptos watch --release`) |
| **52416** | Leptos HMR WebSocket | WS | Hot module reload for development |
| **52417** | Boltstore / Models Store | HTTP | `MODEL_UPDATES_STORE` — replaces 8080 |
| **52418** | llama.cpp / LLM Inference | HTTP | `LLAMA_SERVER` — replaces 8090 |
| **52419** | Memento (ExoMCP Memory) | HTTP | Replaces 9001 |
| **52420** | Cognee Embeddings | HTTP | Replaces 61055 |
| **52421** | Telemetry API Sidecar | HTTP | Ollama-compatible — replaces 11434 |
| **52422** | pgEdge / PostgreSQL (ExoMetro) | TCP | Replaces 30432 / 5432 |
| **52423** | Memgraph Bolt | TCP | Replaces 7687 |
| **52424** | ExoMCP Dashboard | HTTP | Replaces 9100 |
| **52425** | ExoMCP MCP Endpoint | HTTP | Streamable HTTP at `/mcp` |
| **52426** | Hermes WhatsApp Bridge | HTTP | Replaces 3099 |
| **52427** | nexo-rs Prometheus Metrics | HTTP | Replaces 9090 |
| **52428** | nexo-rs Health | HTTP | Replaces 8080 (was conflict) |
| **52429** | nexo-rs Admin Console | HTTP | Replaces 8081 |
| **52430** | Conscia Daemon API | HTTP | Primary Conscia API (flavor-agnostic) |
| **52431** | Conscia NATS Event Bus | TCP | Replaces 4222 (internal) |
| **52432** | Conscia NATS Monitoring | HTTP | Replaces 8222 |
| **52433** | Zrok Controller | HTTP | Local zrok agent API |
| **52434** | Exosystems.net Nginx (local) | HTTP/HTTPS | Local TLS termination for testing |
| **52435** | Mailcatcher / SMTP Dev | SMTP/HTTP | Local mail testing (replaces Brevo) |
| **52436** | Reserved | — | Future Conscia flavor-specific service |
| **52437** | Reserved | — | Future Conscia flavor-specific service |
| **52438** | Reserved | — | Future Conscia flavor-specific service |
| **52439** | Reserved | — | Future Conscia flavor-specific service |
| **52440** | Reserved | — | Future Conscia flavor-specific service |

---

## Environment Variable Mapping

| Service | Old Env Var | New Env Var | New Value |
|---------|-------------|-------------|-----------|
| Exosystems.net site | `LEPTOS_SITE_ADDR` | `LEPTOS_SITE_ADDR` | `127.0.0.1:52415` |
| Leptos HMR | `LEPTOS_RELOAD_PORT` | `LEPTOS_RELOAD_PORT` | `52416` |
| Boltstore | `MODEL_UPDATES_STORE` | `MODEL_UPDATES_STORE` | `http://127.0.0.1:52417` |
| llama.cpp | `LLAMA_SERVER` / `CONSCIA_LLM_SOCKET_*` | `LLAMA_SERVER` | `http://127.0.0.1:52418` |
| Memento | `MEMENTO_URL` | `MEMENTO_URL` | `http://127.0.0.1:52419/mcp` |
| Cognee | `COGNEE_EMBEDDING_ENDPOINT` | `COGNEE_EMBEDDING_ENDPOINT` | `http://127.0.0.1:52420/v1` |
| Telemetry | `TELEMETRY_API_URL` | `TELEMETRY_API_URL` | `http://127.0.0.1:52421` |
| pgEdge | `DATABASE_URL` | `DATABASE_URL` | `postgres://...@127.0.0.1:52422/...` |
| Memgraph | `MEMGRAPH_BOLT_URL` | `MEMGRAPH_BOLT_URL` | `bolt://127.0.0.1:52423` |
| ExoMCP Dashboard | `EXO_DASHBOARD_PORT` | `EXO_DASHBOARD_PORT` | `52424` |
| ExoMCP MCP | `EXO_MCP_PORT` | `EXO_MCP_PORT` | `52425` |
| Hermes Bridge | `HERMES_BRIDGE_URL` | `HERMES_BRIDGE_URL` | `http://127.0.0.1:52426` |
| nexo-rs Metrics | `PROMETHEUS_PORT` | `PROMETHEUS_PORT` | `52427` |
| nexo-rs Health | `HEALTH_PORT` | `HEALTH_PORT` | `52428` |
| nexo-rs Admin | `ADMIN_PORT` | `ADMIN_PORT` | `52429` |
| Conscia API | `CONSCIA_API_ADDR` | `CONSCIA_API_ADDR` | `127.0.0.1:52430` |
| Conscia NATS | `NATS_URL` | `NATS_URL` | `nats://127.0.0.1:52431` |
| Conscia NATS Mon | `NATS_MONITORING_URL` | `NATS_MONITORING_URL` | `http://127.0.0.1:52432` |
| Zrok | `ZROK_API_ENDPOINT` | `ZROK_API_ENDPOINT` | `http://127.0.0.1:52433` |
| Nginx (local) | — | `NGINX_LOCAL_PORT` | `52434` |
| Mailcatcher | — | `MAILCATCHER_SMTP_PORT` / `MAILCATCHER_HTTP_PORT` | `52435` / `52436` |

---

## Socket vs Port Policy

**Default: UNIX domain sockets** for local inter-process communication (higher performance, no port conflicts).

| Service | Socket Path | Port Fallback |
|---------|-------------|---------------|
| llama.cpp (plan) | `/var/run/exo/llama-plan.sock` | 52418 |
| llama.cpp (build) | `/var/run/exo/llama-build.sock` | 52418 |
| llama.cpp (task) | `/var/run/exo/llama-task.sock` | 52418 |
| PostgreSQL | `/var/run/exo/postgresql.sock` | 52422 |
| NATS | `/var/run/exo/nats.sock` | 52431 |

When `--network-modality port` is specified, services bind to loopback TCP ports from this table.

---

## Configuration Files to Update

| File | Changes |
|------|---------|
| `exosystems_net/Cargo.toml` | ✅ `site-addr = "127.0.0.1:52415"`, `reload-port = 52416` |
| `exosystems_net/.env` | ✅ `LEPTOS_SITE_ADDR=127.0.0.1:52415`, `MODEL_UPDATES_STORE=http://127.0.0.1:52417` |
| `exosystems_net/deploy/nginx/exosystems.conf` | Update proxy_pass from 3000 → 52415, 8080 → 52417 |
| `exosystems_net/deploy/nginx/models.conf` | Update proxy_pass from 3000 → 52415, 8080 → 52417 |
| `exosystems_net/src/store.rs` | Default `MODELS_STORE` to `http://127.0.0.1:52417` |
| `conscia/conscia/src/conscierge/router.rs` | `LLAMA_SERVER` to `http://127.0.0.1:52418` |
| `conscia/conscia/src/telemetry.rs` | `local_binding` to `127.0.0.1:52430` |
| `conscia/conscia/src/main.rs` | Telemetry sidecar to 52421 |
| `conscia/conscia/src/workflow_ledger.rs` | PostgreSQL URL to 52422 |
| `exo-mcp/src/main.rs` | Memento 52419, Cognee 52420, Dashboard 52424, MCP 52425 |
| `conscia/docs/decisions/exometro.md` | Update port references |

---

## Verification Checklist

- [ ] `cargo leptos watch --release` in exosystems_net serves on 52415
- [ ] HMR WebSocket connects on 52416
- [ ] Boltstore responds on 52417
- [ ] llama.cpp serves on 52418 (or UNIX sockets)
- [ ] ExoMCP dashboard on 52424, MCP on 52425
- [ ] Memento on 52419, Cognee on 52420
- [ ] Conscia daemon API on 52430
- [ ] NATS on 52431/52432
- [ ] Nginx local on 52434 terminates TLS for exosystems.net testing
- [ ] No port conflicts in 52415–52440 range