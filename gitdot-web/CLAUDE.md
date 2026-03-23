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

## Multi-Fetch Pattern (Provider + IDB Race)

For data-heavy pages (e.g. repo browser), we race IndexedDB against the server API and display whichever resolves first, then update IDB with the API result so the next load is instant.

### Providers

`app/provider/` has two entry points:

- **`app/provider/server.ts`** — server-only. Exports `fetchResources(owner, repo, resources)`, which runs `ApiProvider` to kick off API fetches.
- **`app/provider/client.ts`** — client-only. Exports `resolveResources(owner, repo, requests, promises)`, which races the server promises against `DatabaseProvider` (IndexedDB) and writes API results back to IDB.

### page.tsx / page.client.tsx and layout.tsx / layout.client.tsx Pattern

The same split applies to both pages and layouts. Any route segment that needs to fetch data or use server-only APIs uses a server file (`page.tsx` / `layout.tsx`) paired with a client file (`page.client.tsx` / `layout.client.tsx`). Use the layout variant when the data is needed by the layout shell (e.g. nav items, admin flags) rather than by page content.

Each data-heavy route is split into two files:

**`page.tsx`** (server component) — declares the `Resources` type, calls `fetchResources`, and passes `requests` + `promises` to the client component:

```typescript
// page.tsx
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  readme: RepositoryBlobResource | null;
};

export default async function Page({ params }) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    readme: (p) => p.getBlob("README.md"),
  });
  return <PageClient owner={owner} repo={repo} requests={requests} promises={promises} />;
}
```

**`page.client.tsx`** (client component) — calls `resolveResources` to race IDB vs API, then passes the winning promises to a `PageContent` component that consumes them with `use()`:

```typescript
// page.client.tsx
"use client";
import { resolveResources } from "@/provider/client";
import type { Resources } from "./page";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({ owner, repo, requests, promises }) {
  const resolvedPromises = resolveResources(owner, repo, requests, promises);
  return (
    <Suspense>
      <PageContent promises={resolvedPromises} />
    </Suspense>
  );
}

function PageContent({ promises }: { promises: ResourcePromises }) {
  const readme = use(promises.readme);
  // render...
}
```

### Cookie-Based Incremental GETs

`app/cookie.ts` stores the last-seen commit SHA in a browser cookie (`gd_sha_{owner}_{repo}`). The DAL reads this cookie server-side and forwards it as an `X-Gitdot-Client-Sha` header:

```typescript
// app/dal/repository.ts
const cookie = await getRepoCookie(owner, repo);
const response = await authFetch(url, { headers: repoCookieHeaders(cookie) });
// repoCookieHeaders returns { "X-Gitdot-Client-Sha": sha, "X-Gitdot-Client-Timestamp": at }
```

The backend can use this header to return only what changed since that SHA — making repeat GETs incremental.

### Complete Flow

```
1. page.tsx (server):     fetchResources(owner, repo, defs)    → { requests, promises }
2. page.client.tsx:       resolveResources(owner, repo, ...)   → races IDB vs API
3. PageContent:           use(promises.x)                      → renders first non-null result
4. DatabaseProvider:      writes API result back to IDB        → next visit IDB wins
5. Next visit:            IDB wins race; cookie enables incremental server fetch
```

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
