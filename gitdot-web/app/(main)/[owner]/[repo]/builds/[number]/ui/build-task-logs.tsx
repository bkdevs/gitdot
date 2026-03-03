"use client";

import type { TaskLogResource } from "gitdot-api";
import { useEffect, useRef } from "react";

interface BuildTaskLogsProps {
  logs: TaskLogResource[];
  running: boolean;
}

export function BuildTaskLogs({ logs, running }: BuildTaskLogsProps) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (running && logs.length > 0) {
      bottomRef.current?.scrollIntoView({ behavior: "smooth" });
    }
  }, [logs, running]);

  return (
    <div className="h-64 overflow-y-scroll bg-background p-2 font-mono text-xs border-border border-b [&::-webkit-scrollbar]:hidden">
      {logs.map((log) => (
        <div key={log.seq_num} className="text-muted-foreground">
          {log.body}
        </div>
      ))}
      <div ref={bottomRef} />
    </div>
  );
}
