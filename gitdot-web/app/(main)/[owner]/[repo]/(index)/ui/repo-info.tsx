"use client";

import type { RepositoryResource } from "gitdot-api";
import { useTimezone } from "@/(main)/context/timezone";
import { formatDate } from "@/util/date";

export function RepoInfo({ repository }: { repository: RepositoryResource }) {
  const tz = useTimezone();
  const rows: { label: string; value: string }[] = [
    { label: "visibility", value: repository.visibility },
    {
      label: "created",
      value: formatDate(new Date(repository.created_at), tz),
    },
  ];

  return (
    <div className="flex flex-col p-2 border-b">
      <span className="flex items-center gap-1.5 text-xs text-muted-foreground font-mono mb-1">
        About
      </span>
      <p className="text-xs text-foreground mb-2">
        {repository.description ??
          "This repository does not yet have a description. Descriptions help others quickly understand what the project does and why it exists. Consider adding one in the repository settings to give visitors a concise overview before they dive into the code."}
      </p>
      <div className="flex flex-col gap-1 font-mono text-xs">
        {rows.map((row) => (
          <div key={row.label} className="flex justify-between">
            <span className="text-muted-foreground">{row.label}</span>
            <span className="text-foreground">{row.value}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
