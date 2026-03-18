import type { ReviewResource } from "gitdot-api";
import Link from "@/ui/link";
import { timeAgo } from "@/util";

export function ReviewRow({
  owner,
  repo,
  review,
}: {
  owner: string;
  repo: string;
  review: ReviewResource;
}) {
  return (
    <Link
      href={`/${owner}/${repo}/reviews/${review.number}`}
      data-page-item
      tabIndex={-1}
      className="flex flex-row w-full border-b hover:bg-accent/50 focus:bg-accent/50 select-none cursor-default py-2 h-18 px-4 focus:outline-none"
      prefetch={true}
    >
      <div className="flex flex-col w-full justify-start items-start">
        <div className="text-xs text-muted-foreground flex items-center gap-1">
          <span className="truncate min-w-0">{review.author?.name}</span>
          <span>•</span>
          <span>{timeAgo(new Date(review.created_at))}</span>
        </div>
        <div className="text-sm truncate">
          {review.title || `Review #${review.number}`}
        </div>
        <div className="flex flex-row gap-1 text-xs text-muted-foreground">
          <span>{review.status}</span>
          <span>•</span>
          <span>{review.target_branch}</span>
        </div>
      </div>
    </Link>
  );
}
