"use client";

import type { TaskResource } from "gitdot-api";
import { Check, ChevronDown, ChevronRight, Loader2, X } from "lucide-react";
import { useState } from "react";
import { cn } from "@/util";

const LOG_LINES = [
	"[00:00:01] Setting up environment...",
	"[00:00:02] Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	"[00:00:03] Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
	"[00:00:04] Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.",
	"[00:00:05] Duis aute irure dolor in reprehenderit in voluptate velit esse cillum.",
	"[00:00:06] Excepteur sint occaecat cupidatat non proident, sunt in culpa.",
	"[00:00:07] Qui officia deserunt mollit anim id est laborum.",
	"[00:00:08] Cloning repository...",
	"[00:00:09] Running pre-flight checks...",
	"[00:00:10] Installing dependencies...",
	"[00:00:11] Compiling source files...",
	"[00:00:12] Running tests...",
	"[00:00:13] All checks passed.",
	"[00:00:14] Done.",
];

function StatusIcon({ status }: { status: string }) {
	if (status === "running" || status === "assigned") {
		return <Loader2 className="size-3.5 animate-spin text-blue-400" />;
	}
	if (status === "success") {
		return <Check className="size-3.5 text-green-500" />;
	}
	if (status === "failure") {
		return <X className="size-3.5 text-red-500" />;
	}
	return (
		<span className="size-3.5 rounded-full bg-muted-foreground inline-block" />
	);
}

export function BuildTask({ task }: { task: TaskResource }) {
	const [open, setOpen] = useState(false);

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
				<div className="flex items-center gap-2">
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
				<div className="bg-background p-2 font-mono text-xs">
					{LOG_LINES.map((line) => (
						<div key={line} className="text-muted-foreground">
							{line}
						</div>
					))}
				</div>
			)}
		</div>
	);
}
