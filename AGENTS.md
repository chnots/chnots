# AGENTS.md - Chnots Development Guide

Entry point for agentic coding agents working on the Chnots monorepo.


## Documents 
```
docs/
├── ARCHITECTURE.md          # Tech stack, project structure, build commands, Rust code style
├── FRONTEND.md              # TypeScript/React conventions
├── DESIGN.md                # Design docs index + core beliefs
├── PLANS.md                 # Plans index
├── PRODUCT_SENSE.md         # Core product concepts (Chnot, Toent, Kspace, KTab)
├── QUALITY_SCORE.md         # Testing, code quality, dev workflow
├── RELIABILITY.md           # Database design, sync protocol
├── SECURITY.md              # Security practices
├── design-docs/
│   └── index.md             # Verified design docs catalog
├── exec-plans/              # YOU MUST GENERATE the PLAN FILES before write code, DO NOT USE internal plan tools. ALL PLAN MUST write to the files.
│   ├── active/              # Active task plans (yymm-dd-task-desc.md)
│   ├── done/                # Completed plans
│   └── tech-debt-tracker.md
├── features/                # Per-feature specs (chnot, toent, mdwt, kfile, ktab, graph, llmchat)
├── generated/
│   └── db-schema.md         # Auto-generated DB schema reference
├── impl-spec/               # Per-feature implementation specs
├── product-specs/
│   ├── index.md
│   ├── conception.md        # Domain terminology
│   ├── consistent-naming.md # API/DAO naming conventions
│   ├── develop.md           # Development workflow
│   ├── note.md              # Misc dev notes
│   └── release.md           # Release & deploy process
└── references/              # External references
```
- DO NOT use internal plan file, use /requirement-to-plan skill to create plan.
