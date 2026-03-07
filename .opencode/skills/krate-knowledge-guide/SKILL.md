---
name: krate-knowledge-guide
description: Answer questions about Chnots krate capabilities by reading docs/features instead of relying on memory. Use when user asks about krate feature knowledge, domain behavior, module responsibilities, or says krate, 功能, 特性, docs/features.
allowed-tools: Read, Glob, Grep
---

# Krate Knowledge Guide

Provide feature knowledge for Chnots krates by reading repository docs at runtime.

## When to use

Use this skill when the user asks about:

- what a krate does
- differences between krates
- domain feature behavior
- implementation-facing feature constraints

Typical trigger words include: `krate`, `feature`, `domain`, `功能`, `特性`, `模块`.

## Source of truth

- Always read from `docs/features/*.md` first.
- Do not embed or invent static knowledge in this skill.
- If docs and code seem inconsistent, state that clearly and prefer docs unless user asks for code-level verification.

## Instructions

1. Identify requested krate names or feature topics from the user message.
2. Locate matching files under `docs/features/`:
   - direct match: `docs/features/<krate>.md`
   - if unclear: read multiple files and synthesize
3. Extract only relevant sections for the question.
4. Answer in concise bullets, grouped by krate or topic.
5. Include file references used (for example: `docs/features/chnot.md`).
6. If required information is missing in docs, explicitly say it is not documented and suggest checking code.

## Response style

- Keep answers factual and scoped to the question.
- Separate documented facts from assumptions.
- Prefer wording like "According to `docs/features/...`".
- Do not claim certainty without a doc reference.

## Fallback behavior

If no matching file exists in `docs/features/`:

1. List available feature docs.
2. Tell the user the requested krate/topic is not documented there.
3. Offer to inspect code paths for deeper verification.
