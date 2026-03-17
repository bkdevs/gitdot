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

`app/provider/` defines an abstract `RepoProvider` with two concrete implementations:

- **`DbProvider`** — reads from IndexedDB (client-only, synchronous-ish)
- **`ApiProvider`** — fetches from the backend API via DAL functions

Both implement the same interface, so callers are provider-agnostic. A resource definition object drives what gets fetched:

```typescript
// app/(main)/[owner]/[repo]/resources.ts
export const RepoResources = {
  paths:   (p: RepoProvider) => p.getPaths(),
  commits: (p: RepoProvider) => p.getCommits(),
  blobs:   (p: RepoProvider) => p.getBlobs(),
};
```

Calling `provider.fetch(RepoResources)` returns a map of named promises — one per resource key.

### Racing IDB → API

The server runs `ApiProvider` and passes the resulting promises (`serverPromises`) down to the client via props. The client instantiates `DbProvider` and races the two for each resource using `firstNonNull()` from `app/util/promise.ts`:

```typescript
// app/(main)/[owner]/[repo]/context.tsx
const dbPromises    = new DbProvider(owner, repo).fetch(RepoResources);
const pathsPromise  = firstNonNull(dbPromises.paths,   serverPromises.paths);
const commitsPromise = firstNonNull(dbPromises.commits, serverPromises.commits);
```

`firstNonNull(...promises)` resolves with the first promise that returns a non-null value, ignoring nulls and errors. IDB almost always wins on repeat visits; the API wins on first load or cache miss.

After the server promise settles, a `useEffect` writes the result back to IDB so subsequent visits hit the cache:

```typescript
useEffect(() => {
  serverPromises.paths.then((p) => {
    if (!p) return;
    idb.putPaths(owner, repo, p);
    setRepoCookie(owner, repo, p.commit_sha); // also update cookie (see below)
  });
}, [owner, repo, idb, serverPromises]);
```

### Cookie-Based Incremental GETs

`app/cookie.ts` stores the last-seen commit SHA in a browser cookie (`gd_sha_{owner}_{repo}`). The DAL reads this cookie server-side and forwards it as an `X-Gitdot-Client-Sha` header:

```typescript
// app/dal/repository.ts
const cookie = await getRepoCookie(owner, repo);
const response = await authFetch(url, { headers: repoCookieHeaders(cookie) });
// repoCookieHeaders returns { "X-Gitdot-Client-Sha": sha, "X-Gitdot-Client-Timestamp": at }
```

The backend can use this header to return only what changed since that SHA — making repeat GETs incremental. The cookie is updated client-side after every successful fetch (see `useEffect` above), so each navigation keeps it current.

### Consuming Resources in Components

Client components inside the repo route should **not** fetch data themselves. Instead, read from `useRepoContext()`, which exposes the already-racing promises:

```typescript
import { useRepoContext } from "@/(main)/[owner]/[repo]/context";

export function MyComponent() {
  const { paths, commits, blobs, hasts } = useRepoContext();
  // each is a Promise<Resource | null> — pass to `use()` or Suspense as needed
}
```

`RepoContext` is `RepoPromises & { hasts: Promise<Map<string, Root>> }`, so all four resources are available. The race (IDB vs API) has already been set up by `RepoClient`; consumers just await the winning promise.

### Complete Flow

```
1. Server layout:   ApiProvider.fetch(RepoResources)             → serverPromises (in-flight)
2. Client context:  DbProvider.fetch(RepoResources)              → dbPromises (IDB read)
3. Race:            firstNonNull(dbPromises.x, serverPromises.x) → display first non-null
4. Sync:            useEffect → serverPromises.x → idb.put*() + setRepoCookie()
5. Components:      useRepoContext() → consume the winning promise
6. Next visit:      IDB wins race; cookie enables incremental server fetch
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
