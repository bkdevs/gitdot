---
title: "How gitdot's backend evolved"
slug: "how-gitdot-backend-evolved"
author: "mikkel"
date: "Jun 1, 2026"
---

On December 14th, Paul and I were sitting in Ciao Gloria, discussing what to work on for the next year. We were deep in pivot hell, and had been since we came back from YC in September. We'd researched everything from e-commerce to construction to garbage management. Nothing stood out. Then Paul made an executive call to stick with developer tools. 

We each pitched a few ideas. I don't remember them all, but one was building a better GitHub. Paul left the final call to me. We walked over to Prospect Park, and after a few laps I picked a better GitHub. No particular reason, it just sounded like the most fun. GitHub is a big, complex system, a git remote server, issues, pull requests, and more. We started with the git remote server. Paul took the frontend, I took the backend.

This is how that backend grew from one Rust server into the handful of services it runs today. Each section is a step, with a diagram of the whole system at that point, so you can see what changed.

&nbsp;

## Step 1, one server talking to git

Before serving a single git request, I had to pick a language and framework. The safe choice was Python and FastAPI, which I knew well and could move fast in. Go was close behind, since it's built for this kind of server work. But I wanted something new, and something that wasn't Go, since Gitea already is. There was a product reason too. We wanted gitdot to feel fast, and Rust is about as fast as it gets. Honestly the language barely matters at the server layer. Most of the latency is the database and git itself. But it didn't hurt. So I picked Rust and Axum.

Then I learned git properly. I'd used git for ten years without ever understanding its internals. Blobs, trees, commits, hooks, how clone and push actually work. I read the git docs and the git source to figure it out. To drive git from Rust I used **git2**, the libgit2 bindings. I looked at **gitoxide**, the native Rust implementation, but it wasn't ready, so I stuck with the C library.

Then I built the thing itself. When you run `git clone` or `git push` over HTTPS, git speaks the smart HTTP protocol. The client hits `GET /info/refs` to see what the server has, then makes the real request, `git-upload-pack` for a fetch and `git-receive-pack` for a push. Three endpoints, that's the whole surface.

I chose not to implement that protocol myself. Just not yet. Git ships `git http-backend`, a small CGI program that already speaks smart HTTP correctly. So the server just shells out to it instead of hand-writing pkt-line framing and capability negotiation. Clone and push worked on day one. It streams bodies straight through, since a push can be gigabytes and you don't want it sitting in memory. Repos live as bare repos on disk, and Postgres holds the metadata, Supabase at the time. There was no auth yet, so every repo was public. We punted SSH as it's a separate transport with its own server and keys.

Step one was the whole product in miniature. One server, git on disk, one database.

<img src="/blog/how-gitdot-backend-evolved-1.png" alt="step 1" height="400" />

&nbsp;

## Step 2, giving the server a shape

