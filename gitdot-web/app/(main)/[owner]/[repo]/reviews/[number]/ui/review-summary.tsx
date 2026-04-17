"use client";

import { use } from "react";
import type { ResourcePromisesType } from "@/(main)/[owner]/[repo]/resources";
import type { ViewMode } from "../layout.client";
import type { Resources } from "../layout";
import { ReviewSummaryBody } from "./review-summary-body";
import { ReviewSummaryHeader } from "./review-summary-header";

type ResourcePromises = ResourcePromisesType<Resources>;

export function ReviewSummary({
  owner,
  repo,
  promises,
  view,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
  view: ViewMode;
}) {
  const review = use(promises.review);
  if (!review) return null;

  return (
    <div className="flex flex-col w-full h-full overflow-auto">
      <ReviewSummaryHeader review={review} />
      {view !== "diff" && (
        <ReviewSummaryBody owner={owner} repo={repo} review={review} />
      )}
    </div>
  );
}
