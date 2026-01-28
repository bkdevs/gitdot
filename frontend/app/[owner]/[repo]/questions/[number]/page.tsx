import { getQuestion } from "@/lib/dal";
import { AnswerCard } from "./ui/answer-card";
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
    <div className="w-full h-screen">
      <div className="flex flex-col max-w-4xl py-4">
        <QuestionCard question={question} />
        <div className="ml-3 my-8">
          <AnswersStripe count={question.answers.length} />
        </div>
        {question.answers.map((answer) => (
          <AnswerCard key={answer.id} answer={answer} />
        ))}
      </div>
    </div>
  );
}
