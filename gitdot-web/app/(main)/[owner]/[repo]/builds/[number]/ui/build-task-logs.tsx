"use client";

import { useEffect, useRef, useState } from "react";
import { tailTaskLogs } from "@/lib/s2/client";
import type { S2Record } from "@/lib/s2/shared";

interface BuildTaskLogsProps {
	owner: string;
	repo: string;
	taskId: string;
	initialLogs: S2Record[];
	running: boolean;
}

export function BuildTaskLogs({
	owner,
	repo,
	taskId,
	initialLogs,
	running,
}: BuildTaskLogsProps) {
	const [logs, setLogs] = useState<S2Record[]>(initialLogs);
	const bottomRef = useRef<HTMLDivElement>(null);

	useEffect(() => {
		if (!running) return;
		const controller = tailTaskLogs(owner, repo, taskId, (batch) => {
			setLogs((prev) => {
				const maxSeq = prev.length > 0 ? prev[prev.length - 1].seq_num : -1;
				const newRecords = batch.filter((r) => r.seq_num > maxSeq);
				return newRecords.length === 0 ? prev : [...prev, ...newRecords];
			});
		});
		return () => controller.abort();
	}, [owner, repo, taskId, running]);

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
