"use client";

import type { RepositoryResource } from "gitdot-api";
import { ChevronDown } from "lucide-react";
import { useMemo, useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";

type RepoSort = "recent" | "stars";

const REPO_SORT_LABELS: Record<RepoSort, string> = {
  recent: "Recent",
  stars: "Stars",
};

export function OrgRepositories({
  repos,
  isOwner,
}: {
  repos: RepositoryResource[] | null;
  isOwner: boolean;
}) {
  const [sortBy, setSortBy] = useState<RepoSort>("recent");
  const repositories = useMemo(() => repos ?? [], [repos]);
  const publicRepos = repositories.filter((r) => r.visibility === "public");
  const privateRepos = repositories.filter((r) => r.visibility === "private");

  return (
    <div className="px-3">
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
  repos: RepositoryResource[];
  sortBy: RepoSort;
}) {
  const sortedRepos = [...repos].sort((a, b) => {
    if (sortBy === "stars") return b.stars - a.stars;
    return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
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

function RepoRow({ repo }: { repo: RepositoryResource }) {
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
      </div>
      {repo.description && (
        <div className="text-xs text-foreground truncate pb-1">
          {repo.description}
        </div>
      )}
    </div>
  );
}
