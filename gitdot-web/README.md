## gitdot-web

### Overview

`gitdot-web` is the Next.js 16 frontend for Gitdot, a GitHub alternative for open-source maintainers. It uses the App Router with React 19 server components, Supabase for authentication, and a custom backend API for all application data. Pages are organized into route groups for auth, landing, blog, and the authenticated main app under `app/(main)/[owner]/[repo]/...`.

Data fetching is built around a multi-provider pattern that races IndexedDB (for instant cached reads) against live API responses. A SharedWorker (`app/workers/sync.ts`) runs in the background to bulk-sync all repository resources — blobs, commits, paths, reviews, builds, and questions — into IDB, and pre-computes syntax-highlighted HAST trees for every file. This makes repeat navigations feel instant even on large repos.

### Files

```
gitdot-web/
├── app/
│   ├── (auth)/                   # Login, signup, onboarding, OAuth device flow
│   ├── (blog)/                   # Public blog at /week/[number]
│   ├── (landing)/                # Public landing page
│   ├── (main)/                   # Authenticated app: home, /:owner, /:owner/:repo/...
│   ├── actions/                  # Server actions (mutations)
│   │   ├── build.ts
│   │   ├── diff.ts
│   │   ├── index.ts
│   │   ├── otel.ts
│   │   ├── question.ts
│   │   ├── repository.ts
│   │   ├── review.ts
│   │   ├── runner.ts
│   │   └── user.ts
│   ├── context/
│   │   └── metrics.tsx           # Web vitals / metrics context
│   ├── dal/                      # Data access layer (server-only)
│   │   ├── build.ts
│   │   ├── migration.ts
│   │   ├── oauth.ts
│   │   ├── otel.ts
│   │   ├── question.ts
│   │   ├── repository.ts
│   │   ├── review.ts
│   │   ├── runner.ts
│   │   ├── task.ts
│   │   ├── user.ts
│   │   └── util.ts               # authFetch/authPost/handleResponse
│   ├── db/
│   │   ├── idb.ts                # IndexedDB client (openIdb)
│   │   ├── index.ts
│   │   └── types.ts              # Database interface type
│   ├── hooks/
│   │   ├── use-animate-number.tsx
│   │   └── use-is-typing.tsx
│   ├── lib/
│   │   ├── s2/                   # S2 durable streams client
│   │   └── supabase.ts           # Supabase server client + session helpers
│   ├── provider/
│   │   ├── database.ts           # DatabaseProvider (IDB-backed ClientProvider)
│   │   ├── index.ts
│   │   ├── memory.ts             # MemoryProvider (in-memory ClientProvider)
│   │   ├── server.ts             # ApiProvider + fetchResources()
│   │   └── types.ts              # RepoProvider, ServerProvider, ClientProvider
│   ├── themes/
│   │   └── gitdot-light.ts       # Custom Shiki theme
│   ├── ui/                       # Shared Radix UI + Tailwind components
│   │   ├── button.tsx
│   │   ├── collapsible.tsx
│   │   ├── dialog.tsx
│   │   ├── dropdown-menu.tsx
│   │   ├── input.tsx
│   │   ├── link.tsx              # Custom Link (falls back to <a> for dynamic segments)
│   │   ├── loading.tsx
│   │   ├── scroll.tsx
│   │   ├── separator.tsx
│   │   ├── sheet.tsx
│   │   ├── sidebar.tsx
│   │   ├── skeleton.tsx
│   │   └── tooltip.tsx
│   ├── util/
│   │   ├── cn.ts                 # Tailwind class merging
│   │   ├── date.ts               # timeAgo, formatDate, subtractDays, ...
│   │   ├── github.ts
│   │   ├── promise.ts
│   │   ├── query.ts              # toQueryString
│   │   ├── string.ts             # pluralize, ...
│   │   └── validation.ts         # validateEmail, validateUsername, ...
│   ├── workers/
│   │   ├── index.ts              # SharedWorker client entry
│   │   ├── sync.ts               # SharedWorker: bulk IDB sync + HAST pre-render
│   │   └── util.ts               # Shiki highlighter + language inference
│   ├── config.ts
│   ├── globals.css
│   ├── icons.tsx
│   └── layout.tsx
├── packages/
│   └── api/                      # gitdot-api TypeScript/Zod package
├── public/
│   ├── blog/
│   └── schema/
├── __mocks__/
├── biome.json
├── jest.config.ts
├── next.config.ts
├── package.json
├── pnpm-workspace.yaml
├── proxy.ts                      # Next.js middleware (session refresh)
└── tsconfig.json
```

