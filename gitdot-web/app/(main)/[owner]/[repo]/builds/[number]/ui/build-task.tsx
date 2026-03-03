"use client";

import type { TaskLogResource, TaskResource, TaskStatus } from "gitdot-api";
import {
  Check,
  ChevronDown,
  ChevronRight,
  CircleSlash,
  Loader2,
  X,
} from "lucide-react";
import { useEffect, useState } from "react";
import { cn } from "@/util";
import { BuildTaskLogs } from "./build-task-logs";

export function BuildTask({
  task,
  logs: initialLogs,
}: {
  task: TaskResource;
  logs: TaskLogResource[];
}) {
  const [status, setStatus] = useState<TaskStatus>(task.status);
  const [logs, setLogs] = useState<TaskLogResource[]>(initialLogs);

  const running = status === "running" || status === "assigned";
  const [open, setOpen] = useState(running);

  useEffect(() => {
    if (!running) return;

    const controller = new AbortController();

    (async () => {
      try {
        const response = await fetch(`/actions/tasks/${task.id}/logs`, {
          headers: { Accept: "text/event-stream" },
          signal: controller.signal,
        });

        if (!response.ok || !response.body) {
          console.error(
            `Log stream failed: ${response.status} ${response.statusText}`,
          );
          return;
        }

        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        let buffer = "";

        while (true) {
          const { done, value } = await reader.read();
          if (done) break;

          buffer += decoder.decode(value, { stream: true });

          const events = buffer.split("\n\n");
          buffer = events.pop() ?? "";

          for (const event of events) {
            if (!event.trim()) continue;

            let eventType = "message";
            let data = "";

            for (const line of event.split("\n")) {
              if (line.startsWith("event:")) {
                eventType = line.slice("event:".length).trim();
              } else if (line.startsWith("data:")) {
                data = line.slice("data:".length).trim();
              }
            }

            if (eventType === "batch") {
              try {
                const batch = JSON.parse(data) as TaskLogResource[];
                const sentinel = batch.find((r) => r.finished != null);
                if (sentinel) {
                  setStatus(sentinel.finished as TaskStatus);
                }
                const normalRecords = batch.filter((r) => r.finished == null);
                setLogs((prev) => {
                  const maxSeq =
                    prev.length > 0 ? prev[prev.length - 1].seq_num : -1;
                  const newRecords = normalRecords.filter(
                    (r) => r.seq_num > maxSeq,
                  );
                  return newRecords.length === 0
                    ? prev
                    : [...prev, ...newRecords];
                });
              } catch (err) {
                console.error("Failed to parse log batch:", err);
              }
            } else if (eventType === "error") {
              console.error("Log stream error event:", data);
            }
          }
        }
      } catch (err) {
        if (err instanceof Error && err.name === "AbortError") return;
        console.error("Log stream error:", err);
      }
    })();

    return () => controller.abort();
  }, [task.id, running]);

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
