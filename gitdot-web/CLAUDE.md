# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

See the root `CLAUDE.md` for overall project context, build commands, and backend architecture.

## Frontend-Specific Commands

```bash
pnpm dev                                        # Dev server (port 3000)
pnpm build                                      # Production build
pnpm test                                       # Run all Jest tests
pnpm test -- --testPathPattern=<file>           # Run a specific test file
pnpm biome check . --write                      # Auto-fix lint & format issues
```

**Environment:** Requires `.env.local` with `SUPABASE_URL`, `SUPABASE_PUBLISHABLE_KEY`, and optionally `GITDOT_SERVER_URL` (defaults to `http://localhost:8080`).

## Route Structure

App Router with route groups:

- `(auth)/` — Public auth pages: login, signup, `/oauth/device` (CLI device flow), onboarding
- `(blog)/` — Public blog: `/week/[number]` entries with markdown content
- `(landing)/` — Public landing page at `/`
- `(main)/` — Authenticated app: home, search, notifications, settings, `/:owner`, `/:owner/:repo/...`

Middleware is in `proxy.ts` (not `middleware.ts`). It calls `updateSession()` to refresh the Supabase session cookie on every request.

## Data Fetching Architecture

```
Server Action (app/actions/) → DAL (app/dal/) → Backend API (via authFetch/authPost)
                                               ↗
Server Component              → DAL directly
```

**DAL (`app/dal/`)** — Server-only modules. Import `"server-only"` at the top. Use `authFetch`/`authPost`/`authPatch` from `app/dal/util.ts`, which attach the Supabase JWT automatically and validate responses against Zod schemas via `handleResponse`.

**Server Actions (`app/actions/`)** — Mutations use `"use server"`. Return shape is always `{ success: true } | { error: string }` or `{ data: T } | { error: string }`. Call `refresh()` (from `next/dist/server/app-render/dynamic-rendering`) after mutations to revalidate the current request.

**API Types** — Zod schemas live in the `gitdot-api-ts` workspace package. Import from `gitdot-api` in the DAL for response validation.

## Auth Flow

1. Supabase manages sessions via httpOnly cookies
2. `proxy.ts` middleware refreshes session on every request
3. `createSupabaseClient()` in `app/lib/supabase.ts` — server-side Supabase client with cookie access
4. `getClaims()` / `getSession()` — get identity and access token for API calls
5. Client-side: `UserProvider` context with `useUser()` hook; `useAuthBlocker()` to gate unauthenticated actions

## Component Patterns

**Reusable UI** lives in `app/ui/`. These wrap Radix UI primitives with Tailwind styling. Use CVA (`class-variance-authority`) for components with variants.

**Use `@/ui/link` instead of `next/link`** — enforced by Biome linter. The custom Link component falls back to `<a>` tags for hrefs containing dynamic segments like `[owner]` to avoid hydration mismatches.

**Styling:** Tailwind CSS 4. Use `cn()` from `app/util.ts` (wraps `clsx` + `tailwind-merge`) for conditional/merged class names.

Route-specific components go in `app/(routegroup)/ui/` subfolders, not in the global `app/ui/`.

## Key Utilities (`app/util.ts`)

- `cn(...classes)` — merge Tailwind classes
- `timeAgo()`, `timeAgoFull()` — relative time strings
- `formatDate()`, `formatDateTime()`, `formatTime()` — date formatting
- `pluralize(count, word)` — grammar helper
- `validateEmail()`, `validatePassword()`, `validateRepoSlug()`, `validateUsername()` — input validation
- `toQueryString(obj)` — convert object to URL query params
