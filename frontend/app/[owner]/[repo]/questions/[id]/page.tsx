import { Answers } from "./ui/answers";
import { Question } from "./ui/question";

export default function Page({ params }: { params: { id: string } }) {
  return (
    <div className="flex flex-col w-full h-screen">
      <div className="flex-1 flex-col pb-20">
        <Question />
        <Answers />
      </div>
    </div>
  );
}
