"use client";

import { useActionState, useOptimistic } from "react";
import { useAuthBlocker } from "@/(main)/providers/auth-blocker-provider";
import { type VoteActionResult, voteAction } from "@/actions";
import { TriangleDown, TriangleUp } from "@/icons";
import { cn } from "@/util";

export function VoteBox({
  targetId,
  targetType,
  owner,
  repo,
  number,
  score,
  userVote,
  className,
  iconClassName,
}: {
  targetType: "question" | "answer";
  targetId?: string | undefined;
  owner: string;
  repo: string;
  number: number;
  score: number;
  userVote: number | null;
  className?: string;
  iconClassName?: string;
}) {
  const [optimistic, setOptimistic] = useOptimistic(
    { score, userVote },
    (state, newValue: number) => ({
      score: state.score + newValue - (state.userVote ?? 0),
      userVote: newValue || null,
    }),
  );

  const { requireAuth } = useAuthBlocker();
  const vote = voteAction.bind(null, owner, repo, number, targetId, targetType);
  const [, formAction] = useActionState(
    async (_prev: VoteActionResult | null, formData: FormData) => {
      if (requireAuth()) return null;

      const clickedVote = Number(formData.get("clickedVote")) as 1 | -1;
      const newValue = optimistic.userVote === clickedVote ? 0 : clickedVote;
      formData.set("value", String(newValue));
      setOptimistic(newValue);

      return await vote(formData);
    },
    null,
  );

  return (
    <div
      className={cn(
        "flex flex-col items-center text-muted-foreground",
        "mx-6 mt-1.75 gap-1 text-xl",
        className,
      )}
    >
      <form action={formAction} className="contents">
        <input type="hidden" name="clickedVote" value={1} />
        <button
          type="submit"
          className={cn(
            "cursor-pointer transition-colors",
            optimistic.userVote === 1
              ? "text-upvote"
              : "text-vote hover:text-upvote",
          )}
        >
          <TriangleUp className={iconClassName} />
        </button>
      </form>
      <span
        className={cn(
          "w-6 text-center",
          optimistic.userVote === 1 && "text-upvote",
          optimistic.userVote === -1 && "text-downvote",
        )}
      >
        {optimistic.score}
      </span>
      <form action={formAction} className="contents">
        <input type="hidden" name="clickedVote" value={-1} />
        <button
          type="submit"
          className={cn(
            "cursor-pointer transition-colors",
            optimistic.userVote === -1
              ? "text-downvote"
              : "text-vote hover:text-downvote",
          )}
        >
          <TriangleDown className={iconClassName} />
        </button>
      </form>
    </div>
  );
}
