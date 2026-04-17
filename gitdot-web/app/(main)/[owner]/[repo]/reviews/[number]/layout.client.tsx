"use client";

import { Suspense, useMemo, useState } from "react";
import { PanelLeftOpen, PanelRightOpen } from "lucide-react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { Sidebar } from "@/ui/sidebar";
import { cn } from "@/util";
import type { Resources } from "./layout";
import { ReviewSummary } from "./ui/review-summary";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export type ViewMode = "default" | "summary" | "diff";

export function LayoutClient({
  owner,
  repo,
  requests,
  promises,
  children,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
  children: React.ReactNode;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  const [view, setView] = useState<ViewMode>("default");

  useShortcuts(
    useMemo(
      () => [
        {
          name: "Toggle diffs",
          description: "diffs",
          keys: ["["],
          execute: () => setView((v) => (v === "diff" ? "default" : "diff")),
        },
        {
          name: "Toggle summary",
          description: "summary",
          keys: ["]"],
          execute: () => setView((v) => (v === "summary" ? "default" : "summary")),
        },
      ],
      [],
    ),
  );

  return (
    <div className="flex flex-1 min-w-0 h-full">
      <Sidebar
        containerClassName={cn(
          "shrink-0",
          view === "summary" ? "flex-1" : view === "diff" ? "hidden" : "w-[30%] grow-0",
        )}
        style={{ width: "100%" }}
      >
        <Suspense>
          <ReviewSummary
            owner={owner}
            repo={repo}
            promises={resolvedPromises}
            view={view}
          />
        </Suspense>
      </Sidebar>
      <div
        className={cn(
          "flex flex-1 scrollbar-thin overflow-y-auto items-start",
          view === "summary" && "hidden",
        )}
      >
        {children}
      </div>
      <div className="fixed bottom-6 left-0 flex flex-row w-16 h-8 border-t border-l border-r border-border bg-background">
        <button
          type="button"
          onClick={() => setView(view === "diff" ? "default" : "diff")}
          className={cn(
            "flex flex-1 items-center justify-center border-r border-border transition-colors",
            view === "diff"
              ? "bg-primary text-primary-foreground"
              : "text-muted-foreground hover:text-foreground",
          )}
        >
          <PanelRightOpen className="size-3.5" />
        </button>
        <button
          type="button"
          onClick={() => setView(view === "summary" ? "default" : "summary")}
          className={cn(
            "flex flex-1 items-center justify-center transition-colors",
            view === "summary"
              ? "bg-primary text-primary-foreground"
              : "text-muted-foreground hover:text-foreground",
          )}
        >
          <PanelLeftOpen className="size-3.5" />
        </button>
      </div>
    </div>
  );
}
