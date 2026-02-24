"use client";

import Image from "next/image";
import { useState } from "react";
import type { OrganizationResponse, UserResponse } from "@/lib/dto";
import type {
  GitHubInstallationListResponse,
  GitHubRepositoryListResponse,
} from "@/lib/dto/migration";
import { githubAppInstallUrl } from "@/util";

export function CreateMigrationForm({
  user,
  organizations,
  installations,
  reposByInstallation,
  defaultOrigin,
}: {
  user: UserResponse;
  organizations: OrganizationResponse[];
  installations: GitHubInstallationListResponse;
  reposByInstallation: Record<string, GitHubRepositoryListResponse>;
  defaultOrigin?: string;
}) {
  const defaultLogin = defaultOrigin ?? installations[0]?.github_login ?? "";

  const [origin, setOrigin] = useState(defaultLogin);
  const [selectedRepos, setSelectedRepos] = useState<Set<string>>(new Set());
  const [destination, setDestination] = useState(user.name);

  const repositories = reposByInstallation[origin] ?? [];

  function handleOriginChange(login: string) {
    setOrigin(login);
    setSelectedRepos(new Set());
  }

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

  return (
    <>
      <h1 className="text-lg font-medium border-b border-border pb-2 mb-4">
        Start new migration
      </h1>
      <form className="space-y-4">
        <div className="flex flex-col gap-1">
          <label htmlFor="origin" className="text-xs text-muted-foreground">
            Origin
          </label>
          <div className="flex gap-2">
            <select
              id="origin"
              name="origin"
              value={origin}
              onChange={(e) => handleOriginChange(e.target.value)}
              className="flex-1 p-2 text-sm bg-background border border-border rounded outline-none"
            >
              {installations.length === 0 && (
                <option value="" disabled>
                  No installations found
                </option>
              )}
              {installations.map((installation) => (
                <option key={installation.id} value={installation.github_login}>
                  {installation.github_login} ({installation.installation_type})
                </option>
              ))}
            </select>
            <a
              href={githubAppInstallUrl("migration")}
              className="flex items-center gap-1 px-3 py-2 text-sm border border-border rounded hover:bg-accent transition-colors"
            >
              <Image
                src="/github-logo.svg"
                alt="GitHub"
                width={14}
                height={14}
              />
              Install GitHub App
            </a>
          </div>
        </div>
        <div className="flex flex-col gap-1">
          <span className="text-xs text-muted-foreground">Repositories</span>
          {repositories.length === 0 ? (
            <p className="text-sm text-muted-foreground py-2">
              {origin ? "No repositories found." : "Select an origin above."}
            </p>
          ) : (
            <ul className="border border-border rounded divide-y divide-border max-h-64 overflow-y-auto">
              {repositories.map((repo) => (
                <li key={repo.id}>
                  <label className="flex items-center gap-3 px-3 py-2 text-sm cursor-pointer hover:bg-accent/50">
                    <input
                      type="checkbox"
                      name="repositories"
                      value={repo.full_name}
                      checked={selectedRepos.has(repo.full_name)}
                      onChange={() => toggleRepo(repo.full_name)}
                    />
                    <span className="flex-1 truncate">{repo.name}</span>
                    {repo.private && (
                      <span className="text-xs text-muted-foreground">
                        private
                      </span>
                    )}
                  </label>
                </li>
              ))}
            </ul>
          )}
        </div>
        <div className="flex flex-col gap-1">
          <label
            htmlFor="destination"
            className="text-xs text-muted-foreground"
          >
            Destination
          </label>
          <select
            id="destination"
            name="destination"
            value={destination}
            onChange={(e) => setDestination(e.target.value)}
            className="w-full p-2 text-sm bg-background border border-border rounded outline-none"
          >
            <option value={user.name}>{user.name} (user)</option>
            {organizations.map((org) => (
              <option key={org.id} value={org.name}>
                {org.name} (organization)
              </option>
            ))}
          </select>
        </div>
        <button
          type="button"
          disabled={selectedRepos.size === 0}
          className="px-3 py-2 text-sm bg-primary text-primary-foreground rounded disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Start migration
        </button>
      </form>
    </>
  );
}
