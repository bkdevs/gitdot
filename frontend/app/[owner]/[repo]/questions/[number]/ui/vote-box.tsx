"use client";

import { voteAction } from "@/actions";
import { TriangleDown, TriangleUp } from "@/lib/icons";
import { cn } from "@/util";
import { useActionState, useOptimistic } from "react";

type VoteBoxProps = {
  score: number;
  userVote: number | null;
  owner: string;
  repo: string;
  number: number;
} & ({ type: "question" } | { type: "answer"; answerId: string });

export function VoteBox(props: VoteBoxProps) {
  const { score, userVote } = props;

  const [optimistic, setOptimistic] = useOptimistic(
    { score, userVote },
    (state, newValue: number) => ({
      score: state.score + newValue - (state.userVote ?? 0),
      userVote: newValue || null,
    }),
  );

  const [, formAction] = useActionState(
    async (_prev: null, formData: FormData) => {
      const clickedVote = Number(formData.get("clickedVote")) as 1 | -1;
      const newValue = optimistic.userVote === clickedVote ? 0 : clickedVote;

      formData.set("value", String(newValue));
      setOptimistic(newValue);

      await voteAction(formData);
      return null;
    },
    null,
  );

  return (
    <div className="flex flex-col mx-6 mt-1.75 items-center gap-1 text-muted-foreground text-xl">
      <form action={formAction}>
        <input type="hidden" name="owner" value={props.owner} />
        <input type="hidden" name="repo" value={props.repo} />
        <input type="hidden" name="number" value={props.number} />
        <input type="hidden" name="targetType" value={props.type} />
        {props.type === "answer" && (
          <input type="hidden" name="answerId" value={props.answerId} />
        )}
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
      <form action={formAction}>
        <input type="hidden" name="owner" value={props.owner} />
        <input type="hidden" name="repo" value={props.repo} />
        <input type="hidden" name="number" value={props.number} />
        <input type="hidden" name="targetType" value={props.type} />
        {props.type === "answer" && (
          <input type="hidden" name="answerId" value={props.answerId} />
        )}
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
