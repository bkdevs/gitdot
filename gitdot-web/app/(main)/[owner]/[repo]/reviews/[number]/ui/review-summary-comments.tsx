"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { Pencil, Send, Trash2 } from "lucide-react";
import { usePathname, useRouter } from "next/navigation";
import { useEffect, useMemo, useRef, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { cn, pluralize } from "@/util";
import { UserImage } from "../../../../ui/user-image";
import { useReviewContext } from "../context";

export function ReviewSummaryComments() {
  const {
    activeDiffComments,
    activeDiffDraftComments,
    activeDiff,
    publishReview,
  } = useReviewContext();

  const sorted = useMemo(
    () =>
      [...activeDiffComments].sort((a, b) => {
        const pathA = a.file_path ?? "";
        const pathB = b.file_path ?? "";
        if (pathA !== pathB) return pathA.localeCompare(pathB);
        return (
          (a.line_number_start ?? Infinity) - (b.line_number_start ?? Infinity)
        );
      }),
    [activeDiffComments],
  );

  return (
    <section className="flex flex-col gap-0.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Comments on Diff {activeDiff.position}
      </h2>
      {sorted.length === 0 ? (
        <span className="text-xs text-muted-foreground pt-1">
          no comments yet
        </span>
      ) : (
        <div className="flex flex-col gap-2">
          {sorted.map((comment) => (
            <ReviewSummaryComment key={comment.id} comment={comment} />
          ))}
        </div>
      )}
      {activeDiffDraftComments.length > 0 && (
        <div className="flex justify-start pt-1.5">
          <button
            type="button"
            onClick={() => {}}
            className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground hover:underline decoration-current transition-colors cursor-pointer"
          >
            <Send className="size-3" />
            {`Publish ${pluralize(activeDiffDraftComments.length, "comment")}`}
          </button>
        </div>
      )}
    </section>
  );
}

function CommentHeader({
  name,
  location,
  lineLabel,
  action,
}: {
  name: string;
  location: string | null;
  lineLabel: string | null;
  action?: React.ReactNode;
}) {
  return (
    <div className="flex items-center justify-between w-full">
      <div className="flex items-center gap-0.75 flex-wrap">
        <span className="text-xs text-foreground">{name}</span>
        {location && (
          <>
            <span className="text-xs text-muted-foreground truncate">
              on {location}
            </span>
            {lineLabel && (
              <span className="text-[10px] text-muted-foreground">
                {lineLabel}
              </span>
            )}
          </>
        )}
      </div>
      {action}
    </div>
  );
}

type CommentProps = {
  comment: ReviewCommentResource;
  isActive: boolean;
  name: string;
  location: string | null;
  lineLabel: string | null;
  handleClick: () => void;
};

function ReviewSummaryComment({ comment }: { comment: ReviewCommentResource }) {
  const { draftComments, activeComment, setActiveComment, diffs } =
    useReviewContext();
  const { user } = useUserContext();
  const router = useRouter();
  const pathname = usePathname();

  const isDraft = draftComments.some((d) => d.id === comment.id);
  const isActive = activeComment?.id === comment.id;
  const isAuthor = comment.author_id === user?.id;

  const name = comment.author?.name ?? comment.author_id;
  const fileName = comment.file_path?.split("/").at(-1) ?? null;
  const { line_number_start: start, line_number_end: end } = comment;
  const lineLabel =
    start != null
      ? end != null && end !== start
        ? `(${start}–${end})`
        : `(${start})`
      : null;
  const location = fileName;

  function handleClick() {
    if (isActive) {
      setActiveComment(null);
      return;
    }
    setActiveComment(comment);
    const diff = diffs.find((d) => d.id === comment.diff_id);
    if (diff) router.push(`${pathname}?diff=${diff.position}`);
  }

  const props: CommentProps = {
    comment,
    isActive,
    name,
    location,
    lineLabel,
    handleClick,
  };

  if (isDraft) return <DraftComment {...props} />;
  if (isAuthor) return <AuthorComment {...props} />;
  return <ReviewerComment {...props} />;
}

function DraftComment({
  comment,
  isActive,
  name,
  location,
  lineLabel,
  handleClick,
}: CommentProps) {
  const { deleteDraftComment, updateDraftComment } = useReviewContext();

  function save() {
    updateDraftComment(comment.id, draftCommentBody);
    setIsEditing(false);
  }
  const [isEditing, setIsEditing] = useState(false);
  const [draftCommentBody, setDraftCommentBody] = useState(comment.body);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (!isActive) setIsEditing(false);
  }, [isActive]);

  useEffect(() => {
    if (isEditing) textareaRef.current?.focus();
  }, [isEditing]);

  const action = isActive ? (
    <div className="flex items-center gap-1.5">
      {isEditing ? (
        <>
          <button
            type="button"
            onClick={(e) => {
              e.stopPropagation();
              setDraftCommentBody(comment.body);
              setIsEditing(false);
            }}
            className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
          >
            cancel
          </button>
          <button
            type="button"
            onClick={(e) => {
              e.stopPropagation();
              save();
            }}
            className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
          >
            save
          </button>
        </>
      ) : (
        <>
          <button
            type="button"
            onClick={(e) => {
              e.stopPropagation();
              setDraftCommentBody(comment.body);
              setIsEditing(true);
            }}
            className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
          >
            edit
          </button>
          <button
            type="button"
            onClick={(e) => {
              e.stopPropagation();
              deleteDraftComment(comment.id);
            }}
            className="text-xs text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
          >
            delete
          </button>
        </>
      )}
    </div>
  ) : (
    <span className="text-xs text-muted-foreground italic">draft</span>
  );

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div
          className={cn(
            "group flex flex-col cursor-pointer rounded px-1.5 py-1 -mx-1.5 transition-colors duration-200 select-none",
            isActive && "bg-diff-orange",
          )}
          onClick={handleClick}
        >
          <div className="flex gap-1.5">
            <div className="pt-0.5">
              <UserImage userId={comment.author_id} px={18} />
            </div>
            <div className="flex flex-col flex-1 min-w-0">
              <CommentHeader
                name={name}
                location={location}
                lineLabel={lineLabel}
                action={action}
              />
              <textarea
                ref={textareaRef}
                value={draftCommentBody}
                onChange={(e) => setDraftCommentBody(e.target.value)}
                readOnly={!isEditing}
                onClick={(e) => {
                  if (isEditing) e.stopPropagation();
                }}
                onKeyDown={(e) => {
                  if (e.key === "Enter" && !e.shiftKey) {
                    e.preventDefault();
                    save();
                  }
                }}
                className={cn(
                  "text-sm text-foreground w-full resize-none bg-transparent outline-none field-sizing-content border-b transition-colors duration-150",
                  isEditing
                    ? "select-text border-black dark:border-white"
                    : "pointer-events-none border-transparent",
                )}
              />
            </div>
          </div>
        </div>
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem>
          <Pencil />
          Edit
        </ContextMenuItem>
        <ContextMenuItem variant="destructive">
          <Trash2 />
          Delete
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}

