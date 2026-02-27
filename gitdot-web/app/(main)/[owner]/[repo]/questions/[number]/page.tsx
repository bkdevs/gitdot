import { getCurrentUser, getQuestion } from "@/dal";
import { cn } from "@/util";
import { AnswerCard } from "./ui/answer-card";
import { AnswerForm } from "./ui/answer-form";
import { AnswersStripe } from "./ui/answers-stripe";
import { QuestionCard } from "./ui/question-card";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; number: number }>;
}) {
  const { owner, repo, number } = await params;
  const [user, question] = await Promise.all([
    getCurrentUser(),
    getQuestion(owner, repo, number),
  ]);
  if (!question) return null;

  const { answers } = question;
  const hasUserAnswer = answers.find((answer) => answer.author_id === user?.id);

  return (
    <div className="w-full">
      <div className="flex flex-col flex-1 min-w-0 pb-20">
        <div className="max-w-4xl pt-4 border-border border-r">
          <QuestionCard question={question} owner={owner} repo={repo} />
        </div>

        <div className="w-full border-border border-b" />
        <div
          className={cn(
            "max-w-4xl border-border border-r",
            answers.length > 0 && "pb-4",
          )}
        >
          <AnswersStripe count={answers.length} />
        </div>
        <div className="flex flex-col max-w-4xl">
          {answers.length > 0 && (
            <div className="flex flex-col gap-12 border-border border-r border-b">
              {answers.map((answer) => (
                <AnswerCard
                  key={answer.id}
                  answer={answer}
                  owner={owner}
                  repo={repo}
                  number={question.number}
                />
              ))}
            </div>
          )}

          {!hasUserAnswer && (
            <AnswerForm owner={owner} repo={repo} number={number} />
          )}
        </div>
      </div>
    </div>
  );
}
