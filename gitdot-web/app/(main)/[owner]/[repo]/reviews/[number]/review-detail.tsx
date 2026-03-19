"use client";

import type {
  DiffResource,
  ReviewCommentResource,
  ReviewerResource,
  ReviewResource,
} from "gitdot-api";
import { Suspense, useRef, useState, useTransition } from "react";
import { useUserContext } from "@/(main)/context/user";
import type { DiffEntry } from "@/actions";
import { renderReviewDiffAction } from "@/actions";
import {
  mergeDiffAction,
  publishReviewAction,
  resolveReviewCommentAction,
  submitReviewAction,
  updateDiffAction,
  updateReviewAction,
} from "@/actions/review";
import { Button } from "@/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { Loading } from "@/ui/loading";
import { cn, timeAgo } from "@/util";
import { ReviewBody } from "./review-body";
import {
  type DraftComment,
  ReviewCommentContext,
} from "./review-comment-context";
import { Reviewers } from "./reviewers";

function serverCommentToDraft(
  c: ReviewCommentResource,
  revisionMap: Map<string, number>,
): DraftComment | null {
  if (!c.file_path || c.line_number_start === null || !c.side) return null;
  return {
    id: c.id,
    parent_id: c.parent_id,
    diff_id: c.diff_id,
    revision_id: c.revision_id,
    file_path: c.file_path,
    line_number: c.line_number_start,
    side: c.side as "old" | "new",
    author_name: c.author?.name ?? "Unknown",
    body: c.body,
    created_at: c.created_at,
    resolved: c.resolved,
    revision_number: revisionMap.get(c.revision_id) ?? null,
  };
}

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
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isMerging, setIsMerging] = useState(false);
  const [draftComments, setDraftComments] = useState<DraftComment[]>([]);
  const [activeInput, setActiveInput] = useState<{
    filePath: string;
    lineNumber: number;
    side: "old" | "new";
  } | null>(null);
  const [leftRevision, setLeftRevision] = useState<"base" | number>("base");
  const [rightRevision, setRightRevision] = useState<number | null>(null);
  const [dynamicDiffEntries, setDynamicDiffEntries] = useState<
    DiffEntry[] | null
  >(null);
  const [isLoadingDiff, startDiffTransition] = useTransition();
  const [editingTitle, setEditingTitle] = useState(false);
  const [editingDescription, setEditingDescription] = useState(false);
  const [editingDiffDescription, setEditingDiffDescription] = useState(false);
  const editTitleRef = useRef<HTMLInputElement>(null);
  const editDescriptionRef = useRef<HTMLTextAreaElement>(null);
  const editDiffDescriptionRef = useRef<HTMLTextAreaElement>(null);

  const isReviewer = review.reviewers.some((r) => r.reviewer_id === user?.id);
  const isReviewAuthor = user?.id === review.author_id;
  const canComment = isReviewAuthor || isReviewer;

  function addComment(
    filePath: string,
    lineNumber: number,
    side: "old" | "new",
    body: string,
    parentId?: string,
  ) {
    if (!user || !canComment || !selectedDiff) return;
    setDraftComments((prev) => [
      ...prev,
      {
        id: null,
        parent_id: parentId ?? null,
        diff_id: selectedDiff.id,
        revision_id: selectedDiff.revisions[0]?.id ?? null,
        file_path: filePath,
        line_number: lineNumber,
        side,
        author_name: user.name,
        body,
        created_at: new Date().toISOString(),
        resolved: false,
        revision_number: null,
      },
    ]);
  }

  function handleRevisionChange(newLeft: "base" | number, newRight: number) {
    if (!selectedDiff) return;
    setLeftRevision(newLeft);
    setRightRevision(newRight);

    const latestRevNum = selectedDiff.revisions[0]?.number;
    const isDefault = newLeft === "base" && newRight === latestRevNum;
    if (isDefault) {
      setDynamicDiffEntries(null);
      return;
    }

    startDiffTransition(async () => {
      const entries = await renderReviewDiffAction(
        owner,
        repo,
        number,
        selectedDiff.position,
        newRight,
        newLeft === "base" ? undefined : newLeft,
      );
      setDynamicDiffEntries(entries);
    });
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

  async function handleSubmit(
    action: "approve" | "request_changes" | "comment",
  ) {
    if (!selectedDiff) return;
    setIsSubmitting(true);

    const diffComments = draftComments
      .filter((c) => c.diff_id === selectedDiff.id)
      .map((c) => ({
        body: c.body,
        parent_id: c.parent_id,
        file_path: c.file_path,
        line_number_start: c.line_number,
        line_number_end: null,
        side: c.side,
      }));

    const result = await submitReviewAction(
      owner,
      repo,
      number,
      selectedDiff.position,
      { action, comments: diffComments },
    );

    if ("review" in result) {
      setDraftComments((prev) =>
        prev.filter((c) => c.diff_id !== selectedDiff.id),
      );
    }

    setIsSubmitting(false);
  }

  async function handleMergeDiff(position: number) {
    setIsMerging(true);
    await mergeDiffAction(owner, repo, number, position);
    setIsMerging(false);
  }

  async function handleSaveTitle() {
    const value = editTitleRef.current?.value?.trim();
    setEditingTitle(false);
    if (value && value !== review.title) {
      await updateReviewAction(owner, repo, number, { title: value });
    }
  }

  async function handleSaveDescription() {
    const value = editDescriptionRef.current?.value?.trim();
    setEditingDescription(false);
    if (value !== undefined && value !== review.description) {
      await updateReviewAction(owner, repo, number, { description: value });
    }
  }

  async function handleSaveDiffDescription() {
    if (!selectedDiff) return;
    const value = editDiffDescriptionRef.current?.value?.trim();
    setEditingDiffDescription(false);
    if (value !== undefined && value !== selectedDiff.description) {
      await updateDiffAction(owner, repo, number, selectedDiff.position, {
        description: value,
      });
    }
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
            ) : editingTitle ? (
              <input
                ref={editTitleRef}
                type="text"
                defaultValue={review.title || `Review #${review.number}`}
                className="text-lg font-medium bg-transparent border-b border-border outline-none focus:border-ring"
                autoFocus
                onBlur={handleSaveTitle}
                onKeyDown={(e) => {
                  if (e.key === "Enter") handleSaveTitle();
                  if (e.key === "Escape") setEditingTitle(false);
                }}
              />
            ) : (
              <h1
                className={cn(
                  "text-lg font-medium",
                  isReviewAuthor && "cursor-pointer hover:underline",
                )}
                onClick={() => isReviewAuthor && setEditingTitle(true)}
                onKeyDown={(e) => {
                  if (e.key === "Enter" && isReviewAuthor)
                    setEditingTitle(true);
                }}
              >
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
            ) : editingDescription ? (
              <textarea
                ref={editDescriptionRef}
                defaultValue={review.description}
                placeholder="Add a description..."
                className="text-sm mt-2 bg-transparent border border-border rounded-md p-2 outline-none focus:border-ring resize-none min-h-30"
                autoFocus
                onBlur={handleSaveDescription}
                onKeyDown={(e) => {
                  if (e.key === "Escape") setEditingDescription(false);
                }}
              />
            ) : isReviewAuthor ? (
              <p
                className="text-sm mt-2 cursor-pointer hover:underline"
                onClick={() => setEditingDescription(true)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") setEditingDescription(true);
                }}
              >
                {review.description || "Add a description..."}
              </p>
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
                onClick={() => {
                  setSelectedDiffIndex(index);
                  setLeftRevision("base");
                  setRightRevision(null);
                  setDynamicDiffEntries(null);
                  setEditingDiffDescription(false);
                }}
              >
                <div className="flex items-center gap-2">
                  <span className="text-xs text-muted-foreground">
                    #{diff.position}
                  </span>
                  <span className="text-sm truncate flex-1">{diff.title}</span>
                  {diff.status === "merged" ? (
                    <span className="size-2 rounded-full bg-purple-500 shrink-0" />
                  ) : diff.status === "approved" ? (
                    <span className="size-2 rounded-full bg-green-500 shrink-0" />
                  ) : diff.status === "changes_requested" ? (
                    <span className="size-2 rounded-full bg-orange-500 shrink-0" />
                  ) : null}
                </div>
              </button>
            ))}
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
                  ) : editingDiffDescription ? (
                    <textarea
                      ref={editDiffDescriptionRef}
                      key={`edit-${selectedDiff.id}`}
                      defaultValue={selectedDiff.description}
                      placeholder="Add a description..."
                      className="text-sm text-muted-foreground bg-transparent border border-border rounded-md p-2 outline-none focus:border-ring resize-none min-h-30"
                      autoFocus
                      onBlur={handleSaveDiffDescription}
                      onKeyDown={(e) => {
                        if (e.key === "Escape")
                          setEditingDiffDescription(false);
                      }}
                    />
                  ) : isReviewAuthor ? (
                    <p
                      className="text-sm text-muted-foreground cursor-pointer hover:underline"
                      onClick={() => setEditingDiffDescription(true)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter") setEditingDiffDescription(true);
                      }}
                    >
                      {selectedDiff.description || "Add a description..."}
                    </p>
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
                    {isReviewAuthor &&
                      review.status === "in_progress" &&
                      selectedDiff.status !== "merged" && (
                        <Button
                          variant="outline"
                          size="sm"
                          className="ml-auto shrink-0 text-xs h-6 px-2"
                          disabled={
                            !review.diffs
                              .filter(
                                (d) => d.position <= selectedDiff.position,
                              )
                              .every(
                                (d) =>
                                  d.status === "approved" ||
                                  d.status === "merged",
                              ) || isMerging
                          }
                          onClick={() => handleMergeDiff(selectedDiff.position)}
                        >
                          {isMerging ? "Merging..." : "Stack merge"}
                        </Button>
                      )}
                  </div>

                  {selectedDiff.revisions.length > 0 && (
                    <RevisionSelector
                      revisions={selectedDiff.revisions}
                      leftRevision={leftRevision}
                      rightRevision={
                        rightRevision ?? selectedDiff.revisions[0].number
                      }
                      isLoading={isLoadingDiff}
                      onChange={handleRevisionChange}
                    />
                  )}
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
                    comments: (() => {
                      const revisionMap = new Map(
                        selectedDiff.revisions.map((r) => [r.id, r.number]),
                      );
                      return [
                        ...review.comments
                          .filter((c) => c.diff_id === selectedDiff.id)
                          .map((c) => serverCommentToDraft(c, revisionMap))
                          .filter((c): c is DraftComment => c !== null),
                        ...draftComments.filter(
                          (c) => c.diff_id === selectedDiff.id,
                        ),
                      ];
                    })(),
                    addComment,
                    canComment,
                    isReviewAuthor,
                    onResolve: (commentId, resolved) => {
                      resolveReviewCommentAction(
                        owner,
                        repo,
                        number,
                        commentId,
                        resolved,
                      );
                    },
                    activeInput,
                    setActiveInput,
                  }}
                >
                  {dynamicDiffEntries ? (
                    <Suspense fallback={<Loading />}>
                      <ReviewBody diffEntries={dynamicDiffEntries} />
                    </Suspense>
                  ) : (
                    diffContents[selectedDiff.position]
                  )}
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
          isReviewAuthor={isReviewAuthor}
        />

        {(isReviewer || isReviewAuthor) &&
          review.status === "in_progress" &&
          selectedDiff && (
            <div className="flex flex-col gap-2">
              <h2 className="text-sm font-medium">Submit review</h2>
              {isReviewer && (
                <>
                  <Button
                    variant="default"
                    size="sm"
                    className="w-full"
                    disabled={isSubmitting}
                    onClick={() => handleSubmit("approve")}
                  >
                    Approve
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    className="w-full"
                    disabled={isSubmitting}
                    onClick={() => handleSubmit("request_changes")}
                  >
                    Request changes
                  </Button>
                </>
              )}
              <Button
                variant="ghost"
                size="sm"
                className="w-full"
                disabled={isSubmitting}
                onClick={() => handleSubmit("comment")}
              >
                Comment
              </Button>
              {draftComments.filter((c) => c.diff_id === selectedDiff.id)
                .length > 0 && (
                <p className="text-xs text-muted-foreground">
                  {
                    draftComments.filter((c) => c.diff_id === selectedDiff.id)
                      .length
                  }{" "}
                  pending{" "}
                  {draftComments.filter((c) => c.diff_id === selectedDiff.id)
                    .length === 1
                    ? "comment"
                    : "comments"}
                </p>
              )}
            </div>
          )}
      </div>
    </div>
  );
}

