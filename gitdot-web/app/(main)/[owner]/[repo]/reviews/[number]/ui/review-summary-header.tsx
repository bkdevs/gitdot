"use client";

import type { ReviewResource, ReviewStatus } from "gitdot-api";
import { useParams, usePathname } from "next/navigation";
import Link from "@/ui/link";

export function ReviewSummaryHeader({ review }: { review: ReviewResource }) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const pathname = usePathname();
  const identifier = pathname.split("/reviews/")[1] ?? "";
  const baseHref = `/${owner}/${repo}/reviews/${identifier}`;

  return (
    <div className="shrink-0 h-16 border-b border-border flex flex-col justify-center px-4 py-1 overflow-hidden">
      <div className="min-h-0 flex items-center overflow-hidden">
        <h1 className="text-sm leading-tight line-clamp-2 font-medium">
          {review.title}
        </h1>
      </div>
      <div className="flex items-center justify-between text-xs font-mono text-muted-foreground min-w-0">
        <div className="flex items-center">
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
          <Link className="hover:underline shrink-0" href={baseHref}>
            {identifier}
          </Link>
        </div>
        <ReviewSummaryStatus status={review.status} />
      </div>
    </div>
  );
}

function ReviewSummaryStatus({ status }: { status: ReviewStatus }) {
  switch (status) {
    case "draft":
      return (
        <span className="text-xs shrink-0 text-muted-foreground">draft</span>
      );
    case "in_progress":
      return <span className="text-xs shrink-0 text-foreground">open</span>;
    case "closed":
      return (
        <span className="text-xs shrink-0 text-muted-foreground underline">
          closed
        </span>
      );
  }
}
