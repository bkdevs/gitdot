"use client";

import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { UserSlug } from "@/(main)/[owner]/ui/user-slug";
import { formatDate } from "@/util";

export function ReviewSummaryHeader({ review }: { review: ReviewResource }) {
  return (
    <div className="shrink-0 h-16 border-b border-border flex items-stretch">
      <div className="flex-1 min-w-0 flex flex-col justify-center pl-6 pr-2 pb-1 gap-1">
        <h1 className="text-sm truncate font-medium">{review.title}</h1>
        <div className="flex items-center gap-1 text-xs text-muted-foreground min-w-0">
          <div className="flex items-center gap-1 min-w-0">
            <UserImage userId={review.author?.id} px={16} />
            {review.author && <UserSlug user={review.author} />}
          </div>
          <span className="shrink-0">
            {formatDate(new Date(review.created_at))}
          </span>
        </div>
      </div>
      <div className="border-l border-border flex flex-col">
        <button
          type="button"
          className="flex-1 w-full px-3 text-xs font-mono bg-primary text-primary-foreground hover:bg-primary/90 underline decoration-transparent hover:decoration-current border-b border-border transition-all duration-200"
        >
          Approve
        </button>
        <button
          type="button"
          className="flex-1 w-full px-3 text-xs font-mono text-muted-foreground hover:text-foreground hover:bg-muted transition-all duration-200"
        >
          Reject
        </button>
      </div>
    </div>
  );
}
