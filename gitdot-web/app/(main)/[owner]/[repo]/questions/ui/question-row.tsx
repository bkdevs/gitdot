import type { QuestionResource } from "gitdot-api";
import Link from "@/ui/link";
import { pluralize, timeAgo } from "@/util";
import { VoteBox } from "../[number]/ui/vote-box";

export function QuestionRow({
  owner,
  repo,
  question,
}: {
  owner: string;
  repo: string;
  question: QuestionResource;
}) {
  return (
    <div className="flex flex-row w-full border-b hover:bg-accent/50 select-none cursor-default py-2 h-18">
      <VoteBox
        targetType="question"
        owner={owner}
        repo={repo}
        number={question.number}
        score={question.upvote}
        userVote={question.user_vote}
        className="mx-4 mt-0.5 gap-0.5 text-sm"
        iconClassName="size-3"
      />
      <Link
        href={`/${owner}/${repo}/questions/${question.number}`}
        className="flex flex-col w-full justify-start items-start"
        prefetch={true}
      >
        <div className="text-xs text-muted-foreground flex items-center gap-1">
          <span className="truncate min-w-0">{question.author?.name}</span>•
          <span>{timeAgo(new Date(question.created_at))}</span>
        </div>
        <div className="text-sm truncate">{question.title}</div>
        <div className="flex flex-row gap-1 text-xs text-muted-foreground">
          {pluralize(question.answers.length, "answer")}
          <span>•</span>
          {pluralize(question.comments.length, "comment")}
          <span>•</span>
          {pluralize(question.impression, "view")}
        </div>
      </Link>
    </div>
  );
}
