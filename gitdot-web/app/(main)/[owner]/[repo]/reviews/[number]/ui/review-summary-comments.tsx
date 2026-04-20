"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { AvatarBeam } from "@/ui/avatar-beam";
import { timeAgo } from "@/util";

const MOCK_COMMENTS: ReviewCommentResource[] = [
  {
    id: "00000000-0000-0000-0000-000000000001",
    review_id: "00000000-0000-0000-0000-000000000010",
    diff_id: "00000000-0000-0000-0000-000000000020",
    revision_id: "00000000-0000-0000-0000-000000000030",
    author_id: "00000000-0000-0000-0000-000000000040",
    parent_id: null,
    body: "Should we extract this into a shared utility?",
    file_path: "src/utils/auth.ts",
    line_number_start: 42,
    line_number_end: 42,
    side: "right",
    resolved: false,
    created_at: "2026-04-20T10:00:00Z",
    updated_at: "2026-04-20T10:00:00Z",
    author: { id: "00000000-0000-0000-0000-000000000040", name: "pybbae" },
  },
  {
    id: "00000000-0000-0000-0000-000000000002",
    review_id: "00000000-0000-0000-0000-000000000010",
    diff_id: "00000000-0000-0000-0000-000000000021",
    revision_id: "00000000-0000-0000-0000-000000000031",
    author_id: "00000000-0000-0000-0000-000000000040",
    parent_id: null,
    body: "This needs an error boundary — the fetch can fail silently here.",
    file_path: "src/components/feed.tsx",
    line_number_start: 88,
    line_number_end: 95,
    side: "right",
    resolved: false,
    created_at: "2026-04-20T10:15:00Z",
    updated_at: "2026-04-20T10:15:00Z",
    author: { id: "00000000-0000-0000-0000-000000000040", name: "pybbae" },
  },
  {
    id: "00000000-0000-0000-0000-000000000003",
    review_id: "00000000-0000-0000-0000-000000000010",
    diff_id: "00000000-0000-0000-0000-000000000022",
    revision_id: "00000000-0000-0000-0000-000000000032",
    author_id: "00000000-0000-0000-0000-000000000040",
    parent_id: null,
    body: "Nit: rename to `isLoading` for consistency with the rest of the codebase. We've been using that convention everywhere else in the hooks layer and mixing `loading` and `isLoading` makes it harder to grep for usages.",
    file_path: "src/hooks/useData.ts",
    line_number_start: 14,
    line_number_end: 14,
    side: "right",
    resolved: true,
    created_at: "2026-04-20T10:30:00Z",
    updated_at: "2026-04-20T10:45:00Z",
    author: { id: "00000000-0000-0000-0000-000000000040", name: "pybbae" },
  },
];

export function ReviewSummaryComments() {
  return (
    <section className="flex flex-col gap-1.5">
      <h2 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
        Comments
      </h2>
      <div className="flex flex-col gap-3 -ml-1.5">
        {MOCK_COMMENTS.map((comment) => (
          <ReviewSummaryComment key={comment.id} comment={comment} />
        ))}
      </div>
    </section>
  );
}

function ReviewSummaryComment({ comment }: { comment: ReviewCommentResource }) {
  const name = comment.author?.name ?? comment.author_id;
  return (
    <div className="border-l border-transparent hover:border-foreground pl-1.5 transition-colors duration-200 cursor-pointer">
      <div className="flex flex-col gap-0.5">
        <div className="flex items-center gap-1">
          <AvatarBeam name={name} size={14} />
          <span className="text-xs text-muted-foreground">{name}</span>
          <span className="ml-auto text-xs text-muted-foreground">
            {timeAgo(new Date(comment.created_at))}
          </span>
        </div>
        <span className="text-sm text-foreground">{comment.body}</span>
      </div>
    </div>
  );
}
