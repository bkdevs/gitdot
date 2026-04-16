"use client";

import type { ReviewResource } from "gitdot-api";
import { useState } from "react";
import { cn } from "@/util";

export function ReviewSplashPage({
  owner,
  repo,
  review,
}: {
  owner: string;
  repo: string;
  review: ReviewResource;
}) {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const publishable = title.trim().length > 0 && description.trim().length > 0;

  return (
    <div className="flex flex-1 flex-col items-center min-h-full animate-in fade-in duration-500">
      <div className="flex flex-col items-start gap-8 w-full max-w-lg mt-4">
        <div className="space-y-0.5 w-full">
          <p className="text-xs text-muted-foreground font-mono mt-2">
            <span className="text-foreground/40 select-none"># </span>
            title
          </p>
          <input
            placeholder="title your work..."
            autoFocus
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="text-sm bg-transparent border-l border-border outline-none w-full ml-0.5 placeholder:text-muted-foreground/40 transition-colors focus:border-foreground pl-2"
          />
        </div>
        <div className="space-y-0.5 w-full">
          <p className="text-xs text-muted-foreground font-mono">
            <span className="text-foreground/40 select-none"># </span>
            rationale
          </p>
          <textarea
            placeholder="why did you make this change?"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            className="text-sm bg-transparent border-l border-border outline-none w-full ml-0.5 min-h-24 placeholder:text-muted-foreground/40 transition-colors focus:border-foreground resize-none field-sizing-content pl-2"
          />
        </div>
        <div className="space-y-0.5 w-full">
          <p className="text-xs text-muted-foreground font-mono">
            <span className="text-foreground/40 select-none"># </span>
            diffs
          </p>
          <div className="space-y-1 pt-0.5">
            {review.diffs.map((diff, i) => {
              const latestRevision = diff.revisions.at(-1);
              const sha = latestRevision?.commit_hash.slice(0, 7);
              return (
                <div key={diff.id} className="flex items-start gap-1">
                  <span className="font-mono text-xs shrink-0 text-muted-foreground">
                    {i + 1}.
                  </span>
                  <span className="text-xs text-muted-foreground">
                    {diff.message.split("\n")[0]}
                    {sha && (
                      <span className="text-muted-foreground/50"> ({sha})</span>
                    )}
                  </span>
                </div>
              );
            })}
          </div>
        </div>
      </div>
      <div className="flex justify-end w-full max-w-lg mt-6">
        <button
          type="button"
          className={cn(
            "font-mono text-sm underline transition-all duration-300",
            publishable
              ? "text-foreground decoration-current cursor-pointer"
              : "text-muted-foreground/60 decoration-transparent cursor-not-allowed",
          )}
        >
          Publish.
        </button>
      </div>
    </div>
  );
}
