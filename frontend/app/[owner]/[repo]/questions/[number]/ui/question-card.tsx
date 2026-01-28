import { MarkdownBody } from "@/[owner]/[repo]/ui/markdown/markdown-body";
import type { QuestionResponse } from "@/lib/dto/question";
import { timeAgoFull } from "@/util";
import { Comments } from "./comments";
import { VoteBox } from "./vote-box";

export function QuestionCard({ question }: { question: QuestionResponse }) {
  return (
    <div className="flex">
      <VoteBox score={question.upvote} />
      <div className="flex-1">
        <h1 className="text-xl font-medium mb-2">{question.title}</h1>
        <MarkdownBody content={question.body} />

        <div className="flex justify-between text-xs">
          <div className="flex gap-4">
            <span>
              <span className="text-muted-foreground">Asked</span>{" "}
              {timeAgoFull(new Date(question.created_at))}
            </span>
            <span>
              <span className="text-muted-foreground">Modified</span>{" "}
              {timeAgoFull(new Date(question.updated_at))}
            </span>
          </div>
          <div className="flex gap-2 text-muted-foreground">
            <span className="underline">{question.author.name}</span>
            <span>1,234</span>
          </div>
        </div>

        <div className="my-0.5 border-b border-border" />
        <Comments />
      </div>
    </div>
  );
}
