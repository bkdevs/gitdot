"use client";

import type { QuestionResource } from "gitdot-api";
import { MarkdownBody } from "@/(main)/[owner]/[repo]/ui/markdown/markdown-body";
import { useUser } from "@/(main)/providers/user-provider";
import { formatDate, timeAgoFull } from "@/util";
import { CommentThread } from "./comment-thread";
import { QuestionDropdown } from "./question-dropdown";
import { VoteBox } from "./vote-box";

type QuestionCardProps = {
  question: QuestionResource;
  owner: string;
  repo: string;
};

export function QuestionCard({ question, owner, repo }: QuestionCardProps) {
  const { user } = useUser();
  const wasUpdated = question.created_at !== question.updated_at;
  const isOwner = user?.id === question.author_id;

  return (
    <div className="flex pb-4">
      <VoteBox
        targetType="question"
        owner={owner}
        repo={repo}
        number={question.number}
        score={question.upvote}
        userVote={question.user_vote}
      />
      <div className="flex-1">
        <div className="flex flex-col group relative">
          {isOwner && (
            <div className="absolute top-0 right-0">
              <QuestionDropdown owner={owner} repo={repo} question={question} />
            </div>
          )}
          <h1 className="text-xl font-medium pr-8">{question.title}</h1>
          <MarkdownBody content={question.body} />

          <div className="flex flex-row gap-1 items-center text-xs text-muted-foreground">
            <span className="text-blue-400 cursor-pointer">
              {question.author?.name ?? "unknown"}
            </span>
            <span>
              <span className="text-muted-foreground">asked</span>{" "}
              {formatDate(new Date(question.created_at))}
              {", "}
              {wasUpdated ? (
                <>
                  <span className="text-muted-foreground">updated</span>{" "}
                  {timeAgoFull(new Date(question.updated_at))}
                </>
              ) : (
                timeAgoFull(new Date(question.created_at))
              )}
            </span>
          </div>
        </div>
        <CommentThread
          parentType="question"
          owner={owner}
          repo={repo}
          number={question.number}
          comments={question.comments}
        />
      </div>
    </div>
  );
}
