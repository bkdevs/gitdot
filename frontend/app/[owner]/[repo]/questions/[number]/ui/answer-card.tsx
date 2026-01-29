"use client";

import { MarkdownBody } from "@/[owner]/[repo]/ui/markdown/markdown-body";
import type { AnswerResponse } from "@/lib/dto/question";
import { formatDate, timeAgoFull } from "@/util";
import { AnswerDropdown } from "./answer-dropdown";
import { CommentThread } from "./comment-thread";
import { VoteBox } from "./vote-box";
import { useUser } from "@/providers/user-provider";

type AnswerCardProps = {
  answer: AnswerResponse;
  owner: string;
  repo: string;
  number: number;
};

export function AnswerCard({ answer, owner, repo, number }: AnswerCardProps) {
  const wasUpdated = answer.created_at !== answer.updated_at;
  const user = useUser();

  return (
    <div className="flex">
      <VoteBox
        targetType="answer"
        targetId={answer.id}
        owner={owner}
        repo={repo}
        number={number}
        score={answer.upvote}
        userVote={answer.user_vote}
      />
      <div className="flex-1">
        <div className="flex flex-col group relative">
          {user?.id === answer.author_id && (
            <div className="absolute top-0 right-0">
              <AnswerDropdown
              owner={owner}
              repo={repo}
              number={number}
              answer={answer}
              />
            </div>
          )}
          <MarkdownBody content={answer.body} />
        </div>

        <div className="flex flex-row gap-1 items-center text-xs text-muted-foreground">
          <span className="text-blue-400 cursor-pointer">
            {answer.author?.name ?? "unknown"}
          </span>
          <span>
            <span className="text-muted-foreground">answered</span>{" "}
            {formatDate(new Date(answer.created_at))}
            {", "}
            {wasUpdated ? (
              <>
                <span className="text-muted-foreground">updated</span>{" "}
                {timeAgoFull(new Date(answer.updated_at))}
              </>
            ) : (
              timeAgoFull(new Date(answer.created_at))
            )}
          </span>
        </div>

        <CommentThread
          parentType="answer"
          parentId={answer.id}
          owner={owner}
          repo={repo}
          number={number}
          comments={answer.comments}
        />
      </div>
    </div>
  );
}
