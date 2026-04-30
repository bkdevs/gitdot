"use client";

import type { ReviewResource } from "gitdot-api";
import { useParams, usePathname } from "next/navigation";
import Link from "@/ui/link";

export function ReviewSummaryHeader({ review }: { review: ReviewResource }) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const pathname = usePathname();
  const identifier = pathname.split("/reviews/")[1] ?? "";
  const baseHref = `/${owner}/${repo}/reviews/${identifier}`;

  return (
    <div className="shrink-0 h-16 border-b border-border flex flex-col justify-center pl-4 pr-2 py-1 overflow-hidden gap-0.5">
      <div className="flex items-center overflow-hidden">
        <h1 className="text-base leading-tight line-clamp-2 font-medium">
          {review.title || (
            <span className="text-muted-foreground">
              Review #{review.number}
            </span>
          )}
        </h1>
      </div>
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
        <Link className="hover:underline shrink-0" href={baseHref}>
          {identifier}
        </Link>
      </div>
    </div>
  );
}
