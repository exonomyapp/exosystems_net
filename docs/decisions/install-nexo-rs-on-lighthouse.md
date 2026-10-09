# Install nexo-rs on Lighthouse

## Status

Proposed

## Context

The Lighthouse instance hosts exosystems.net behind nginx.

The program named nexo-rs at `https://github.com/lordmacu/nexo-rs` provides a Rust multi-agent gateway in a single static binary near 90 megabytes. The binary requires no Node runtime and no Python runtime. The program provides a built-in local broker with no external server. The program provides MCP client and server. The program provides per-agent sandboxes. The program provides durable workflows. The program provides Prometheus metrics on port 9090, health on port 8080, and admin console on 127.0.0.1:9091.

## Decision

The Lighthouse instance runs nexo-rs. Installation uses the release artifact for Linux x86_64 from that repository with sha256 verification. Configuration uses nexo YAML files. The instance keeps nginx, certificates, and DNS. The instance keeps node state in SQLite and content in Willow namespaces.

## Consequences

Agent coverage continues through nexo-rs plugins for messaging and mail. Configuration moves to nexo YAML files. Memory use stays near the binary size plus channel plugins. Transcript history uses JSONL with FTS5.
