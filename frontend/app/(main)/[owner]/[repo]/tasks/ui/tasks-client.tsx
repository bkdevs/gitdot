"use client";

import type { TasksResponse } from "@/lib/dto";
import { TaskRow } from "./task-row";
import { TasksHeader } from "./tasks-header";

export function TasksClient({
  owner,
  repo,
  tasks,
}: {
  owner: string;
  repo: string;
  tasks: TasksResponse;
}) {
  return (
    <div className="flex flex-col">
      <TasksHeader owner={owner} repo={repo} />
      {tasks.map((task) => (
        <TaskRow key={task.id} task={task} />
      ))}
    </div>
  );
}
