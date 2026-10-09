# Integrate the exo-mcp memory gateway into the tooling workflow

- Status: Accepted
- Deciders: project owner
- Technical Story: assistants doing tooling work need memory that survives across sessions, and the local gateway already connects three complementary memory systems behind one set of tools.

## Context and Problem Statement

Tooling work spans many sessions: version upgrades, trigger reviews, delivery evidence, and template tuning. Each session starts with reduced context, so durable memory carries decisions, corrections, and verified facts forward. Three systems on this machine hold complementary kinds of knowledge. Memento holds working knowledge and episode knowledge, with reinforcement, scheduled review, and forgetting. Cognee holds meaning knowledge and procedure knowledge as a durable knowledge graph. Graphify holds the code map built from source files. The exo-mcp gateway connects all three behind one set of nine tools, served on this machine with a browser dashboard, and the assistant software already connects to it on startup. What is missing is a standing prescription: which tool serves which kind of knowledge, where the instructions for their systematic use live, and how the memory lifecycle runs during ordinary work. Without it, memory writes happen ad hoc and retrieval depends on individual habit.

## Decision

The gateway serves all tooling work. The gateway routes working knowledge and episode knowledge to Memento. The gateway routes meaning knowledge and procedure knowledge to Cognee. Graphify is engaged only when the assistant uses type codebase. Type codebase stores nothing as memory. The code map at `code/exotalk/graphify-out/graph.json` holds the source structure. The working pattern is search before acting, record durable findings on completion, review flagged memories on schedule, and remove obsolete memories. The gateway journal file at `code/memento-data/gateway-journal.jsonl` is removed. Database-held workflow history records memory writes and deletions.

Instructions for using these tools live in the project instruction file at `/home/exocrat/code/tooling/AGENTS.md` and in the behavior skills that load every session. Task-specific memory needs are written in the task's own plan. A BMAD memory agent scoped to the tooling project is being built through BMAD's builder skill: it keeps its own MEMORY.md files beside the work and is calibrated to the owner in a setup talk.

## Evaluated placement for a durable execution library

DBOS holds execution state. DBOS saves program progress into a database during execution. DBOS resumes from the last finished step after interruption. DBOS runs scheduled jobs exactly once. DBOS waits for human approval without losing place. DBOS records the full history of every run. DBOS holds scheduled review passes, forgetting passes, and curation passes. DBOS holds the retry queue for Cognee writes that report PENDING. DBOS holds the replacement for the removed journal file. DBOS holds execution state only. Memories remain in Memento, Cognee, and Graphify. The workflow ID follows the identity rule named by the owner. Archon covers delivery jobs. DBOS covers memory-operation jobs.

The gateway performs preflight before every Cognee write. Preflight requires LLM_API_KEY set in the gateway environment. Preflight requires the configured embeddings endpoint reachable within 3 seconds. Preflight returns PENDING with the missing dependency named when requirements are absent. The gateway passes LLM_API_KEY through only when explicitly set. The gateway performs no key substitution. Every PENDING Cognee write is held at `code/memento-data/pending-cognee.jsonl` for retry by identical save. The holding file migrates to DBOS workflows. Cognee writes use the embeddings endpoint named by the owner.

## Consequences

Memory writes stay uniform across sessions and assistants. Database-held workflow history shows what was recorded and removed.
