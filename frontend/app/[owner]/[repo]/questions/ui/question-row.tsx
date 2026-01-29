import type { QuestionResponse } from "@/lib/dto";
import { TriangleDown, TriangleUp } from "@/lib/icons";
import Link from "@/ui/link";
import { pluralize, timeAgo } from "@/util";

export function QuestionRow({
  owner,
  repo,
  question,
}: {
  owner: string;
  repo: string;
  question: QuestionResponse;
}) {
  return (
    <Link
      href={`/${owner}/${repo}/questions/${question.number}`}
      className="flex flex-row w-full border-b hover:bg-accent/50 select-none cursor-default py-2 h-18"
      prefetch={true}
    >
      <div className="flex flex-col mx-4 mt-0.5">
        <div className="flex flex-col items-center gap-0.5 text-muted-foreground text-sm">
          <button
            type="submit"
            className="text-muted-foreground/50 hover:text-foreground cursor-pointer"
          >
            <TriangleUp className="size-3" />
          </button>
          <span>{question.upvote}</span>
          <button
            type="submit"
            className="text-muted-foreground/50 hover:text-foreground cursor-pointer"
          >
            <TriangleDown className="size-3" />
          </button>
        </div>
      </div>
      <div className="flex flex-col w-full justify-start items-start">
        <div className="text-xs text-muted-foreground flex items-center gap-1">
          <span className="truncate min-w-0">{question.author.name}</span>•
          <span>{timeAgo(new Date(question.created_at))}</span>
        </div>
        <div className="text-[15px] truncate">{question.title}</div>
        <div className="flex flex-row gap-1 text-xs text-muted-foreground">
          {pluralize(question.answers.length, "answer")}
          <span>•</span>
          {pluralize(question.comments.length, "comment")}
          <span>•</span>
          {pluralize(question.impression, "view")}
        </div>
      </div>
    </Link>
  );
}
