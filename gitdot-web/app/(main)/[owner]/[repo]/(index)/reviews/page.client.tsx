"use client";

import { Suspense, use, useMemo, useState } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";
import { ReviewRow } from "./ui/review-row";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export type ReviewsFilter = "draft" | "open" | "closed" | "all";

function filterReviews(
  reviews: NonNullable<Resources["reviews"]>,
  filter: ReviewsFilter,
): NonNullable<Resources["reviews"]> {
  const filtered =
    filter === "all" ? reviews : reviews.filter((r) => r.status === filter);

  return filtered.sort(
    (a, b) =>
      new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
  );
}

export function PageClient({
  owner,
  repo,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent owner={owner} repo={repo} promises={resolvedPromises} />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  promises,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
}) {
  const reviews = use(promises.reviews);
  const [filter, setFilter] = useState<ReviewsFilter>("all");

  const filteredReviews = useMemo(
    () => filterReviews(reviews ?? [], filter),
    [reviews, filter],
  );

  if (!reviews) return null;

  return (
    <div className="flex flex-col">
      <div className="flex flex-row items-center gap-2 px-4 h-10 border-b text-sm">
        {(["all", "draft", "open", "closed"] as const).map((f) => (
          <button
            key={f}
            type="button"
            onClick={() => setFilter(f)}
            className={`cursor-default ${filter === f ? "text-foreground" : "text-muted-foreground hover:text-foreground"}`}
          >
            {f}
          </button>
        ))}
      </div>
      {filteredReviews.map((review) => (
        <ReviewRow key={review.id} owner={owner} repo={repo} review={review} />
      ))}
    </div>
  );
}
