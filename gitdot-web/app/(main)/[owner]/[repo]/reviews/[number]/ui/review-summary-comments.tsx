"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { Pencil, Trash2 } from "lucide-react";
import { usePathname, useRouter } from "next/navigation";
import { useMemo } from "react";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { cn } from "@/util";
import { UserImage } from "../../../../ui/user-image";
import { useReviewContext } from "../context";

export function ReviewSummaryComments() {
  const { activeDiffComments, activeDiffDraftComments, activeDiff } =
    useReviewContext();
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
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Comments on Diff {activeDiff.position}
      </h2>
      {sorted.length === 0 ? (
        <span className="text-xs text-muted-foreground">no comments yet</span>
      ) : (
        <div className="flex flex-col gap-3">
          {sorted.map((comment) => (
            <ReviewSummaryComment key={comment.id} comment={comment} />
          ))}
        </div>
      )}
    </section>
  );
}

function ReviewSummaryComment({ comment }: { comment: ReviewCommentResource }) {
  const name = comment.author?.name ?? comment.author_id;
  const { activeComment, setActiveComment, diffs, draftComments } =
    useReviewContext();
  const isDraft = draftComments.some((d) => d.id === comment.id);
  const isActive = activeComment?.id === comment.id;
  const router = useRouter();
  const pathname = usePathname();

  function handleClick() {
    if (isActive) {
      setActiveComment(null);
      return;
    }

    setActiveComment(comment);
    const diff = diffs.find((d) => d.id === comment.diff_id);
    if (diff) router.push(`${pathname}?diff=${diff.position}`);
  }

  const fileName = comment.file_path?.split("/").at(-1) ?? null;
  const { line_number_start: start, line_number_end: end } = comment;
  const lineLabel =
    start != null
      ? end != null && end !== start
        ? `(${start}–${end})`
        : `(${start})`
      : null;
  const location = fileName ?? null;

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
              <div className="flex items-center justify-between">
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
                {isDraft && (
                  <span className="text-xs text-muted-foreground">draft</span>
                )}
              </div>
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
