"use client";

import { useActionState, useOptimistic } from "react";
import { voteAction } from "@/actions";
import type { CommentResponse } from "@/lib/dto";
import { TriangleUp } from "@/lib/icons";
import { cn, timeAgoFull } from "@/util";

export function CommentRow({
  owner,
  repo,
  number,
  comment,
}: {
  owner: string;
  repo: string;
  number: number;
  comment: CommentResponse;
}) {
  const { id, body, author, upvote, user_vote, created_at } = comment;
  const voteComment = voteAction.bind(null, owner, repo, number, id, "comment");

  const [optimistic, setOptimistic] = useOptimistic(
    { upvote, user_vote },
    (state, newValue: number) => ({
      upvote: state.upvote + newValue - (state.user_vote ?? 0),
      user_vote: newValue || null,
    }),
  );

  const [, formAction] = useActionState(
    async (_prev: null, formData: FormData) => {
      const newValue = optimistic.user_vote === 1 ? 0 : 1;
      formData.set("value", String(newValue));
      setOptimistic(newValue);
      await voteComment(formData);
      return null;
    },
    null,
  );

  return (
    <div className="flex flex-row items-center border-border border-b py-1">
      <div className="flex flex-row items-start">
        <div className="flex flex-row items-center justify-between w-8">
          <span
            className={cn(
              "text-left",
              optimistic.user_vote === 1
                ? "text-orange-500"
                : "text-muted-foreground",
            )}
          >
            {optimistic.upvote}
          </span>
          <form action={formAction} className="contents">
            <button
              type="submit"
              className={cn(
                "cursor-pointer 0 transition-colors",
                optimistic.user_vote === 1
                  ? "text-orange-500"
                  : "text-muted-foreground hover:text-foreground",
              )}
            >
              <TriangleUp className="mb-0.5 size-3" />
            </button>
          </form>
        </div>
        <p className="pl-4 flex-1">
          {body}
          <span className="text-muted-foreground shrink-0">
            {" — "}
            <span className="text-blue-400 cursor-pointer">{author?.name}</span>{" "}
            {timeAgoFull(new Date(created_at))}
          </span>
        </p>
      </div>
    </div>
  );
}
