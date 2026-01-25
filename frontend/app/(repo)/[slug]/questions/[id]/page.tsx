import { QuestionHeader } from "./ui/question-header";
import { Question } from "./ui/question";
import { Answers } from "./ui/answers";

export default function Page({ params }: { params: { id: string } }) {
  return (
    <div className="flex flex-col w-full h-screen">
      <QuestionHeader />
      <div className="flex-1 flex-col overflow-y-auto scrollbar-thin pb-20">
        <Question />
        <Answers />
      </div>
    </div>
  );
}
