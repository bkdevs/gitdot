"use client";

import type { RepositoryCommitResource, RepositoryResource } from "gitdot-api";
import { ChevronDown } from "lucide-react";
import { useMemo, useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { formatDate } from "@/util";

type RepoSort = "recent" | "contributions" | "stars";

const REPO_SORT_LABELS: Record<RepoSort, string> = {
  recent: "Recent",
  contributions: "Contributions",
  stars: "Stars",
};

type Repository = {
  owner: string;
  name: string;
  description?: string;
  stars: number;
  visibility: string;
  count: number;
  lastDate: Date | null;
};

export function UserRepos({
  repos,
  commits,
  isOwner,
}: {
  repos: RepositoryResource[] | null;
  commits: RepositoryCommitResource[];
  isOwner: boolean;
}) {
  const [sortBy, setSortBy] = useState<RepoSort>("recent");
  const repositories = useMemo(
    () => buildRepositories(repos ?? [], commits),
    [repos, commits],
  );
  const publicRepos = repositories.filter((r) => r.visibility === "public");
  const privateRepos = repositories.filter((r) => r.visibility === "private");

  return (
    <div>
      <div className="flex items-baseline justify-between mb-2">
        <span className="text-xs text-muted-foreground font-mono">
          <span className="text-foreground/40 select-none"># </span>
          Repositories
        </span>
        <DropdownMenu>
          <DropdownMenuTrigger className="flex items-center gap-0.5 text-xs text-muted-foreground/60 font-mono cursor-pointer transition-colors hover:text-foreground">
            {REPO_SORT_LABELS[sortBy]}
            <ChevronDown className="size-3" />
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end" className="min-w-20">
            {(Object.keys(REPO_SORT_LABELS) as RepoSort[]).map((key) => (
              <DropdownMenuItem
                key={key}
                className="text-xs font-mono"
                onClick={() => setSortBy(key)}
              >
                {REPO_SORT_LABELS[key]}
              </DropdownMenuItem>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      {repositories.length ? (
        isOwner ? (
          <div className="flex flex-col gap-4">
            {publicRepos.length > 0 && (
              <RepoGroup label="Public" repos={publicRepos} sortBy={sortBy} />
            )}
            {privateRepos.length > 0 && (
              <RepoGroup label="Private" repos={privateRepos} sortBy={sortBy} />
            )}
          </div>
        ) : (
          <RepoGroup repos={publicRepos} sortBy={sortBy} />
        )
      ) : (
        <span className="font-mono text-xs">no repos</span>
      )}
    </div>
  );
}

function RepoGroup({
  label,
  repos,
  sortBy,
}: {
  label?: string;
  repos: Repository[];
  sortBy: RepoSort;
}) {
  const sortedRepos = [...repos].sort((a, b) => {
    if (sortBy === "stars") return b.stars - a.stars;
    if (sortBy === "contributions") return b.count - a.count;
    const ta = a.lastDate ? a.lastDate.getTime() : Number.NEGATIVE_INFINITY;
    const tb = b.lastDate ? b.lastDate.getTime() : Number.NEGATIVE_INFINITY;
    return tb - ta;
  });
  return (
    <div>
      {label && (
        <p className="text-xs text-muted-foreground font-mono mb-0.5">
          <span className="text-foreground/40 select-none">## </span>
          {label}
        </p>
      )}
      <div className="flex flex-col gap-1">
        {sortedRepos.map((repo) => (
          <RepoRow key={`${repo.owner}/${repo.name}`} repo={repo} />
        ))}
      </div>
    </div>
  );
}

function RepoRow({ repo }: { repo: Repository }) {
  return (
    <div className="flex flex-col">
      <div className="flex items-baseline justify-between gap-4">
        <div className="flex items-center gap-1 min-w-0">
          <Link
            href={`/${repo.owner}/${repo.name}`}
            className="text-sm font-medium underline decoration-transparent hover:decoration-current transition-colors duration-200 truncate"
          >
            <span className="font-normal text-muted-foreground">
              {repo.owner}/
            </span>
            {repo.name}
          </Link>
          {repo.stars > 0 && (
            <span className="text-xs text-muted-foreground font-mono">
              ({repo.stars})
            </span>
          )}
        </div>
        <span className="text-xs font-mono whitespace-nowrap text-muted-foreground">
          {repo.lastDate ? (
            <>
              {repo.count} contributions
              <span className="mx-1.5">•</span>
              {formatDate(repo.lastDate)}
            </>
          ) : (
            "no contributions"
          )}
        </span>
      </div>
      {repo.description && (
        <div className="text-xs text-foreground truncate pb-1">
          {repo.description}
        </div>
      )}
    </div>
  );
}

function buildRepositories(
  repos: RepositoryResource[],
  commits: RepositoryCommitResource[],
): Repository[] {
  const stats = new Map<string, { count: number; lastDate: Date }>();
  for (const c of commits) {
    const key = `${c.owner_name}/${c.repo_name}`;
    const date = new Date(c.date);
    const existing = stats.get(key);
    if (existing) {
      existing.count += 1;
      if (date > existing.lastDate) existing.lastDate = date;
    } else {
      stats.set(key, { count: 1, lastDate: date });
    }
  }

  const repositories: Repository[] = [];
  const repositoryKeys = new Set<string>();

  for (const r of repos) {
    const key = `${r.owner}/${r.name}`;
    repositoryKeys.add(key);
    const s = stats.get(key);
    repositories.push({
      owner: r.owner,
      name: r.name,
      description: r.description,
      stars: r.stars,
      visibility: r.visibility,
      count: s?.count ?? 0,
      lastDate: s?.lastDate ?? null,
    });
  }

  for (const [key, s] of stats) {
    if (repositoryKeys.has(key)) continue;
    const [owner, name] = key.split("/");
    repositories.push({
      owner,
      name,
      stars: 0,
      visibility: "public",
      count: s.count,
      lastDate: s.lastDate,
    });
  }

  return repositories;
}
