import { getQuestions } from "@/lib/dal";
import { QuestionsClient } from "./ui/questions-client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const questions = await getQuestions(owner, repo);
  if (!questions) return null;

  return (
    <QuestionsClient
      owner={owner}
      repo={repo}
      questions={questions.questions}
    />
  );
}