### APIs

- **DAL (`app/dal/`)** — Server-only data access. All functions import `"server-only"` and call `authFetch`/`authPost` with automatic Supabase JWT injection.

  - **`app/dal/util.ts`**
    - `authFetch(url, options?)` — `fetch` with `Authorization: Bearer <token>` header
    - `authPost(url, request, extraHeaders?)` — POST with JSON body
    - `authPatch(url, request)` — PATCH with JSON body
    - `authDelete(url, options?)` — DELETE request
    - `authHead(url, options?)` — HEAD request
    - `handleResponse<T>(response, schema: ZodType<T>): Promise<T | null>` — validates response with Zod; returns `null` on 304/404; throws `ApiError` on other failures
    - `handleEmptyResponse(response)` — validates non-body responses
    - `class ApiError` — wraps HTTP error status + message

  - **`app/dal/repository.ts`**
    ```ts
    createRepository(owner, repo, request): Promise<RepositoryResource | null>
    getRepositoryBlob(owner, repo, query): Promise<RepositoryBlobResource | null>
    getRepositoryBlobs(owner, repo, request): Promise<RepositoryBlobsResource | null>
    getRepositoryPaths(owner, repo, query?): Promise<RepositoryPathsResource | null>
    getRepositoryCommits(owner, repo, query?): Promise<RepositoryCommitsResource | null>
    getRepositoryCommit(owner, repo, sha): Promise<RepositoryCommitResource | null>
    getRepositoryCommitDiff(owner, repo, sha): Promise<RepositoryCommitDiffResource | null>
    getRepositorySettings(owner, repo): Promise<RepositorySettingsResource | null>
    getRepositoryResources(owner, repo, request?): Promise<RepositoryResourcesResource | null>
    ```

  - **`app/dal/review.ts`** — `getReview`, `listReviews`, `createReview`, `updateReview`
  - **`app/dal/build.ts`** — `getBuild`, `getBuilds`
  - **`app/dal/question.ts`** — `listQuestions`, `createQuestion`, `updateQuestion`
  - **`app/dal/user.ts`** — `getUser`, `updateUser`

- **Server Actions (`app/actions/`)** — `"use server"` mutations. Return `{ success: true } | { error: string }` or `{ data: T } | { error: string }`.

  - **`app/actions/repository.ts`**
    - `getRepositoryHast(owner, repo, path): Promise<Root | null>` — server action returning syntax-highlighted HAST tree
  - **`app/actions/review.ts`** — `createReview`, `submitReview`, `addComment`
  - **`app/actions/build.ts`** — `triggerBuild`, `cancelBuild`
  - **`app/actions/user.ts`** — `updateProfile`, `updateSettings`

