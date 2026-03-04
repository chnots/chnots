---
name: requirement-to-plan
description: Convert requirements into executable task plans and task lifecycle commits in this repo. Use when user says 需求转计划, 写计划文档, 创建任务计划, task-start, task-done, or asks to confirm plan then create feat/yymm-dd/task-desc branch and commit workflow. Create docs/yymm-dd-task-desc.md, start with task-start commit, finish by moving to docs/done and committing task-done.
---

# Requirement To Plan

Turn a natural-language requirement into a concrete, executable plan file in `docs/`.

## When to use

Use this skill when the user asks to:

- write a plan document
- convert requirements into an implementation plan
- create a task file before coding
- create a `docs/yymm-dd-*.md` task note

## Inputs to collect

Extract these fields from the user request and repository context:

1. Goal: what should be delivered
2. Scope: in-scope and out-of-scope
3. Constraints: tech limits, compatibility, deadlines, platform
4. Acceptance criteria: observable completion conditions
5. A short task slug for filename (kebab-case)

If some fields are missing, infer reasonable defaults from existing `docs/*.md` style and the codebase architecture.

## File placement and naming

1. Create the plan under `docs/` (not `docs/done/`).
2. Filename format must follow repo convention from `docs/develop.md`:

   - `yymm-dd-task-desc.md`

3. Use today's date for `yymm-dd`.
4. Use a concise kebab-case English slug for `task-desc`.
5. If target filename already exists, append a numeric suffix:

   - `yymm-dd-task-desc-2.md`

## Plan template

Write the plan in concise Markdown using this structure:

```markdown
# <title>

## Background

<1-3 bullets about why>

## Goals

- <goal 1>
- <goal 2>

## Scope

- In scope: <items>
- Out of scope: <items>

## Technical Plan

1. <step 1>
2. <step 2>
3. <step 3>

## Risks and Decisions

- Risk: <risk> -> Mitigation: <how>
- Decision: <choice and reason>

## Acceptance Criteria

- [ ] <verifiable criterion 1>
- [ ] <verifiable criterion 2>

## Execution Checklist

- [ ] Confirm the plan is executable with user
- [ ] Create branch `feat/<yymm-dd>/<task-desc>`
- [ ] Create first commit: `task-start: <filepath> <desc>`
- [ ] Implement in small commits
- [ ] Move file to `docs/done/` after completion
- [ ] Create final commit: `task-done: <filepath> <desc>`
```

## Writing rules

- Keep plan actionable and implementation-oriented.
- Prefer short bullets over long paragraphs.
- Include concrete module/file hints when known.
- Avoid vague wording like "optimize later" without a condition.
- Match repository terminology (`Chnot`, `Toent`, `Kspace`, etc.) when relevant.

## Execution workflow

1. Read `docs/develop.md` and any related docs for naming and process alignment.
2. Derive a filename from date + slug.
3. Generate the plan using the template above.
4. Save the new file under `docs/`.
5. Ask user to confirm whether the plan is executable.
6. After user confirmation:

   - create and checkout branch `feat/<yymm-dd>/<task-desc>`
   - create first commit with message `task-start: <filepath> <desc>`

7. When the implementation is confirmed complete, use this skill again to finish the task flow:

   - move plan file from `docs/` to `docs/done/`
   - commit with message `task-done: <filepath> <desc>`

8. In each response, report:

   - created file path
   - chosen slug
   - branch name (if created)
   - top 3 implementation steps

## Done criteria

The skill is complete when:

- a new plan file exists in `docs/` with valid naming convention
- the plan includes scope, technical steps, risks, and acceptance criteria
- start flow is done after confirmation (`feat/<yymm-dd>/<task-desc>` + `task-start` commit)
- completion flow is done after implementation (`docs/done/` move + `task-done` commit)
