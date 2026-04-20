"use client";

import type { ReviewResource } from "gitdot-api";
import { createContext, useContext } from "react";

type ReviewContext = {
  review: ReviewResource;
};
const ReviewContext = createContext<ReviewContext | null>(null);

export function ReviewProvider({
  review,
  children,
}: {
  review: ReviewResource;
  children: React.ReactNode;
}) {
  return <ReviewContext value={{ review }}>{children}</ReviewContext>;
}

export function useReviewContext(): ReviewContext {
  const ctx = useContext(ReviewContext);
  if (!ctx)
    throw new Error("useReviewContext must be used within ReviewProvider");
  return ctx;
}
