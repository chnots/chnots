# AGENTS.md - Chnots Web Application

This file contains development guidelines and commands for agentic coding agents working on the Chnots web application.

## Development Commands

### Build & Development

- **Start development server**: `pnpm run dev` (runs on host 0.0.0.0)
- **Build for production**: `pnpm run build` (TypeScript compilation + Rsbuild build)
- **Preview production build**: `pnpm run preview`

### Code Quality

- **Lint and format**: `pnpm run lint` (Biome linter with auto-fix)
- **Full check**: `pnpm run check` (Biome check with auto-fix)

### Testing

- **No test framework currently configured** - Tests should be added using appropriate framework (Jest, Vitest, etc.)

## Project Architecture

### Tech Stack

- **Frontend**: React 19 + TypeScript
- **Build Tool**: Rsbuild
- **Styling**: Tailwind CSS 4.0 with Emotion for styled components
- **UI Components**: Radix UI primitives with custom components
- **State Management**: Zustand
- **Forms**: React Hook Form with Zod validation
- **Code Editor**: CodeMirror with various language extensions
- **Date/Time**: date-fns and dayjs
- **Routing**: React Router v7

### Directory Structure

- `src/krate/` - Core business logic organized by domain (chnot, kfile, mdwt, etc.)
- `src/common/` - Shared components and utilities
- `src/lib/` - Utility functions and helpers
- `src/hooks/` - Custom React hooks

Each domain in `krate/` follows the pattern:

- `dto.ts` - Data Transfer Objects (API request/response types)
- `po.ts` - Persistent Objects (database entity types)
- `service.ts` - API service functions
- `store.ts` - Zustand state management
- `component/` - React components for that domain

## Code Style Guidelines

### Imports

```typescript
// 1. External libraries (alphabetical)
import { clsx } from "clsx";
import { cva } from "class-variance-authority";

// 2. Internal imports with @ alias (grouped by directory)
import type { PageRsp } from "@/common/types";
import request from "@/lib/request";
import type { ChnotMeta } from "./dto";
import { cn } from "@/lib/utils";
```

- Use absolute imports with `@/` prefix for src files
- Group imports: external libs first, then internal imports
- Type imports use `import type` keyword
- Place component-specific imports at the bottom

### TypeScript & Types

- **Strict mode enabled** - All types must be properly defined
- Use interfaces for object shapes, types for unions/primitives
- Leverage utility types: `Partial<T>`, `Pick<T, K>`, `Omit<T, K>`
- Use branded types where appropriate (e.g., `Varchar<40>`, `TID`)
- Export types from dto files, import them where needed

### Component Conventions

```typescript
// Functional components with TypeScript
function Button({
  className,
  variant = "default",
  size = "sm",
  asChild = false,
  ...props
}: React.ComponentProps<"button"> & {
  variant?: "default" | "destructive" | "outline";
  size?: "sm" | "md" | "lg";
  asChild?: boolean;
}) {
  const Comp = asChild ? Slot : "button";
  return (
    <Comp
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  );
}
```

- Use function declarations, not arrow functions for components
- Spread props at the end (`...props`)
- Use `cn()` utility for conditional class names
- Leverage `class-variance-authority` for component variants
- Use `asChild` pattern for polymorphic components

### Styling & UI

- **Primary**: Tailwind CSS classes
- **Secondary**: Emotion styled components for complex styles
- Use `cn()` utility from `@/lib/utils` to merge classes
- Follow Tailwind naming conventions
- Use Radix UI for accessible primitives
- Maintain consistent spacing and color tokens

### Error Handling

- Service functions should handle errors appropriately
- Use try/catch for async operations
- Leverage React Error Boundaries for component errors
- Return proper error types from API calls

### Naming Conventions

- **Files**: kebab-case (e.g., `button.tsx`, `user-input.tsx`)
- **Components**: PascalCase (e.g., `Button`, `UserInput`)
- **Functions/Variables**: camelCase (e.g., `chnotMetaCommit`, `saveState`)
- **Constants**: UPPER_SNAKE_CASE (e.g., `BASE_URL`, `API_ENDPOINTS`)
- **Types**: PascalCase for interfaces/types, descriptive names (e.g., `ChnotMetaCommitReq`)

### API & Services

- Use the request utility (`@/lib/request`) for HTTP calls
- Service functions should be async and return typed responses
- DTO files define request/response interfaces
- Use environment-agnostic request handling (Tauri vs browser)

### State Management

- Use Zustand for component and domain state
- Keep state logic in `store.ts` files within domains
- Separate concerns: UI state vs business logic state
- Use hooks for accessing store state

### Form Handling

- Use React Hook Form for form management
- Zod schemas for validation
- Leverage resolver integration with React Hook Form
- Controlled components with proper error handling

## Development Workflow

### Before Submitting Changes

1. Run `pnpm run lint` to fix formatting issues
2. Run `pnpm run check` for full code quality check
3. Run `pnpm run build` to ensure TypeScript compilation
4. Test functionality manually in development mode

### Adding New Features

1. Create new domain in `src/krate/` if needed
2. Follow existing patterns for dto/po/service/store structure
3. Use existing UI components from `src/common/component/ui/`
4. Ensure proper TypeScript typing throughout
5. Add any new dependencies to package.json

### Code Review Checklist

- [ ] TypeScript compilation without errors
- [ ] Biome linting passes
- [ ] Proper imports and exports
- [ ] Consistent naming conventions
- [ ] Error handling implemented
- [ ] Responsive design considerations
- [ ] Accessibility (use Radix UI components)

## Environment Notes

### Browser vs Tauri

- Application supports both browser and Tauri desktop environments
- Use `isTauri` from `@/lib/request` for environment-specific logic
- Request handling automatically adapts to environment

### Path Aliases

- `@/` maps to `src/` directory
- Configure in both `tsconfig.json` and `rsbuild.config.ts`

### Component Library

- Extensive UI component library in `src/common/component/ui/`
- Follow existing patterns when creating new components
- Leverage Radix UI primitives for accessibility