function RevisionSelector({
  revisions,
  leftRevision,
  rightRevision,
  isLoading,
  onChange,
}: {
  revisions: { number: number; commit_hash: string }[];
  leftRevision: "base" | number;
  rightRevision: number;
  isLoading: boolean;
  onChange: (left: "base" | number, right: number) => void;
}) {
  // Revisions are sorted desc (latest first), show asc for the selector
  const sorted = [...revisions].sort((a, b) => a.number - b.number);

  return (
    <div
      className={cn(
        "flex items-center gap-2 text-xs",
        isLoading && "opacity-50",
      )}
    >
      <DropdownMenu>
        <DropdownMenuTrigger className="flex items-center gap-1 px-2 py-1 border border-border rounded hover:bg-accent cursor-pointer text-muted-foreground">
          {leftRevision === "base" ? "Base" : `Rev ${leftRevision}`}
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuRadioGroup
            value={String(leftRevision)}
            onValueChange={(v) =>
              onChange(v === "base" ? "base" : Number(v), rightRevision)
            }
          >
            <DropdownMenuRadioItem value="base">Base</DropdownMenuRadioItem>
            {sorted
              .filter((r) => r.number < rightRevision)
              .map((r) => (
                <DropdownMenuRadioItem key={r.number} value={String(r.number)}>
                  Rev {r.number}
                </DropdownMenuRadioItem>
              ))}
          </DropdownMenuRadioGroup>
        </DropdownMenuContent>
      </DropdownMenu>

      <span className="text-muted-foreground">→</span>

      <DropdownMenu>
        <DropdownMenuTrigger className="flex items-center gap-1 px-2 py-1 border border-border rounded hover:bg-accent cursor-pointer text-muted-foreground">
          Rev {rightRevision}
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuRadioGroup
            value={String(rightRevision)}
            onValueChange={(v) => onChange(leftRevision, Number(v))}
          >
            {sorted.map((r) => (
              <DropdownMenuRadioItem key={r.number} value={String(r.number)}>
                Rev {r.number}
              </DropdownMenuRadioItem>
            ))}
          </DropdownMenuRadioGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
