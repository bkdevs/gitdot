"use client";

import type {
  ReviewAuthorResource,
  ReviewDiffResource,
  ReviewerResource,
} from "gitdot-api";
import { useRef, useState } from "react";
import { useRepoContext } from "@/(main)/[owner]/[repo]/resources/context";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { addReviewerAction, removeReviewerAction } from "@/actions/review";
import { DatabaseProvider } from "@/provider/database";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function ReviewSummaryReviewers({
  owner,
  repo,
  number,
  reviewers,
  diffs,
  author,
}: {
  owner: string;
  repo: string;
  number: number;
  reviewers: ReviewerResource[];
  diffs: ReviewDiffResource[];
  author?: ReviewAuthorResource | null;
}) {
  return (
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Reviewers
      </h2>
      <div className="flex flex-col gap-1.5">
        {author && (
          <div className="flex items-center gap-1.5">
            <UserImage userId={author.id} px={18} />
            <span className="text-sm text-muted-foreground">{author.name}</span>
            <span className="text-xs text-muted-foreground/50">(author)</span>
          </div>
        )}
        {reviewers.map((reviewer) => {
          const approved = diffs.filter((diff) => {
            const latest = diff.revisions.reduce(
              (a, b) => (b.number > a.number ? b : a),
              diff.revisions[0],
            );
            return latest?.verdicts.some(
              (v) => v.reviewer_id === reviewer.reviewer_id,
            );
          }).length;
          return (
            <ReviewerRow
              key={reviewer.id}
              owner={owner}
              repo={repo}
              number={number}
              reviewer={reviewer}
              approvedCount={approved}
              totalDiffs={diffs.length}
            />
          );
        })}
      </div>
      <AddReviewer owner={owner} repo={repo} number={number} />
    </section>
  );
}

function ReviewerRow({
  owner,
  repo,
  number,
  reviewer,
  approvedCount,
  totalDiffs,
}: {
  owner: string;
  repo: string;
  number: number;
  reviewer: ReviewerResource;
  approvedCount: number;
  totalDiffs: number;
}) {
  const [removing, setRemoving] = useState(false);
  const [removeError, setRemoveError] = useState<string | null>(null);
  const { provider: memoryProvider } = useRepoContext();
  const name = reviewer.user?.name ?? reviewer.reviewer_id;

  return (
    <>
      <div className="group flex items-center gap-1.5">
        <UserImage userId={reviewer.reviewer_id} px={18} />
        <span className="text-sm text-muted-foreground">{name}</span>
        <button
          type="button"
          onClick={() => setRemoving(true)}
          className="opacity-0 group-hover:opacity-100 transition-opacity text-xs text-red-500 underline cursor-pointer"
          aria-label={`Remove ${name}`}
        >
          remove
        </button>
        <span className="font-mono text-xs text-muted-foreground ml-auto">
          {approvedCount}/{totalDiffs} approved
        </span>
      </div>
      <Dialog
        open={removing}
        onOpenChange={(open) => {
          if (!open) {
            setRemoving(false);
            setRemoveError(null);
          }
        }}
      >
        <DialogContent
          animations
          showOverlay
          className="p-0 overflow-hidden w-96"
        >
          <div className="px-2 py-2 flex flex-col gap-0 pb-1">
            <DialogTitle className="text-sm font-normal text-foreground">
              Remove {name}?
            </DialogTitle>
            <p className="text-xs text-muted-foreground">
              Are you sure you want to remove {name} as a reviewer?
            </p>
          </div>
          {removeError && (
            <p className="px-2 pb-1 text-xs text-red-500">{removeError}</p>
          )}
          <div className="flex items-center justify-end h-8 border-t border-border">
            <button
              type="button"
              onClick={() => {
                setRemoving(false);
                setRemoveError(null);
              }}
              className="flex items-center px-2 h-full text-xs border-l border-border hover:bg-accent/50 transition-colors cursor-pointer"
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={async () => {
                const result = await removeReviewerAction(
                  owner,
                  repo,
                  number,
                  reviewer.user?.name ?? "",
                );
                if ("error" in result) {
                  setRemoveError(result.error);
                } else {
                  memoryProvider.deleteReview(number);
                  await new DatabaseProvider(owner, repo).deleteReview(number);
                  setRemoving(false);
                  setRemoveError(null);
                }
              }}
              className="flex items-center px-3 h-full text-xs text-red-500 bg-background hover:underline hover:bg-red-50 border-l border-border transition-colors cursor-pointer"
            >
              Remove
            </button>
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}

function AddReviewer({
  owner,
  repo,
  number,
}: {
  owner: string;
  repo: string;
  number: number;
}) {
  const [adding, setAdding] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const { provider: memoryProvider } = useRepoContext();

  function handleButtonClick() {
    setAdding(true);
    setError(null);
    setTimeout(() => inputRef.current?.focus(), 0);
  }

  async function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Escape") {
      e.stopPropagation();
      setAdding(false);
      setError(null);
      return;
    }
    if (e.key === "Enter") {
      const userName = inputRef.current?.value.trim();
      if (!userName) return;
      const formData = new FormData();
      formData.set("user_name", userName);
      const result = await addReviewerAction(owner, repo, number, formData);
      if ("error" in result) {
        setError(result.error);
      } else {
        memoryProvider.deleteReview(number);
        await new DatabaseProvider(owner, repo).deleteReview(number);
        setAdding(false);
        setError(null);
      }
    }
  }

  if (adding) {
    return (
      <div className="flex flex-col gap-1">
        <input
          ref={inputRef}
          type="text"
          placeholder="Username..."
          onKeyDown={handleKeyDown}
          onBlur={() => {
            setAdding(false);
            setError(null);
          }}
          className="h-5 w-full text-xs bg-transparent border-b border-muted-foreground/30 focus:border-foreground text-foreground placeholder:text-muted-foreground outline-none transition-colors duration-200"
        />
        {error && <span className="text-xs text-destructive">{error}</span>}
      </div>
    );
  }

  return (
    <button
      type="button"
      onClick={handleButtonClick}
      className="h-5 text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer text-left"
    >
      Add reviewer...
    </button>
  );
}
