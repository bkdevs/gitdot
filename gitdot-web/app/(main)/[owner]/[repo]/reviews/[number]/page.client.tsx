"use client";

import type { ReviewResource } from "gitdot-api";
import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { MarkdownBody } from "@/(main)/[owner]/[repo]/ui/markdown/markdown-body";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { UserSlug } from "@/(main)/[owner]/ui/user-slug";
import { Loading } from "@/ui/loading";
import { formatDate, timeAgoFull } from "@/util";
import type { Resources } from "./page";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  number,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  number: number;
  requests: ResourceRequests;
  promises: ResourcePromises;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent
        owner={owner}
        repo={repo}
        number={number}
        promises={resolvedPromises}
      />
    </Suspense>
  );
}

function PageContent({
  promises,
}: {
  owner: string;
  repo: string;
  number: number;
  promises: ResourcePromises;
}) {
  const review = use(promises.review);
  if (!review) return null;

  return (
    <div className="max-w-3xl mx-auto px-6 py-8 flex flex-col gap-6">
      <ReviewHeader review={review} />
      {review.description && (
        <div className="border-t pt-6">
          <MarkdownBody content={review.description} />
        </div>
      )}
    </div>
  );
}

function ReviewHeader({ review }: { review: ReviewResource }) {
  const openCount = review.diffs.filter((d) => d.status === "open").length;
  const approvedCount = review.diffs.filter(
    (d) => d.status === "approved",
  ).length;
  const mergedCount = review.diffs.filter((d) => d.status === "merged").length;

  return (
    <div className="flex flex-col gap-3">
      <div className="flex items-start gap-3">
        <h1 className="text-xl font-medium leading-snug flex-1">
          {review.title || `Review #${review.number}`}
        </h1>
        <span
          className={`text-xs shrink-0 mt-1 font-medium ${reviewStatusColor(review.status)}`}
        >
          {review.status}
        </span>
      </div>

      <div className="flex items-center gap-2 text-xs text-muted-foreground">
        <UserImage userId={review.author?.id} px={16} />
        {review.author && <UserSlug user={review.author} className="text-xs" />}
        <span>·</span>
        <span title={formatDate(new Date(review.created_at))}>
          {timeAgoFull(new Date(review.created_at))}
        </span>
        <span>·</span>
        <span className="font-mono">{review.target_branch}</span>
      </div>

      <div className="flex items-center gap-4 text-xs text-muted-foreground">
        <span>{review.diffs.length} diffs</span>
        {openCount > 0 && (
          <span className="text-blue-500">{openCount} open</span>
        )}
        {approvedCount > 0 && (
          <span className="text-green-600">{approvedCount} approved</span>
        )}
        {mergedCount > 0 && (
          <span className="text-purple-500">{mergedCount} merged</span>
        )}
        {review.comments.length > 0 && (
          <span>{review.comments.length} comments</span>
        )}
        {review.reviewers.length > 0 && (
          <div className="flex items-center gap-1 ml-auto">
            {review.reviewers.map((r) => (
              <UserImage key={r.id} userId={r.user?.id} px={18} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

function reviewStatusColor(status: string): string {
  switch (status) {
    case "open":
      return "text-blue-500";
    case "approved":
      return "text-green-600";
    case "changes_requested":
      return "text-amber-500";
    case "merged":
      return "text-purple-500";
    case "closed":
      return "text-muted-foreground";
    default:
      return "text-muted-foreground";
  }
}
