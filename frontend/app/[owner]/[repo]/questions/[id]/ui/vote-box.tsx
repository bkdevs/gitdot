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

function TriangleUp() {
  return (
    <svg width="16.971" height="12" viewBox="0 0 16.971 12">
      <polygon points="8.485,0 0,12 16.971,12" fill="currentColor" />
    </svg>
  );
}

function TriangleDown() {
  return (
    <svg width="16.971" height="12" viewBox="0 0 16.971 12">
      <polygon points="0,0 16.971,0 8.485,12" fill="currentColor" />
    </svg>
  );
}
