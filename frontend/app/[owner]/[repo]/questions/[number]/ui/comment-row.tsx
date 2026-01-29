"use client";

import { voteAction } from "@/actions";
import type { CommentResponse } from "@/lib/dto";
import { TriangleUp } from "@/lib/icons";
import { cn, timeAgoFull } from "@/util";
import { Check, Edit3 } from "lucide-react";
import { useActionState, useOptimistic, useState } from "react";

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
  const { body, author, created_at } = comment;

  const [editing, setEditing] = useState(false);
  const [newBody, setNewBody] = useState(body);

  return (
    <div
      className={cn(
        "flex group flex-row justify-between items-center border-border border-b py-1 pr-1",
      )}
    >
      <div className="flex flex-row items-start flex-1">
        <CommentVote
          owner={owner}
          repo={repo}
          number={number}
          comment={comment}
        />

        <div className="pl-4" />

        {editing ? (
          <input
            type="text"
            className="flex-1 w-full ring-0 outline-0"
            value={newBody}
            onChange={(e) => setNewBody(e.target.value)}
            onBlur={() => {
              setEditing(false);
              setNewBody(body);
            }}
            onKeyDown={(e) => {
              if (e.key === "Escape") {
                setEditing(false);
                setNewBody(body);
              }
            }}
            autoFocus
          />
        ) : (
          <p className="flex-1">
            {body}
            <span className="text-muted-foreground shrink-0">
              {" — "}
              <span className="text-blue-400 cursor-pointer">
                {author?.name}
              </span>{" "}
              {timeAgoFull(new Date(created_at))}
            </span>
          </p>
        )}
      </div>
      <div className="shrink-0">
        {editing ? (
          <Check
            className="size-3 hover:text-foreground hover:stroke-3"
            onClick={() => setEditing(true)}
          />
        ) : (
          <Edit3
            className="size-3 opacity-0 group-hover:opacity-100 transition-opacity hover:text-foreground hover:stroke-3"
            onClick={() => setEditing(true)}
          />
        )}
      </div>
    </div>
  );
}

function CommentVote({
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
  const { id, upvote, user_vote } = comment;
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
  );
}
