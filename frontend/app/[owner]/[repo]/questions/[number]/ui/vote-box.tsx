"use client";

import { useActionState, useOptimistic } from "react";
import { voteAction, type VoteActionResult } from "@/actions";
import { TriangleDown, TriangleUp } from "@/lib/icons";
import { cn } from "@/util";

export function VoteBox({
  targetId,
  targetType,
  owner,
  repo,
  number,
  score,
  userVote,
}: {
  targetType: "question" | "answer";
  targetId?: string | undefined;
  owner: string;
  repo: string;
  number: number;
  score: number;
  userVote: number | null;
}) {
  const [optimistic, setOptimistic] = useOptimistic(
    { score, userVote },
    (state, newValue: number) => ({
      score: state.score + newValue - (state.userVote ?? 0),
      userVote: newValue || null,
    }),
  );

  const vote = voteAction.bind(null, owner, repo, number, targetId, targetType);
  const [, formAction] = useActionState(
    async (_prev: VoteActionResult | null, formData: FormData) => {
      const clickedVote = Number(formData.get("clickedVote")) as 1 | -1;
      const newValue = optimistic.userVote === clickedVote ? 0 : clickedVote;
      formData.set("value", String(newValue));
      setOptimistic(newValue);

      return await vote(formData);
    },
    null,
  );

  return (
    <div className="flex flex-col mx-6 mt-1.75 gap-1 items-center text-muted-foreground text-xl">
      <form action={formAction} className="contents">
        <input type="hidden" name="clickedVote" value={1} />
        <button
          type="submit"
          className={cn(
            "cursor-pointer transition-colors",
            optimistic.userVote === 1
              ? "text-orange-500"
              : "text-muted-foreground hover:text-foreground",
          )}
        >
          <TriangleUp />
        </button>
      </form>
      <span
        className={cn(
          optimistic.userVote === 1 && "text-orange-500",
          optimistic.userVote === -1 && "text-blue-500",
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
              ? "text-blue-500"
              : "text-muted-foreground hover:text-foreground",
          )}
        >
          <TriangleDown />
        </button>
      </form>
    </div>
  );
}
