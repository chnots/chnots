# Frontend Development Guide

## Imports

```typescript
// External libraries (alphabetical)
import { clsx } from "clsx";
import { cva } from "class-variance-authority";

// Internal with @ alias
import type { PageRsp } from "@/common/types";
import request from "@/lib/request";

// Local imports
import type { ChnotMeta } from "./dto";
```

## Components

- Use arrow functions (not function declarations)
- Spread props at end: `{...props}`
- Use `cn()` for class merging
- Radix UI + class-variance-authority patterns
- PascalCase for component names

## Files

- kebab-case for files (e.g., `button.tsx`, `user-input.tsx`)
- Domain structure: `src/krate/<domain>/dto.ts|po.ts|service.ts|store.ts|component/`

## State Management

- Zustand for global/domain state
- React Hook Form + Zod for forms
- Service functions in `service.ts`

## API Calls

- Use request utility from `@/lib/request`
- Automatically adapts to Tauri/browser environment
- `request.postJson(endpoint, data)` pattern

## Styling

- **Tailwind 4.0** - Utility-first CSS with Emotion
- **Biome** - Linter/formatter (replaces ESLint/Prettier)
