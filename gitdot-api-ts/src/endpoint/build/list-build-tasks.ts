import { z } from "zod";
import { TaskResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListBuildTasksRequest = z.object({});
export type ListBuildTasksRequest = z.infer<typeof ListBuildTasksRequest>;

export const ListBuildTasksResponse = z.array(TaskResource);
export type ListBuildTasksResponse = z.infer<typeof ListBuildTasksResponse>;

export const ListBuildTasks = {
  path: "/repository/{owner}/{repo}/build/{number}/tasks",
  method: "GET",
  request: ListBuildTasksRequest,
  response: ListBuildTasksResponse,
} satisfies Endpoint;
export type ListBuildTasks = typeof ListBuildTasks;
