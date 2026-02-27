"use client";

import type { CommentResource } from "gitdot-api";
import { Check, Edit3 } from "lucide-react";
import { useActionState, useOptimistic, useRef, useState } from "react";
import { useAuthBlocker } from "@/(main)/providers/auth-blocker-provider";
import { useUser } from "@/(main)/providers/user-provider";
import {
  type UpdateCommentActionResult,
  updateCommentAction,
  type VoteActionResult,
  voteAction,
} from "@/actions";
import { TriangleUp } from "@/icons";
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
  comment: CommentResource;
}) {
  const { body, author, created_at } = comment;
  const [editing, setEditing] = useState(false);
  const formRef = useRef<HTMLFormElement>(null);
  const { user } = useUser();
  const isOwner = user?.id === comment.author_id;

  const updateComment = updateCommentAction.bind(
    null,
    owner,
    repo,
    number,
    comment.id,
  );

  const [optimisticBody, setOptimisticBody] = useOptimistic(
    body,
    (_, newBody: string) => newBody,
  );

  const [, formAction] = useActionState(
    async (_prev: UpdateCommentActionResult, formData: FormData) => {
      const newBody = formData.get("body") as string;
      setOptimisticBody(newBody);
      return await updateComment(formData);
    },
    { comment: comment },
  );

  return (
    <div
      className={cn(
        "flex group flex-row justify-between items-center border-b py-1 transition-colors duration-200",
        editing ? "border-primary" : "border-border",
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
          <form ref={formRef} action={formAction} className="contents">
            <input
              type="text"
              name="body"
              className="flex-1 w-full ring-0 outline-0"
              defaultValue={optimisticBody}
              onBlur={() => {
                setEditing(false);
              }}
              onKeyDown={(e) => {
                if (e.key === "Escape") {
                  setEditing(false);
                } else if (e.key === "Enter") {
                  e.preventDefault();
                  setEditing(false);
                  formRef.current?.requestSubmit();
                }
              }}
              autoFocus
            />
          </form>
        ) : (
          <p className="flex-1">
            {optimisticBody}
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
      {isOwner && (
        <div className="shrink-0">
          {editing ? (
            <Check
              className="size-3 hover:text-foreground hover:stroke-3"
              onMouseDown={(e) => {
                e.preventDefault();
                setEditing(false);
                formRef.current?.requestSubmit();
              }}
            />
          ) : (
            <Edit3
              className="size-3 opacity-0 group-hover:opacity-100 transition-opacity hover:text-foreground hover:stroke-3"
              onClick={() => setEditing(true)}
            />
          )}
        </div>
      )}
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
  comment: CommentResource;
}) {
  const { id, upvote, user_vote } = comment;
  const { requireAuth } = useAuthBlocker();
  const voteComment = voteAction.bind(null, owner, repo, number, id, "comment");
  const [optimistic, setOptimistic] = useOptimistic(
    { upvote, user_vote },
    (state, newValue: number) => ({
      upvote: state.upvote + newValue - (state.user_vote ?? 0),
      user_vote: newValue || null,
    }),
  );

  const [, formAction] = useActionState(
    async (_prev: VoteActionResult | null, formData: FormData) => {
      if (requireAuth()) return null;
      const newValue = optimistic.user_vote === 1 ? 0 : 1;
      formData.set("value", String(newValue));
      setOptimistic(newValue);
      return await voteComment(formData);
    },
    null,
  );

  return (
    <div className="flex flex-row items-center justify-between w-7">
      <span
        className={cn(
          "text-left transition-colors",
          optimistic.user_vote === 1 ? "text-upvote" : "text-muted-foreground",
        )}
      >
        {" "}
        {optimistic.upvote}
      </span>
      <form action={formAction} className="contents">
        <button
          type="submit"
          className={cn(
            "cursor-pointer transition-colors",
            optimistic.user_vote === 1
              ? "text-upvote"
              : "text-vote hover:text-upvote",
          )}
        >
          <TriangleUp className="mb-0.5 size-3" />
        </button>
      </form>
    </div>
  );
}
