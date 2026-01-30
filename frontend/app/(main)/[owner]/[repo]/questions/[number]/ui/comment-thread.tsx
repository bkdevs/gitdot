"use client";

import { useOptimistic } from "react";
import { useUser } from "@/(main)/providers/user-provider";
import { createCommentAction } from "@/actions";
import type { CommentResponse } from "@/lib/dto";
import { CommentInput } from "./comment-input";
import { CommentRow } from "./comment-row";

export function CommentThread({
  parentType,
  parentId,
  owner,
  repo,
  number,
  comments,
}: {
  parentType: "question" | "answer";
  parentId?: string | undefined;
  owner: string;
  repo: string;
  number: number;
  comments: CommentResponse[];
}) {
  const { user } = useUser();
  const createComment = createCommentAction.bind(
    null,
    owner,
    repo,
    number,
    parentType,
    parentId,
  );
  const [optimisticComments, addOptimisticComment] = useOptimistic(
    comments,
    (state, newBody: string) => [
      ...state,
      {
        id: crypto.randomUUID(),
        parent_id: crypto.randomUUID(),
        author_id: user?.id || "",
        body: newBody,
        upvote: 0,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        user_vote: null,
        author: { id: user?.id || "", name: user?.name || "" },
      },
    ],
  );

  return (
    <div className="flex flex-col text-xs">
      <div className="my-0.5 border-b border-border" />

      {optimisticComments.map((comment) => (
        <CommentRow
          key={comment.id}
          owner={owner}
          repo={repo}
          number={number}
          comment={comment}
        />
      ))}

      <CommentInput
        createComment={createComment}
        addOptimisticComment={addOptimisticComment}
      />
    </div>
  );
}
