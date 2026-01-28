import { Answers } from "./ui/answers";
import { QuestionCard } from "./ui/question-card";

export default function Page({ params }: { params: { id: string } }) {
  return (
    <div className="w-full h-screen">
      <div className="flex flex-col max-w-4xl px-6 py-4">
        <QuestionCard />
        <Answers />
      </div>
    </div>
  );
}
