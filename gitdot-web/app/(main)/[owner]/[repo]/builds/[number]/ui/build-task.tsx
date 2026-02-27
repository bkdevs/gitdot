"use client";

import type { TaskResource, TaskStatus } from "gitdot-api";
import {
  Check,
  ChevronDown,
  ChevronRight,
  CircleSlash,
  Loader2,
  X,
} from "lucide-react";
import { useEffect, useState } from "react";
import { tailTaskLogs } from "@/lib/s2/client";
import type { S2Record } from "@/lib/s2/shared";
import { cn } from "@/util";
import { BuildTaskLogs } from "./build-task-logs";

export function BuildTask({
  task,
  logs: initialLogs,
  owner,
  repo,
}: {
  task: TaskResource;
  logs: S2Record[];
  owner: string;
  repo: string;
}) {
  const [status, setStatus] = useState<TaskStatus>(task.status);
  const [logs, setLogs] = useState<S2Record[]>(initialLogs);

  const running = status === "running" || status === "assigned";
  const [open, setOpen] = useState(running);

  useEffect(() => {
    if (!running) return;

    const controller = tailTaskLogs(owner, repo, task.id, (batch) => {
      const sentinel = batch.find((r) =>
        r.headers.some(([k]) => k === "task-finished"),
      );
      if (sentinel) {
        const header = sentinel.headers.find(([k]) => k === "task-finished");
        if (header) setStatus(header[1] as TaskStatus);
      }

      const normalRecords = batch.filter((r) =>
        r.headers.every(([k]) => k !== "task-finished"),
      );

      setLogs((prev) => {
        const maxSeq = prev.length > 0 ? prev[prev.length - 1].seq_num : -1;
        const newRecords = normalRecords.filter((r) => r.seq_num > maxSeq);
        return newRecords.length === 0 ? prev : [...prev, ...newRecords];
      });
    });
    return () => controller.abort();
  }, [owner, repo, task.id, running]);

  return (
    <div className="flex flex-col">
      <button
        type="button"
        onClick={() => setOpen((o) => !o)}
        className={cn(
          "sticky top-0 z-10 flex h-9 w-full items-center justify-between border-b px-3 font-mono text-xs",
          open ? "bg-sidebar" : "bg-sidebar-primary",
        )}
      >
        <div className="flex items-center gap-1.5">
          <StatusIcon status={status} />
          <span>{task.name}: </span>
          <span className="text-muted-foreground">{task.command}</span>
        </div>
        {open ? (
          <ChevronDown className="size-3.5 text-muted-foreground" />
        ) : (
          <ChevronRight className="size-3.5 text-muted-foreground" />
        )}
      </button>
      {open && <BuildTaskLogs logs={logs} running={running} />}
    </div>
  );
}

function StatusIcon({ status }: { status: TaskStatus }) {
  if (status === "running" || status === "assigned") {
    return <Loader2 className="size-3 animate-spin text-muted-foreground" />;
  }
  if (status === "success") {
    return <Check className="size-3 text-green-600" />;
  }
  if (status === "failure") {
    return <X className="size-3 text-red-600" />;
  }
  return <CircleSlash className="size-3 text-muted-foreground" />;
}
