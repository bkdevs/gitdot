"use client";

import type { TaskResource } from "gitdot-api";
import type { S2Record } from "@/lib/s2";
import {
	Check,
	ChevronDown,
	ChevronRight,
	CircleSlash,
	Loader2,
	X,
} from "lucide-react";
import { useState } from "react";
import { cn } from "@/util";

function StatusIcon({ status }: { status: string }) {
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

export function BuildTask({ task, logs }: { task: TaskResource; logs: S2Record[] }) {
  const [open, setOpen] = useState(false);
  console.log(logs);

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
					<StatusIcon status={task.status} />
					<span>{task.name}: </span>
					<span className="text-muted-foreground">{task.command}</span>
				</div>
				{open ? (
					<ChevronDown className="size-3.5 text-muted-foreground" />
				) : (
					<ChevronRight className="size-3.5 text-muted-foreground" />
				)}
			</button>
			{open && (
				<div className="bg-background p-2 font-mono text-xs border-border border-b">
					{logs.map((log) => (
						<div key={log.seq_num} className="text-muted-foreground">
							{log.body}
						</div>
					))}
				</div>
			)}
		</div>
	);
}
