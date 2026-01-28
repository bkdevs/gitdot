"use client";

import { createCommentAction, voteAction } from "@/actions";
import type { CommentResponse } from "@/lib/dto";
import { TriangleUp } from "@/lib/icons";
import { useUser } from "@/providers/user-provider";
import { cn, timeAgoFull } from "@/util";
import { useActionState, useOptimistic, useRef, useState } from "react";

type CommentsProps = {
  owner: string;
  repo: string;
  number: number;
  comments: CommentResponse[];
} & ({ parentType: "question" } | { parentType: "answer"; answerId: string });

export function Comments(props: CommentsProps) {
  const { owner, repo, number, comments } = props;
  const user = useUser();
  if (!user) return null; // TODO: block input but show comments for unauthenticated

  const [optimisticComments, addOptimisticComment] = useOptimistic(
    comments,
    (state, newBody: string) => [
      ...state,
      {
        id: crypto.randomUUID(),
        parent_id: crypto.randomUUID(),
        author_id: user.id,
        body: newBody,
        upvote: 0,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        user_vote: null,
        author: { id: user.id, name: user.name },
      },
    ],
  );

  return (
    <div className="flex flex-col text-xs">
      <div className="my-0.5 border-b border-border" />

      {optimisticComments.map((comment) => (
        <Comment
          key={comment.id}
          owner={owner}
          repo={repo}
          number={number}
          comment={comment}
        />
      ))}

      <CommentInput {...props} addOptimisticComment={addOptimisticComment} />
    </div>
  );
}

function Comment({
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
      await voteAction(formData);
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
          <form action={formAction}>
            <input type="hidden" name="owner" value={owner} />
            <input type="hidden" name="repo" value={repo} />
            <input type="hidden" name="number" value={number} />
            <input type="hidden" name="targetType" value="comment" />
            <input type="hidden" name="commentId" value={id} />
            <button
              type="submit"
              className={cn(
                "cursor-pointer 0 transition-colors",
                optimistic.user_vote === 1
                  ? "text-orange-500"
                  : "text-muted-foreground hover:text-foreground",
              )}
            >
              <TriangleUp className="-mb-0.25 size-3" />
            </button>
          </form>
        </div>
        <p className="pl-4 flex-1">
          {body}
          <span className="text-muted-foreground shrink-0">
            {" — "}
            <span className="text-blue-400 cursor-pointer">{author.name}</span>{" "}
            {timeAgoFull(new Date(created_at))}
          </span>
        </p>
      </div>
    </div>
  );
}

function CommentInput(
  props: CommentsProps & { addOptimisticComment: (body: string) => void },
) {
  const [showInput, setShowInput] = useState(false);
  const formRef = useRef<HTMLFormElement>(null);

  const [, formAction] = useActionState(
    async (_prevState: { error?: string } | null, formData: FormData) => {
      const body = formData.get("body") as string;
      formRef.current?.reset();
      (document.activeElement as HTMLElement)?.blur();
      setShowInput(false);
      addOptimisticComment(body);

      const result = await createCommentAction(formData);
      if (!result.success) {
        return { error: result.error };
      }
      return null;
    },
    null,
  );

  const { owner, repo, number, parentType, addOptimisticComment } = props;
  const answerId = parentType === "answer" ? props.answerId : null;

  return (
    <div className="flex flex-row w-full pt-1">
      {showInput ? (
        <form ref={formRef} action={formAction} className="w-full">
          <input type="hidden" name="owner" value={owner} />
          <input type="hidden" name="repo" value={repo} />
          <input type="hidden" name="number" value={number} />
          <input type="hidden" name="parentType" value={parentType} />
          {answerId && <input type="hidden" name="answerId" value={answerId} />}
          <input
            className="border-b border-bg ring-0 outline-none h-5 w-full"
            type="text"
            name="body"
            placeholder="Write comment..."
            autoFocus
            onBlur={(e) => {
              if (e.target.value.length === 0) {
                setShowInput(false);
              }
            }}
            onKeyDown={(e) => {
              if (e.key === "Escape") {
                setShowInput(false);
              } else if (
                e.key === "Enter" &&
                e.currentTarget.value.length === 0
              ) {
                e.preventDefault();
              }
            }}
          />
        </form>
      ) : (
        <button
          type="button"
          className="underline text-muted-foreground cursor-pointer h-5"
          onClick={() => setShowInput(true)}
        >
          Add comment..
        </button>
      )}
    </div>
  );
}
