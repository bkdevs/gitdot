import { ChevronUp, ChevronDown } from "lucide-react";

export function VoteBox({ score }: { score: number }) {
  return (
    <div className="flex flex-col items-center gap-0 text-muted-foreground text-xl">
      <button className="text-muted-foreground hover:text-foreground">
        <ChevronUp />
      </button>
      <span>{score}</span>
      <button className="text-muted-foreground hover:text-foreground">
        <ChevronDown />
      </button>
    </div>
  );
}
