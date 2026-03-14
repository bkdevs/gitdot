"use server";

import type {
  PublishReviewRequest,
  ReviewerResource,
  ReviewResource,
} from "gitdot-api";
import { refresh } from "next/cache";
import { addReviewer, publishReview, removeReviewer } from "@/dal";

export type AddReviewerActionResult =
  | { reviewer: ReviewerResource }
  | { error: string };

export async function addReviewerAction(
  owner: string,
  repo: string,
  number: number,
  formData: FormData,
): Promise<AddReviewerActionResult> {
  const userName = formData.get("user_name") as string;
  if (!userName) {
    return { error: "Username is required" };
  }

  const result = await addReviewer(owner, repo, number, {
    user_name: userName,
  });
  if (!result) {
    return { error: "addReviewer call failed" };
  }

  refresh();
  return { reviewer: result };
}

export type RemoveReviewerActionResult = { success: true } | { error: string };

export async function removeReviewerAction(
  owner: string,
  repo: string,
  number: number,
  reviewerName: string,
): Promise<RemoveReviewerActionResult> {
  try {
    await removeReviewer(owner, repo, number, reviewerName);
  } catch {
    return { error: "removeReviewer call failed" };
  }

  refresh();
  return { success: true };
}

export type PublishReviewActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function publishReviewAction(
  owner: string,
  repo: string,
  number: number,
  request: PublishReviewRequest,
): Promise<PublishReviewActionResult> {
  const result = await publishReview(owner, repo, number, request);
  if (!result) {
    return { error: "publishReview call failed" };
  }

  refresh();
  return { review: result };
}
