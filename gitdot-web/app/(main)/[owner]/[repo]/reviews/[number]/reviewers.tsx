"use client";

import type { ReviewerResource } from "gitdot-api";
import { useActionState, useState } from "react";
import {
  type AddReviewerActionResult,
  addReviewerAction,
  removeReviewerAction,
} from "@/actions/review";
import { Button } from "@/ui/button";
import { Input } from "@/ui/input";

export function Reviewers({
  owner,
  repo,
  number,
  reviewers: initialReviewers,
}: {
  owner: string;
  repo: string;
  number: number;
  reviewers: ReviewerResource[];
}) {
  const [reviewers, setReviewers] =
    useState<ReviewerResource[]>(initialReviewers);
  const [removing, setRemoving] = useState<string | null>(null);

  const [addState, addAction, isAdding] = useActionState(
    async (_prev: AddReviewerActionResult | null, formData: FormData) => {
      const result = await addReviewerAction(owner, repo, number, formData);
      if ("reviewer" in result) {
        setReviewers((prev) => [...prev, result.reviewer]);
      }
      return result;
    },
    null,
  );

  async function handleRemove(reviewerName: string) {
    setRemoving(reviewerName);
    const result = await removeReviewerAction(
      owner,
      repo,
      number,
      reviewerName,
    );
    if ("success" in result) {
      setReviewers((prev) => prev.filter((r) => r.user?.name !== reviewerName));
    }
    setRemoving(null);
  }

  return (
    <div className="flex flex-col gap-3">
      <h2 className="text-sm font-medium">Reviewers</h2>

      {reviewers.length > 0 ? (
        <div className="flex flex-col gap-2">
          {reviewers.map((reviewer) => (
            <div
              key={reviewer.id}
              className="flex items-center justify-between text-sm"
            >
              <div className="flex items-center gap-2">
                <span>{reviewer.user?.name ?? "Unknown"}</span>
                <span className="text-xs text-muted-foreground">
                  {reviewer.status}
                </span>
              </div>
              <Button
                variant="ghost"
                size="icon-sm"
                disabled={removing === reviewer.user?.name}
                onClick={() =>
                  reviewer.user?.name && handleRemove(reviewer.user.name)
                }
              >
                ×
              </Button>
            </div>
          ))}
        </div>
      ) : (
        <p className="text-sm text-muted-foreground">No reviewers yet</p>
      )}

      <form action={addAction} className="flex gap-2">
        <Input
          name="user_name"
          placeholder="Username"
          className="h-8 text-sm"
          required
        />
        <Button type="submit" variant="outline" size="sm" disabled={isAdding}>
          {isAdding ? "Adding..." : "Add"}
        </Button>
      </form>

      {addState && "error" in addState && (
        <p className="text-xs text-destructive">{addState.error}</p>
      )}
    </div>
  );
}
