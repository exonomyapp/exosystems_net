# Deploy exo-mcp with DBOS and Postgres to Lighthouse

## Status

Proposed

## Context

The Lighthouse instance hosts exosystems.net behind nginx.

The gateway named exo-mcp serves nine tools. The gateway routes working knowledge and episode knowledge to Memento. The gateway routes meaning knowledge and procedure knowledge to Cognee. Graphify is engaged only when the assistant uses type codebase. The code map at `code/exotalk/graphify-out/graph.json` holds source structure.

Cognee requires an embeddings endpoint and a language-model key. The local gateway uses the embeddings model named v5-small-clustering-F16 at `http://127.0.0.1:8090/v1`.

DBOS holds execution state in Postgres. Postgres 18 is the current stable major.

## Decision

The Lighthouse instance runs Postgres 18. The Lighthouse instance runs the gateway named exo-mcp. The gateway uses DBOS held in Postgres 18 on that instance. DBOS holds scheduled review passes. DBOS holds forgetting passes. DBOS holds curation passes. DBOS holds the retry queue for Cognee writes that report PENDING. DBOS holds workflow history for memory writes and deletions. The workflow ID follows the identity rule named by the owner.

Memento runs on the Lighthouse instance for working knowledge and episode knowledge. Cognee runs on the Lighthouse instance for meaning knowledge and procedure knowledge. Cognee uses the endpoint at `https://integrate.api.nvidia.com/v1` with the model name and dimensions named by the owner. The model named nvidia/embed-qa-4 provides 1024 dimensions with 512 tokens of input. The model named nvidia/llama-3.2-nv-embedqa-1b-v2 provides 2048 dimensions with configuration to 384, 512, 768, 1024, and 2048 with 2048 tokens of input. The model named nvidia/nemotron-3-embed-1b provides 2048 dimensions with support for the first 1024 dimensions with 32768 tokens of input across 34 languages. The model named nvidia/nv-embedqa-mistral-7b-v2 provides 4096 dimensions with 512 tokens of input. Storage per text at 1024 dimensions measures about 4 kilobytes. Storage per text at 2048 dimensions measures about 8 kilobytes. Storage per text at 4096 dimensions measures about 16 kilobytes. The gateway passes the endpoint address, the model name, and the dimensions to Cognee. Graphify supplies the code map from the repository file.

## Consequences

Memory writes on the Lighthouse instance survive restarts. Scheduled passes execute exactly once. PENDING Cognee writes retry through DBOS workflows. Workflow history records memory operations in Postgres. Model selection for Cognee determines embedding dimensions and input length.
