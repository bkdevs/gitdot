"use client";

import { createCommentAction } from "@/actions";
import type { CommentResponse } from "@/lib/dto";
import { TriangleUp } from "@/lib/icons";
import { useUser } from "@/providers/user-provider";
import { timeAgoFull } from "@/util";
import { useActionState, useOptimistic, useRef, useState } from "react";

type CommentsProps = {
  owner: string;
  repo: string;
  number: number;
  comments: CommentResponse[];
} & ({ parentType: "question" } | { parentType: "answer"; answerId: string });

export function Comments(props: CommentsProps) {
  const { comments } = props;
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
          body={comment.body}
          author={comment.author?.name ?? "Unknown"}
          score={comment.upvote}
          created_at={new Date(comment.created_at)}
        />
      ))}

      <CommentInput {...props} addOptimisticComment={addOptimisticComment} />
    </div>
  );
}

function Comment({
  body,
  author,
  score,
  created_at,
}: {
  body: string;
  author: string;
  score: number;
  created_at: Date;
}) {
  return (
    <div className="flex flex-row items-center border-border border-b py-1">
      <div className="flex flex-row items-start">
        <div className="flex flex-row items-center justify-between w-8">
          <span className="text-left text-muted-foreground">{score}</span>
          <button
            type="button"
            className="text-muted-foreground hover:text-foreground cursor-pointer mb-0.5"
          >
            <TriangleUp className="size-3" />
          </button>
        </div>
        <p className="pl-4 flex-1">
          {body}
          <span className="text-muted-foreground shrink-0">
            {" — "}
            <span className="text-blue-400 cursor-pointer">{author}</span>{" "}
            {timeAgoFull(created_at)}
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
