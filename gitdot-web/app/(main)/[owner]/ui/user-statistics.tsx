export function UserStatistics() {
  return (
    <div className="grid grid-cols-2 gap-8">
      <div>
        <p className="text-xs text-muted-foreground font-mono mb-1">
          <span className="text-foreground/40 select-none"># </span>Streak
        </p>
        <div className="grid grid-cols-[auto_auto] gap-x-2 font-mono text-xs">
          <span className="text-muted-foreground">current</span>
          <span>12d</span>
          <span className="text-muted-foreground">longest</span>
          <span>31d</span>
          <span className="text-muted-foreground">active days</span>
          <span>214 / 365</span>
        </div>
      </div>
      <div>
        <p className="text-xs text-muted-foreground font-mono mb-1">
          <span className="text-foreground/40 select-none"># </span>Statistics
        </p>
        <div className="grid grid-cols-[auto_auto] gap-x-2 font-mono text-xs">
          <span className="text-muted-foreground">loc</span>
          <span>184,209</span>
          <span className="text-muted-foreground">commits</span>
          <span>2,341</span>
          <span className="text-muted-foreground">issues</span>
          <span>847</span>
        </div>
      </div>
    </div>
  );
}
