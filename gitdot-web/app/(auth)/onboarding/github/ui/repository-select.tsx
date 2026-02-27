"use client";

import type {
  GitHubInstallationResource,
  GitHubRepositoryResource,
  UserResource,
} from "gitdot-api";
import { useState, useTransition } from "react";
import { migrateGitHubRepositoriesAction } from "@/actions";

export function RepositorySelect({
  user,
  installation,
  repositories,
}: {
  user: UserResource;
  installation: GitHubInstallationResource;
  repositories: GitHubRepositoryResource[] | null;
}) {
  const [selectedRepos, setSelectedRepos] = useState<Set<string>>(new Set());
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();

  function toggleRepo(name: string) {
    setSelectedRepos((prev) => {
      const next = new Set(prev);
      if (next.has(name)) {
        next.delete(name);
      } else {
        next.add(name);
      }
      return next;
    });
  }

  function handleSubmit() {
    setError(null);
    startTransition(async () => {
      const result = await migrateGitHubRepositoriesAction(
        installation.installation_id,
        installation.github_login,
        installation.installation_type,
        user.name,
        "user",
        [...selectedRepos],
      );
      if ("error" in result) {
        setError(result.error);
      }
    });
  }

  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-center justify-center h-screen">
      <div className="flex flex-col text-sm w-sm">
        <p className="pb-2">Repositories.</p>
        <p className="text-primary/60 pb-4">
          Select the repositories you want to migrate to gitdot.
        </p>

        {repositories && repositories.length > 0 ? (
          <ul className="border border-border rounded divide-y divide-border max-h-64 overflow-y-auto">
            {repositories.map((repo) => (
              <li key={repo.id}>
                <label className="flex items-center gap-3 px-3 py-2 cursor-pointer hover:bg-accent/50">
                  <input
                    type="checkbox"
                    name="repositories"
                    value={repo.name}
                    checked={selectedRepos.has(repo.name)}
                    onChange={() => toggleRepo(repo.name)}
                  />
                  <span className="flex-1 truncate">{repo.full_name}</span>
                  {repo.private && (
                    <span className="text-xs text-primary/40">private</span>
                  )}
                </label>
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-primary/60">No repositories found.</p>
        )}

        {error && <p className="text-sm text-destructive mt-2">{error}</p>}

        <div className="flex justify-end gap-3 mt-4">
          <button
            type="button"
            disabled={selectedRepos.size === 0 || isPending}
            onClick={handleSubmit}
            className="px-3 py-1 text-sm bg-primary text-primary-foreground rounded disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isPending ? "Starting migration..." : "Start migration"}
          </button>
        </div>
      </div>
    </div>
  );
}
