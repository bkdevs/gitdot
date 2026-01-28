import { MarkdownBody } from "@/[owner]/[repo]/ui/markdown/markdown-body";
import type { AnswerResponse } from "@/lib/dto/question";
import { timeAgoFull } from "@/util";
import { Comments } from "./comments";
import { VoteBox } from "./vote-box";

export function AnswerCard({ answer }: { answer: AnswerResponse }) {
  return (
    <div className="flex">
      <VoteBox score={answer.upvote} />
      <div className="flex-1">
        <MarkdownBody content={answer.body} />

        <div className="flex mt-8 justify-between text-sm">
          <div className="flex gap-4">
            <span>
              <span className="text-muted-foreground">Asked</span>{" "}
              {timeAgoFull(new Date(answer.created_at))}
            </span>
          </div>
          <div className="flex gap-2 text-muted-foreground">
            <span className="underline">{answer.author.name}</span>
            <span>1,234</span>
          </div>
        </div>

        <div className="mt-2 mb-2 border border-gray-100" />
        <Comments />
      </div>
    </div>
  );
}
