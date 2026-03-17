# agent-client

An AI agent built from scratch in Rust — no LangChain, no Rig, no magic.

Raw HTTP against the Anthropic Messages API. Every layer is hand-rolled to understand exactly what frameworks abstract away.

## What it does

- Sends requests to Claude via raw HTTP (reqwest)
- Maintains conversation history across turns
- Supports tool calls — Claude can request a tool, you execute it, send the result back
- Loops until Claude returns a final answer (ReAct pattern)

## Project structure

```
src/
  main.rs              — conversation loop (REPL)
  tools.rs             — tool implementations
  model/
    adapter.rs         — ModelAdapter trait
    anthropic.rs       — concrete HTTP implementation
    types.rs           — ModelRequest, Message, MessageContent, ModelResponse
```

## Run

```bash
export ANTHROPIC_API_KEY=your_key_here
cargo run
```

Then ask something like: `What time is it on my local machine?`

## What this covers

This is Phase 1 of a series on building secure AI agents in Rust:

- Raw HTTP request/response cycle
- Message history (how multi-turn conversation actually works)
- Tool call format: `tool_use` → execute → `tool_result` → final answer
- What Rig and similar frameworks hide from you

## Known security issues (intentional — fixed in next phase)

- No tool authorization — Claude decides which tools to call
- No input validation on tool args
- No audit log
- Vulnerable to prompt injection
- Unbounded tool execution loop

These are discussed in the accompanying video.
