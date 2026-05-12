import { GitCommit, Star, Users } from "lucide-react";

export function RepoStats() {
  return (
    <div className="flex h-15 border-b">
      <StatCell icon={<Star className="size-3" />} value={142} label="stars" />
      <StatCell
        icon={<GitCommit className="size-3" />}
        value={387}
        label="commits"
      />
      <StatCell
        icon={<Users className="size-3" />}
        value={12}
        label="contributors"
      />
    </div>
  );
}

function StatCell({
  icon,
  value,
  label,
}: {
  icon: React.ReactNode;
  value: number;
  label: string;
}) {
  return (
    <div className="flex-1 flex flex-col items-center justify-center border-r last:border-r-0 min-w-0">
      <span className="text-sm font-medium font-mono">{value}</span>
      <span className="flex items-center gap-1 text-[10px] text-muted-foreground font-mono">
        {icon}
        {label}
      </span>
    </div>
  );
}
