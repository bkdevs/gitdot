"use client";

import { useParams } from "next/navigation";
import Link from "@/ui/link";
import { formatDate } from "@/util";

export function ReviewSummaryHeader({ review }: { review: ReviewResource }) {
  const { owner, repo, number } = useParams<{
    owner: string;
    repo: string;
    number: string;
  }>();

  return (
    <div className="shrink-0 h-16 justify-center border-b border-border flex flex-col px-6 py-1 overflow-hidden">
      <div className="min-h-0 flex items-center overflow-hidden">
        <h1 className="text-sm leading-tight line-clamp-2 font-medium">
          {review.title}
        </h1>
      </div>
      <div className="flex items-center justify-between gap-2">
        <div className="flex items-center text-xs font-mono text-muted-foreground min-w-0">
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
        <span className="text-xs text-muted-foreground font-mono shrink-0">
          {formatDate(new Date(review.created_at))}
        </span>
      </div>
    </div>
  );
}
