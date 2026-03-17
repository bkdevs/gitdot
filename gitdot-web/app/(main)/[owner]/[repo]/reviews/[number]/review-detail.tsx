"use client";

import type {
  DiffResource,
  ReviewerResource,
  ReviewResource,
} from "gitdot-api";
import { useRef, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import { publishReviewAction } from "@/actions/review";
import { Button } from "@/ui/button";
import { cn, timeAgo } from "@/util";
import {
  type DraftComment,
  ReviewCommentContext,
} from "./review-comment-context";
import { Reviewers } from "./reviewers";

function getLatestVerdicts(diff: DiffResource) {
  const latestRevision = diff.revisions[0];
  if (!latestRevision) return [];
  return latestRevision.verdicts;
}

function DiffVerdicts({
  diff,
  reviewers,
}: {
  diff: DiffResource;
  reviewers: ReviewerResource[];
}) {
  const verdicts = getLatestVerdicts(diff);
  if (reviewers.length === 0) return null;

  return (
    <div className="flex flex-col gap-1.5">
      {reviewers.map((reviewer) => {
        const verdict = verdicts.find(
          (v) => v.reviewer_id === reviewer.reviewer_id,
        );
        return (
          <div key={reviewer.id} className="flex items-center gap-2 text-xs">
            <span
              className={cn(
                "size-2 rounded-full shrink-0",
                verdict?.verdict === "approved"
                  ? "bg-green-500"
                  : verdict?.verdict === "changes_requested"
                    ? "bg-orange-500"
                    : "bg-muted-foreground/30",
              )}
            />
            <span className="text-muted-foreground truncate">
              {reviewer.user?.name ?? "Unknown"}
            </span>
            {verdict && (
              <span
                className={cn(
                  "ml-auto shrink-0",
                  verdict.verdict === "approved"
                    ? "text-green-500"
                    : "text-orange-500",
                )}
              >
                {verdict.verdict === "approved"
                  ? "Approved"
                  : "Changes requested"}
              </span>
            )}
          </div>
        );
      })}
    </div>
  );
}

export function ReviewDetail({
  owner,
  repo,
  number,
  review,
  diffContents,
}: {
  owner: string;
  repo: string;
  number: number;
  review: ReviewResource;
  diffContents: Record<number, React.ReactNode>;
}) {
  const { user } = useUserContext();
  const [selectedDiffIndex, setSelectedDiffIndex] = useState(0);
  const isDraft = review.status === "draft" && user?.id === review.author_id;
  const selectedDiff: DiffResource | undefined =
    review.diffs[selectedDiffIndex];
  const [isPublishing, setIsPublishing] = useState(false);
  const [draftComments, setDraftComments] = useState<DraftComment[]>([]);
  const [activeInput, setActiveInput] = useState<{
    filePath: string;
    lineNumber: number;
    side: "old" | "new";
  } | null>(null);

  const canComment =
    user?.id === review.author_id ||
    review.reviewers.some((r) => r.reviewer_id === user?.id);

  function addComment(
    filePath: string,
    lineNumber: number,
    side: "old" | "new",
    body: string,
  ) {
    if (!user || !canComment || !selectedDiff) return;
    setDraftComments((prev) => [
      ...prev,
      {
        diff_id: selectedDiff.id,
        revision_id: selectedDiff.revisions[0]?.id ?? null,
        file_path: filePath,
        line_number: lineNumber,
        side,
        author_name: user.name,
        body,
        created_at: new Date().toISOString(),
      },
    ]);
  }

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
            {review.diffs.map((diff, index) => {
              const verdicts = getLatestVerdicts(diff);
              const allApproved =
                review.reviewers.length > 0 &&
                review.reviewers.every((r) =>
                  verdicts.some(
                    (v) =>
                      v.reviewer_id === r.reviewer_id &&
                      v.verdict === "approved",
                  ),
                );
              const hasChangesRequested = verdicts.some(
                (v) => v.verdict === "changes_requested",
              );

              return (
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
                    <span className="text-sm truncate flex-1">
                      {diff.title}
                    </span>
                    {allApproved ? (
                      <span className="size-2 rounded-full bg-green-500 shrink-0" />
                    ) : hasChangesRequested ? (
                      <span className="size-2 rounded-full bg-orange-500 shrink-0" />
                    ) : null}
                  </div>
                </button>
              );
            })}
          </div>

          <div className="flex-1 min-w-0">
            {selectedDiff ? (
              <div className="flex flex-col">
                <div className="flex flex-col gap-2 px-4 py-3 border-border border-b">
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

                {review.reviewers.length > 0 && (
                  <div className="px-4 py-3 border-border border-b">
                    <h3 className="text-xs font-medium text-muted-foreground mb-2">
                      Reviewer status
                    </h3>
                    <DiffVerdicts
                      diff={selectedDiff}
                      reviewers={review.reviewers}
                    />
                  </div>
                )}

                <ReviewCommentContext.Provider
                  value={{
                    comments: draftComments.filter(
                      (c) => c.diff_id === selectedDiff.id,
                    ),
                    addComment,
                    canComment,
                    activeInput,
                    setActiveInput,
                  }}
                >
                  {diffContents[selectedDiff.position]}
                </ReviewCommentContext.Provider>
              </div>
            ) : (
              <p className="text-sm text-muted-foreground px-4 py-3">
                No diffs
              </p>
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
