import { z } from "zod";

import { TaskResource } from "../../resource";

export const ListBuildTasksRequest = z.object({});
export type ListBuildTasksRequest = z.infer<typeof ListBuildTasksRequest>;

export const ListBuildTasks = {
  path: "/repository/{owner}/{repo}/build/{number}/tasks",
  method: "GET",
  request: ListBuildTasksRequest,
  response: z.array(TaskResource),
} as const;
export type ListBuildTasks = typeof ListBuildTasks;
