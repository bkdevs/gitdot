import { MarkdownBody } from "@/[owner]/[repo]/ui/markdown/markdown-body";
import type { AnswerResponse } from "@/lib/dto/question";
import { timeAgoFull } from "@/util";
import { CommentThread } from "./comment-thread";
import { VoteBox } from "./vote-box";

type AnswerCardProps = {
  answer: AnswerResponse;
  owner: string;
  repo: string;
  number: number;
};

export function AnswerCard({ answer, owner, repo, number }: AnswerCardProps) {
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
        <MarkdownBody content={answer.body} />

        <div className="flex justify-between text-xs">
          <div className="flex gap-4">
            <span>
              <span className="text-muted-foreground">Asked</span>{" "}
              {timeAgoFull(new Date(answer.created_at))}
            </span>
          </div>
          <div className="flex gap-2 text-muted-foreground">
            <span className="underline">{answer.author?.name}</span>
            <span>1,234</span>
          </div>
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
