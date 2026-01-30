import { getQuestion } from "@/lib/dal";
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
  const question = await getQuestion(owner, repo, number);
  if (!question) return null;

  return (
    <div className="w-full">
      <div className="flex flex-col flex-1 min-w-0 pb-20">
        <div className="max-w-4xl mt-4">
          <QuestionCard question={question} owner={owner} repo={repo} />
        </div>

        <AnswersStripe count={question.answers.length} />
        <div className="flex flex-col max-w-4xl">
          <div className="flex flex-col gap-12">
            {question.answers.map((answer) => (
              <AnswerCard
                key={answer.id}
                answer={answer}
                owner={owner}
                repo={repo}
                number={question.number}
              />
            ))}
          </div>

          <AnswerForm
            owner={owner}
            repo={repo}
            number={number}
            answers={question.answers}
          />
        </div>
      </div>
    </div>
  );
}
