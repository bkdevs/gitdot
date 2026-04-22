"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { Pencil, Trash2 } from "lucide-react";
import { usePathname, useRouter } from "next/navigation";
import { useMemo } from "react";
import { AvatarBeam } from "@/ui/avatar-beam";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { cn, timeAgo } from "@/util";
import { useReviewContext } from "../context";

export function ReviewSummaryComments() {
  const { comments } = useReviewContext();
  const sorted = useMemo(
    () =>
      [...comments].sort(
        (a, b) =>
          new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
      ),
    [comments],
  );

  return (
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Comments
      </h2>
      <div className="flex flex-col gap-6">
        {sorted.map((comment) => (
          <ReviewSummaryComment key={comment.id} comment={comment} />
        ))}
      </div>
    </section>
  );
}

function formatLocation(lineStart: number, lineEnd: number | null): string {
  if (lineEnd != null && lineEnd !== lineStart)
    return `:L${lineStart}-${lineEnd}`;
  return `:L${lineStart}`;
}

function ReviewSummaryComment({ comment }: { comment: ReviewCommentResource }) {
  const name = comment.author?.name ?? comment.author_id;
  const { activeComment, setActiveComment, diffs } = useReviewContext();
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

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div
          className="group flex flex-col cursor-pointer"
          onClick={handleClick}
        >
          {comment.file_path && (
            <span
              className={cn(
                "text-xs font-mono text-muted-foreground truncate underline decoration-transparent group-hover:decoration-current transition-colors duration-200",
                isActive && "decoration-current",
              )}
            >
              {comment.file_path}
              {comment.line_number_start != null &&
                formatLocation(
                  comment.line_number_start,
                  comment.line_number_end,
                )}
            </span>
          )}
          <div className="flex flex-col gap-1 pb-1">
            <span className="text-sm text-foreground">{comment.body}</span>
            <div className="flex items-center gap-1 ml-auto">
              <AvatarBeam name={name} size={14} />
              <span className="text-xs text-muted-foreground">{name}</span>
              <span className="text-xs text-muted-foreground">
                {timeAgo(new Date(comment.created_at))}
              </span>
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