With git working, I turned to structure. I'm obsessive about it. A messy codebase gives me an emotional barrier to even opening it. So I spent a couple of weeks on how to lay out an Axum server, something consistent enough that any new feature had one obvious place to go, no matter who wrote it. That was hard while learning Rust and Axum at once, with almost nothing to copy from. Plenty of Rust CLIs existed, but almost no real servers. So I borrowed from Eric Evans' *Domain-Driven Design* and a [post on hexagonal architecture in Rust](https://www.howtocodeit.com/guides/master-hexagonal-architecture-in-rust#anatomy-of-a-bad-rust-application).

The result was splitting the backend into two crates. **gitdot-server** is the thin HTTP layer, just Axum handlers and routing. **gitdot-core** holds the domain logic. Splitting it out was also a bet on the future. I expected more servers to come, and a standalone core meant any of them could reuse the business logic without pulling in the HTTP layer. Inside core, everything has the same shape. A service runs an operation, a repository hits Postgres through sqlx, a client wraps an external service like GitHub or email, a model carries the data, and a DTO is the stable contract back out. A handler barely does anything. It reads the request, calls a service, and maps the result to the API type.

Errors and responses run through one funnel, so every handler looks the same. Every error collapses into a single type that knows its HTTP status, and every response is a typed resource with a status code. This is where I felt good about picking Rust. No try/catch scattered everywhere, just ? to bubble an error up, and all the handling collected in one place.

The outside still looks the same, one server and one database. But the structure is what made the rest come fast. Users, repositories, organizations, each new domain was just the same layers again. It paid off in a way I didn't expect, too. Once Claude Code started writing parts of the code, the predictable structure gave it a clear pattern to follow, and the output came back higher quality and easier to review. It's the shape everything after is built on.

<img src="/blog/how-gitdot-backend-evolved-2.png" alt="step 2" height="400" />

&nbsp;

## Step 3, a second consumer

For a while, the request and response types, routes, and resource shapes all lived inside gitdot-server. Fine, while the server was the only thing that needed them.

Then I started building gitdot-cli. The CLI is its own story for another post, but it surfaced the problem immediately. A client has to know exactly what the server expects and returns, the request body, the response shape, the path, the method. All of it existed already, trapped inside the server crate, and the CLI had no business depending on the whole server to borrow a few structs.

So I pulled the contract into its own crate, gitdot-api. It holds two things. The resource types, the shapes the API returns, like `RepositoryResource` or `UserResource`. And the endpoint definitions, each tying a path and method to a request and response type. Server and CLI both depend on it, so the contract has one definition and both sides agree by construction.

Since gitdot-api has no server code in it, writing a client is almost mechanical. The CLI is just the first. Anyone could pull in gitdot-api and build their own against the same typed contract. The web frontend does the same from the other side, with the types mirrored in TypeScript.

The CLI unlocked one more thing, private repos. Until then every repo was public, since the git endpoints had no auth. The CLI fixed that with git's own credential system. Login still ran on Supabase then, which I'll come back to. `dot login` hands your gitdot token to `git credential approve`, which stores it in whatever helper git already uses, like the macOS keychain. After that, cloning or pushing a private repo over HTTPS just works. Git pulls the token back out and sends it, and the server checks it. You never type a token, and I never built credential storage. Git already had one, the same way it already had http-backend.

<img src="/blog/how-gitdot-backend-evolved-3.png" alt="step 3" height="400" />

&nbsp;

## Step 4, owning auth and the database

So far there was one backend server. The next thing to split off was auth, and again the reason was speed.

Early on we didn't build auth ourselves. We used Supabase, both its auth product and the Postgres that came with it. It got login working fast and let us focus on git.

Then we profiled the app, and a lot of the time went to database roundtrips. Not the queries, the roundtrips. Every call to a managed database elsewhere on the network adds latency, and it stacks up when one request makes several. I wanted Postgres sitting right next to the servers, in our own VPC, so a roundtrip was a fraction of a millisecond instead of a network hop.

Once I decided to host our own database, Supabase auth got awkward. Its auth expects its own database, so keeping it meant running two Postgres instances and syncing them, one for auth and one for everything else. Exactly the split I didn't want. We'd planned to own auth eventually anyway, for the control over login flows, so we said screw it and built a minimal version now.

gitdot-auth is its own Axum server. It handles the usual flows, email one-time-code login, GitHub OAuth, a device-code flow for the CLI, session refresh, logout. Same shape as the main server underneath, thin handlers delegating to an `AuthenticationService` in gitdot-core. Log in and it hands back a signed token, and every server verifies it with the same public key.

What I like most is how the routes are locked down. A small set is public for the CLI. These are the device-code endpoints, where you trade a long random code for a token. Everything else handles email, OAuth, and sessions, and it sits behind a middleware that only passes requests from our own web app. gitdot-web runs on Vercel, which signs every request with an identity token, and the auth server checks it. So even on the public internet, the sensitive routes are reachable only through gitdot-web. The token design, the device flow, the OIDC gating. There's enough here for its own post.

<img src="/blog/how-gitdot-backend-evolved-4.png" alt="step 4" height="400" />

&nbsp;

## Step 5, a third server and shared plumbing

Around the same time Paul wanted to collect client-side metrics, web vitals and some basic analytics. Useful data, but nothing to do with serving git or API requests, and the traffic is the opposite shape, high volume and fire-and-forget. I didn't want a flood of analytics writes slowing down a clone or a page load. So it became a third Axum server, gitdot-metrics, that only ingests events and stores them.

Standing it up was easy, and that is the bet from Step 2 paying off. Like gitdot-auth before it, gitdot-metrics just leans on gitdot-core for the domain logic it needs. That one crate is now shared by every server, which is exactly why I split it out back then.

Three servers now, with a lot in common. Each needs logging, request IDs, rate limiting, Vercel token verification, and the same extractors for things like the client IP and the user token. I'd written all of it once and wasn't going to copy it twice. So it moved into its own crate, gitdot-axum. It's the plumbing counterpart to gitdot-api. gitdot-api holds the contract every client shares, and gitdot-axum holds the middleware and extractors every server reuses. All three depend on it.

You might ask whether three servers beat one big one. Fair question. One server is simpler to deploy, with no shared crate to maintain. But splitting buys real things. Each deploys and scales on its own, one failing can't take down the others, and the boundaries make each easier to hold in your head. The cost is mostly boilerplate. Each server has its own setup, plus a crate like gitdot-axum to keep it from drifting. But with a well-structured repo and Claude Code, that part is almost nothing. The pattern is already there to copy. With three small servers it doesn't feel heavy yet, and when it does, the boundaries are clean enough that merging or splitting again won't be a rewrite.

<img src="/blog/how-gitdot-backend-evolved-5.png" alt="step 4" height="400" />

&nbsp;

## Step 6, what's next

Honestly, I'm nowhere near happy with where the backend is. My notes are full of TODOs. But that's software. You never get it right from the start, you just keep making it a little better. 

The next step I keep circling back to is pulling git serving into its own server. Right now it lives in the main server and leans on `git http-backend`. The CGI was always a stopgap. It forks a process per request and hides its internals, fine at our size but not at GitHub's scale. GitHub and GitLab both started here and eventually wrote their own, like GitHub's [spokes-receive-pack](https://github.com/github/spokes-receive-pack). Giving git its own home and replacing the CGI with native Rust lets us treat git traffic on its own terms. It's why I learned git's internals early. When we get there, I want to actually understand what I'm reimplementing. I also want to add SSH support, for the SSH fans out there.

Back in December I picked a better GitHub on a walk around Prospect Park, for no reason other than it sounded fun. Six months in, I'm pretty sure it was the right call. This work suits us, and we're going to keep at it. Always be building!
