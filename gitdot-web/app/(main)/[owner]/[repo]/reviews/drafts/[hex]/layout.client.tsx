"use client";

import type { ReviewResource } from "gitdot-api";
import { Suspense } from "react";
import { Sidebar } from "@/ui/sidebar";
import { ReviewSummary } from "../../[number]/ui/review-summary";

export function LayoutClient({
  owner,
  repo,
  reviewPromise,
  children,
}: {
  owner: string;
  repo: string;
  reviewPromise: Promise<ReviewResource | null>;
  children: React.ReactNode;
}) {
  return (
    <div className="flex flex-1 min-w-0 h-full">
      <Sidebar containerClassName="w-[30%] grow-0" style={{ width: "100%" }}>
        <Suspense>
          <ReviewSummary
            owner={owner}
            repo={repo}
            promises={{ review: reviewPromise }}
          />
        </Suspense>
      </Sidebar>
      <div className="flex flex-1 scrollbar-thin overflow-y-auto items-start">
        {children}
      </div>
    </div>
  );
}
