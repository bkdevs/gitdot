"use client";

import { useParams } from "next/navigation";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { UserSlug } from "@/(main)/[owner]/ui/user-slug";
import Link from "@/ui/link";
import { formatDate } from "@/util";

export function ReviewSummaryHeader({ review }: { review: ReviewResource }) {
  const { owner, repo, number } = useParams<{
    owner: string;
    repo: string;
    number: string;
  }>();

  return (
    <div className="shrink-0 h-16 border-b border-border flex items-center justify-between pl-6 pr-3 gap-2">
      <div className="flex flex-col min-w-0">
        <h1 className="text-sm truncate font-medium">{review.title}</h1>
        <div className="flex items-center text-xs font-mono text-muted-foreground">
          <Link className="hover:underline shrink-0" href={`/${owner}`}>
            {owner}
          </Link>
          <span>/</span>
          <Link className="hover:underline shrink-0" href={`/${owner}/${repo}`}>
            {repo}
          </Link>
          <span>/</span>
          <Link
            className="hover:underline shrink-0"
            href={`/${owner}/${repo}/reviews`}
          >
            reviews
          </Link>
          <span>/</span>
          <Link
            className="hover:underline shrink-0"
            href={`/${owner}/${repo}/reviews/${number}`}
          >
            {number}
          </Link>
        </div>
      </div>
      <div className="flex flex-col items-end gap-0.5 shrink-0">
        <span className="text-xs text-muted-foreground">
          {formatDate(new Date(review.created_at))}
        </span>
        <div className="flex items-center gap-1 text-xs text-muted-foreground">
          <UserImage userId={review.author?.id} px={16} />
          {review.author && <UserSlug user={review.author} />}
        </div>
      </div>
    </div>
  );
}