function AuthorComment({
  comment,
  isActive,
  name,
  location,
  lineLabel,
  handleClick,
}: CommentProps) {
  const action = isActive ? (
    <div className="flex items-center gap-1.5">
      <button
        type="button"
        className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
      >
        edit
      </button>
      <button
        type="button"
        className="text-xs text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
      >
        delete
      </button>
      <button
        type="button"
        className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
      >
        reply
      </button>
    </div>
  ) : null;

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div
          className={cn(
            "group flex flex-col cursor-pointer rounded px-1.5 py-1 -mx-1.5 transition-colors duration-200 select-none",
            isActive && "bg-diff-orange",
          )}
          onClick={handleClick}
        >
          <div className="flex gap-1.5">
            <div className="pt-0.5">
              <UserImage userId={comment.author_id} px={18} />
            </div>
            <div className="flex flex-col flex-1 min-w-0">
              <CommentHeader
                name={name}
                location={location}
                lineLabel={lineLabel}
                action={action}
              />
              <span className="text-sm text-foreground">{comment.body}</span>
            </div>
          </div>
        </div>
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem>
          <Pencil />
          Edit
        </ContextMenuItem>
        <ContextMenuItem variant="destructive">
          <Trash2 />
          Delete
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}

function ReviewerComment({
  comment,
  isActive,
  name,
  location,
  lineLabel,
  handleClick,
}: CommentProps) {
  const action = isActive ? (
    <div className="flex items-center gap-1.5">
      <button
        type="button"
        className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
      >
        reply
      </button>
    </div>
  ) : null;

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div
          className={cn(
            "group flex flex-col cursor-pointer rounded px-1.5 py-1 -mx-1.5 transition-colors duration-200 select-none",
            isActive && "bg-diff-orange",
          )}
          onClick={handleClick}
        >
          <div className="flex gap-1.5">
            <div className="pt-0.5">
              <UserImage userId={comment.author_id} px={18} />
            </div>
            <div className="flex flex-col flex-1 min-w-0">
              <CommentHeader
                name={name}
                location={location}
                lineLabel={lineLabel}
                action={action}
              />
              <span className="text-sm text-foreground">{comment.body}</span>
            </div>
          </div>
        </div>
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem>
          <Pencil />
          Edit
        </ContextMenuItem>
        <ContextMenuItem variant="destructive">
          <Trash2 />
          Delete
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}
