import { TriangleDown, TriangleUp } from "@/lib/icons";

export function VoteBox({ score }: { score: number }) {
  return (
    <div className="flex flex-col mr-6 mt-1.5 items-center gap-1 text-muted-foreground text-xl">
      <button
        type="submit"
        className="text-muted-foreground hover:text-foreground cursor-pointer"
      >
        <TriangleUp />
      </button>
      <span>{score}</span>
      <button
        type="submit"
        className="text-muted-foreground hover:text-foreground cursor-pointer"
      >
        <TriangleDown />
      </button>
    </div>
  );
}
