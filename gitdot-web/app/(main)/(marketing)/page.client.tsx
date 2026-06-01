"use client";

import type { RepositoryResource } from "gitdot-api";
import Image from "next/image";
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
    <>
      <div className="flex flex-col gap-2 px-3 py-2 h-full overflow-y-auto scrollbar-none">
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

        <div className="flex flex-col gap-2">
          {feeds[tab].map((repo) => (
            <div key={repo.id} className="flex flex-col">
              <div className="flex items-baseline justify-between gap-4">
                <Link
                  href={`/${repo.owner}/${repo.name}`}
                  className="text-sm font-medium dark:font-normal underline decoration-transparent hover:decoration-current transition-colors duration-200 truncate"
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
              {repo.description && (
                <div className="text-xs text-foreground truncate pb-1">
                  {repo.description}
                </div>
              )}
            </div>
          ))}
        </div>
      </div>

      <div className="justify-self-end pr-4 pt-1 flex flex-col items-start">
        <Image
          className="dark:invert"
          src="/gitdot-long-black.svg"
          alt="gitdot logo"
          width={120}
          height={57}
        />
        <span className="mt-1 text-xs font-mono text-muted-foreground">
          Build something great.
        </span>
        <button
          type="button"
          onClick={() => window.dispatchEvent(new Event("toggleAuthDialog"))}
          className="text-xs font-mono underline text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
        >
          sign up
        </button>
      </div>
    </>
  );
}
