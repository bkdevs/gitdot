import type { QuestionResponse } from "@/lib/dto";
import Link from "@/ui/link";

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
      className="flex w-full border-b hover:bg-accent/50 select-none cursor-default py-2 px-2"
      prefetch={true}
    >
      <div className="flex flex-col w-full justify-start items-start min-w-0">
        <div className="text-sm truncate w-full">
          #{question.number}: {question.title}
        </div>
      </div>
    </Link>
  );
}
