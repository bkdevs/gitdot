"use client";

import type { DiffResource, ReviewResource } from "gitdot-api";
import { useRef, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import { publishReviewAction } from "@/actions/review";
import { Button } from "@/ui/button";
import { cn, timeAgo } from "@/util";
import { Reviewers } from "./reviewers";

export function ReviewDetail({
  owner,
  repo,
  number,
  review,
}: {
  owner: string;
  repo: string;
  number: number;
  review: ReviewResource;
}) {
  const { user } = useUserContext();
  const [selectedDiffIndex, setSelectedDiffIndex] = useState(0);
  const isDraft = review.status === "draft" && user?.id === review.author_id;
  const selectedDiff: DiffResource | undefined =
    review.diffs[selectedDiffIndex];
  const [isPublishing, setIsPublishing] = useState(false);

  const titleRef = useRef<HTMLInputElement>(null);
  const descriptionRef = useRef<HTMLTextAreaElement>(null);
  const diffDescriptionRefs = useRef<Map<number, HTMLTextAreaElement>>(
    new Map(),
  );

  async function handlePublish() {
    setIsPublishing(true);

    const diffs = review.diffs
      .map((diff) => {
        const textarea = diffDescriptionRefs.current.get(diff.position);
        return {
          position: diff.position,
          description: textarea?.value,
        };
      })
      .filter((d) => d.description !== undefined);

    await publishReviewAction(owner, repo, number, {
      title: titleRef.current?.value || undefined,
      description: descriptionRef.current?.value || undefined,
      diffs: diffs.length > 0 ? diffs : undefined,
    });

    setIsPublishing(false);
  }

  return (
    <div className="w-full flex">
      <div className="flex flex-col flex-1 min-w-0 pb-20">
        <div className="pt-4 px-4">
          <div className="flex flex-col gap-1">
            {isDraft ? (
              <input
                ref={titleRef}
                type="text"
                defaultValue={review.title || `Review #${review.number}`}
                className="text-lg font-medium bg-transparent border-b border-border outline-none focus:border-ring"
              />
            ) : (
              <h1 className="text-lg font-medium">
                {review.title || `Review #${review.number}`}
              </h1>
            )}

            <div className="flex flex-row items-center gap-2 text-sm text-muted-foreground">
              <span>{review.status}</span>
              <span>•</span>
              <span>{review.author?.name}</span>
              <span>•</span>
              <span>{review.target_branch}</span>
              <span>•</span>
              <span>{timeAgo(new Date(review.created_at))}</span>
            </div>

            {isDraft ? (
              <textarea
                ref={descriptionRef}
                defaultValue={review.description}
                placeholder="Add a description..."
                className="text-sm mt-2 bg-transparent border border-border rounded-md p-2 outline-none focus:border-ring resize-none min-h-30"
              />
            ) : (
              review.description && (
                <p className="text-sm mt-2">{review.description}</p>
              )
            )}
          </div>
        </div>

        <div className="w-full border-border border-b mt-4" />

        <div className="flex flex-1 min-h-0">
          <div className="w-64 shrink-0 border-border border-r">
            <div className="px-4 py-2 text-sm text-muted-foreground">
              {review.diffs.length}{" "}
              {review.diffs.length === 1 ? "diff" : "diffs"}
            </div>
            {review.diffs.map((diff, index) => (
              <button
                key={diff.id}
                type="button"
                className={cn(
                  "w-full text-left px-4 py-2 border-border border-b transition-colors",
                  index === selectedDiffIndex
                    ? "bg-accent"
                    : "hover:bg-accent/50",
                )}
                onClick={() => setSelectedDiffIndex(index)}
              >
                <div className="flex items-center gap-2">
                  <span className="text-xs text-muted-foreground">
                    #{diff.position}
                  </span>
                  <span className="text-sm truncate">{diff.title}</span>
                </div>
              </button>
            ))}
          </div>

          <div className="flex-1 min-w-0 px-4 py-3">
            {selectedDiff ? (
              <div className="flex flex-col gap-2">
                {isDraft ? (
                  <textarea
                    ref={(el) => {
                      if (el) {
                        diffDescriptionRefs.current.set(
                          selectedDiff.position,
                          el,
                        );
                      }
                    }}
                    key={selectedDiff.id}
                    defaultValue={selectedDiff.description}
                    placeholder="Add a description..."
                    className="text-sm text-muted-foreground bg-transparent border border-border rounded-md p-2 outline-none focus:border-ring resize-none min-h-30"
                  />
                ) : (
                  selectedDiff.description && (
                    <p className="text-sm text-muted-foreground">
                      {selectedDiff.description}
                    </p>
                  )
                )}

                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                  <span>{selectedDiff.status}</span>
                  {selectedDiff.revisions.length > 0 && (
                    <>
                      <span>•</span>
                      <span>
                        {selectedDiff.revisions.length}{" "}
                        {selectedDiff.revisions.length === 1
                          ? "revision"
                          : "revisions"}
                      </span>
                      <span>•</span>
                      <span>
                        {selectedDiff.revisions[0].commit_hash.slice(0, 7)}
                      </span>
                    </>
                  )}
                </div>
              </div>
            ) : (
              <p className="text-sm text-muted-foreground">No diffs</p>
            )}
          </div>
        </div>
      </div>

      <div className="w-64 shrink-0 pt-4 px-4 flex flex-col gap-4">
        {isDraft && (
          <Button
            variant="default"
            size="sm"
            className="w-full"
            disabled={isPublishing}
            onClick={handlePublish}
          >
            {isPublishing ? "Publishing..." : "Publish"}
          </Button>
        )}
        <Reviewers
          owner={owner}
          repo={repo}
          number={number}
          reviewers={review.reviewers}
        />
      </div>
    </div>
  );
}
