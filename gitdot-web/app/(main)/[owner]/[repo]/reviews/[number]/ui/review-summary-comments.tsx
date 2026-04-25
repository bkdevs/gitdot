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
import { cn, timeAgo } from "@/util";
import { UserImage } from "../../../../ui/user-image";
import { useReviewContext } from "../context";

export function ReviewSummaryComments() {
  const { activeDiffComments, activeDiff } = useReviewContext();
  const groups = useMemo(() => {
    const sorted = [...activeDiffComments].sort((a, b) => {
      const pathA = a.file_path ?? "";
      const pathB = b.file_path ?? "";
      if (pathA !== pathB) return pathA.localeCompare(pathB);
      return (a.line_number_start ?? Infinity) - (b.line_number_start ?? Infinity);
    });
    const map = new Map<string, ReviewCommentResource[]>();
    for (const comment of sorted) {
      const key = comment.file_path ?? "";
      const group = map.get(key);
      if (group) group.push(comment);
      else map.set(key, [comment]);
    }
    return map;
  }, [activeDiffComments]);

  return (
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Comments on Diff {activeDiff.position}
      </h2>
      {groups.size === 0 ? (
        <span className="text-xs text-muted-foreground">no comments yet</span>
      ) : (
        <div className="flex flex-col gap-4">
          {[...groups.entries()].map(([filePath, comments]) => (
            <div key={filePath} className="flex flex-col gap-1 pb-3">
              {filePath && (
                <span className="text-xs font-mono text-muted-foreground truncate">
                  {filePath}
                </span>
              )}
              <div className="flex flex-col gap-3">
                {comments.map((comment) => (
                  <ReviewSummaryComment key={comment.id} comment={comment} />
                ))}
              </div>
            </div>
          ))}
        </div>
      )}
    </section>
  );
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
          className={cn(
            "group flex flex-col cursor-pointer rounded px-1.5 py-1 -mx-1.5 transition-colors duration-200",
            isActive && "bg-diff-orange",
          )}
          onClick={handleClick}
        >
          <div className="flex gap-1.5">
            <div className="pt-0.5">
              <UserImage userId={comment.author_id} px={18} />
            </div>
            <div className="flex flex-col">
              <div className="flex items-center gap-1.5">
                <span className="text-sm text-foreground">{name}</span>
                <span className="text-xs text-muted-foreground">{timeAgo(new Date(comment.created_at))}</span>
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
