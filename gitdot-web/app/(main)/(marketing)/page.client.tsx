"use client";

import type { RepositoryResource } from "gitdot-api";
import { useState } from "react";
import Link from "@/ui/link";
import { cn } from "@/util";

type FeedTab = "trending" | "new";

export function PageClient({
  trending,
  latest,
}: {
  trending: RepositoryResource[];
  latest: RepositoryResource[];
}) {
  const [tab, setTab] = useState<FeedTab>("trending");
  const feeds: Record<FeedTab, RepositoryResource[]> = {
    trending,
    new: latest,
  };

  return (
    <div className="flex flex-col gap-1 px-3 pt-4 md:pt-5 pb-2">
      <div className="flex items-baseline gap-4">
        {(Object.keys(feeds) as FeedTab[]).map((key) => (
          <button
            key={key}
            type="button"
            onClick={() => setTab(key)}
            className={cn(
              "text-sm font-mono cursor-pointer transition-colors",
              key === tab
                ? "font-semibold text-foreground"
                : "text-muted-foreground hover:text-foreground",
            )}
          >
            {key}
          </button>
        ))}
      </div>

      <div className="flex flex-col">
        {feeds[tab].map((repo) => (
          <Link
            key={repo.id}
            href={`/${repo.owner}/${repo.name}`}
            data-page-item
            className="group flex flex-col py-1 cursor-pointer outline-none"
          >
            <div className="flex items-baseline justify-between gap-4">
              <span className="text-sm font-medium dark:font-normal underline decoration-transparent group-hover:decoration-current group-focus:decoration-current truncate">
                <span className="font-normal text-muted-foreground">
                  {repo.owner}/
                </span>
                {repo.name}
              </span>
              {repo.stars > 0 && (
                <span className="text-xs text-muted-foreground font-mono">
                  ({repo.stars})
                </span>
              )}
            </div>
            {repo.description && (
              <div className="text-xs text-foreground truncate pb-1">
                {repo.description}
              </div>
            )}
          </Link>
        ))}
      </div>
    </div>
  );
}
