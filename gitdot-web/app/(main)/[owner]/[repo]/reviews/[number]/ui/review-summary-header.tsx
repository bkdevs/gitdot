"use client";

import type { ReviewResource, ReviewStatus } from "gitdot-api";
import { Send } from "lucide-react";
import { useParams, usePathname } from "next/navigation";
import { useState } from "react";
import Link from "@/ui/link";
import { cn } from "@/util";
import { useReviewContext } from "../context";

export function ReviewSummaryHeader({ review }: { review: ReviewResource }) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const pathname = usePathname();
  const identifier = pathname.split("/reviews/")[1] ?? "";
  const baseHref = `/${owner}/${repo}/reviews/${identifier}`;

  return (
    <div className="shrink-0 h-16 border-b border-border flex flex-col justify-center pl-4 pr-2 py-1 overflow-hidden">
      <div className="min-h-0 flex items-center overflow-hidden">
        <h1 className="text-sm leading-tight line-clamp-2 font-medium">
          {review.title || (
            <span className="text-muted-foreground">
              Review #{review.number}
            </span>
          )}
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
        {review.status === "draft" ? (
          <DraftPublishButton />
        ) : (
          <ReviewStatusBadge status={review.status} />
        )}
      </div>
    </div>
  );
}

function DraftPublishButton() {
  const { diffs, publishReview } = useReviewContext();
  const [pending, setPending] = useState(false);

  const pendingCount = diffs.filter((d) => d.status === "draft").length;
  const publishable = pendingCount === 0;

  return (
    <button
      type="button"
      disabled={!publishable || pending}
      onClick={async () => {
        setPending(true);
        await publishReview();
        setPending(false);
      }}
      className="flex h-5 items-center gap-1 font-mono text-xs shrink-0 px-2 text-primary-foreground bg-primary disabled:opacity-60 disabled:cursor-not-allowed cursor-pointer outline-none"
    >
      <Send className="size-3" />
      {pending ? "publishing..." : "Publish"}
    </button>
  );
}

function ReviewStatusBadge({ status }: { status: ReviewStatus }) {
  return (
    <span
      className={cn("font-mono text-xs shrink-0", {
        "text-foreground": status === "open",
        "text-muted-foreground underline": status === "closed",
      })}
    >
      {status}
    </span>
  );
}
