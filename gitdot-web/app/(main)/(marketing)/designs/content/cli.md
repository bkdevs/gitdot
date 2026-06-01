---
title: "Using the CLI"
slug: "cli"
order: 2
author: "mikkelk"
date: "Jun 8, 2026"
---

# Using the CLI

The gitdot CLI lets you manage repositories, runners, and your account without
leaving the terminal.

## Authenticating

Run the device login flow to connect the CLI to your account:

```bash
gitdot auth login
```

This opens a browser window where you confirm the device code. Once approved, the
CLI stores a token locally for subsequent commands.

## Common commands

- `gitdot repo create` — create a new repository
- `gitdot repo list` — list your repositories
- `gitdot runner register` — register a CI runner
