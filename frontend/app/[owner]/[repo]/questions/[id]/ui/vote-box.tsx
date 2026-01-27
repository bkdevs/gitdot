import { TriangleUp, TriangleDown } from "@/lib/icons";

export function VoteBox({ score }: { score: number }) {
  return (
    <div className="flex flex-col items-center gap-1 text-muted-foreground text-xl">
      <button className="text-muted-foreground hover:text-foreground cursor-pointer">
        <TriangleUp />
      </button>
      <span>{score}</span>
      <button className="text-muted-foreground hover:text-foreground cursor-pointer">
        <TriangleDown />
      </button>
    </div>
  );
}