- **Provider (`app/provider/`)** — Multi-fetch pattern racing IDB against live API.

  - **`app/provider/types.ts`**
    ```ts
    // Abstract base: implemented by ApiProvider (server) and DatabaseProvider/MemoryProvider (client)
    abstract class RepoProvider {
      abstract getBlob(path: string): Promise<RepositoryBlobResource | null>
      abstract getHast(path: string): Promise<Root | null>
      abstract getPaths(): Promise<RepositoryPathsResource | null>
      abstract getCommit(sha: string): Promise<RepositoryCommitResource | null>
      abstract getCommits(): Promise<RepositoryCommitResource[] | null>
      abstract getBlobs(): Promise<RepositoryBlobsResource | null>
      abstract getSettings(): Promise<RepositorySettingsResource | null>
      abstract getQuestions(): Promise<QuestionResource[] | null>
      abstract getReview(number: number): Promise<ReviewResource | null>
      abstract getReviews(): Promise<ReviewResource[] | null>
      abstract getBuilds(): Promise<BuildResource[] | null>
      abstract getBuild(number: number): Promise<BuildResource | null>
    }

    // ServerProvider.fetch() records which method+args were called (for client replay)
    abstract class ServerProvider extends RepoProvider {
      fetch<T extends ResourceDefinition>(def: T): ResourceResult<ShapeFromDefinition<T>>
    }

    // ClientProvider.replay() re-invokes recorded requests against the client provider
    abstract class ClientProvider extends RepoProvider {
      replay(requests: Record<string, ResourceRequestType>): Record<string, Promise<unknown>>
    }
    ```

  - **`app/provider/server.ts`**
    ```ts
    // Entry point for server components
    fetchResources(owner, repo, resources): ResourceResult<T>

    // Example usage in page.tsx:
    const { requests, promises } = fetchResources(owner, repo, {
      readme: (p) => p.getBlob("README.md"),
    });
    ```
    - `class ApiProvider extends ServerProvider` — implements all `RepoProvider` methods via DAL calls

  - **`app/provider/database.ts`** — `class DatabaseProvider extends ClientProvider` — implements all methods via `openIdb()`

- **IndexedDB (`app/db/idb.ts`)**
  - `openIdb(): Database` — returns a `Database` handle (no-ops on server/SSR via Proxy)
  - `Database` interface methods:
    - `getPaths / putPaths` — file tree entries keyed by `owner/repo/path`
    - `getBlob / putBlobs` — file content keyed by `owner/repo/path`
    - `getCommit / putCommit / getCommits / putCommits`
    - `getHast / putHast` — pre-rendered Shiki HAST trees
    - `getSettings / putSettings`
    - `getMetadata / putMetadata` — stores `last_commit` + `last_updated` for incremental sync
    - `getQuestions / putQuestions`
    - `getReview / putReview / getReviews / putBuilds / getBuilds`

- **SharedWorker (`app/workers/sync.ts`)** — Background sync worker. Fetches all repo resources from `/:owner/:repo/resources`, writes them to IDB, then pre-renders Shiki HAST for every file blob.
  ```ts
  // Connect from client:
  const worker = new SharedWorker("/workers/sync.js");
  worker.port.postMessage({ owner, repo });
  worker.port.onmessage = ({ data }) => {
    // data: { resourcesReady: boolean, hastsReady: boolean }
  };
  ```

- **Auth (`app/lib/supabase.ts`)**
  - `createSupabaseClient()` — server-side Supabase client with cookie-based session
  - `updateSession(request)` — called in middleware (`proxy.ts`) to refresh session on every request; returns `{ user, response }`
  - `getSession()` — returns current Supabase session with access token
  - `getClaims()` — returns JWT claims including `UserMetadata` (`username`, `orgs`)

- **Utilities (`app/util/`)**
  - `cn(...classes)` — merges Tailwind classes via `clsx` + `tailwind-merge`
  - `timeAgo(date)`, `timeAgoFull(date)` — relative time strings
  - `formatDate(date)`, `formatDateTime(date)`, `formatTime(date)`
  - `subtractDays(date, n)` — returns new Date n days before
  - `pluralize(count, word)` — grammar helper
  - `toQueryString(obj)` — object → URL query string (skips null/undefined)
  - `validateEmail(s)`, `validatePassword(s)`, `validateRepoSlug(s)`, `validateUsername(s)`
