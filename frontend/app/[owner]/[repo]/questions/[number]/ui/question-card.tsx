"use client";

import { MarkdownBody } from "@/[owner]/[repo]/ui/markdown/markdown-body";
import { updateQuestionAction } from "@/actions";
import type { QuestionResponse } from "@/lib/dto/question";
import { formatDate, timeAgoFull } from "@/util";
import { CommentThread } from "./comment-thread";
import { QuestionDropdown } from "./question-dropdown";
import { VoteBox } from "./vote-box";

type QuestionCardProps = {
  question: QuestionResponse;
  owner: string;
  repo: string;
};

export function QuestionCard({ question, owner, repo }: QuestionCardProps) {
  const wasUpdated = question.created_at !== question.updated_at;
  const updateQuestion = updateQuestionAction.bind(
    null,
    owner,
    repo,
    question.number,
  );

  return (
    <div className="flex group">
      <VoteBox
        targetType="question"
        owner={owner}
        repo={repo}
        number={question.number}
        score={question.upvote}
        userVote={question.user_vote}
      />
      <div className="flex-1">
        <div className="flex items-start justify-between">
          <h1 className="text-xl font-medium">{question.title}</h1>
          <QuestionDropdown owner={owner} repo={repo} question={question} />
        </div>
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
