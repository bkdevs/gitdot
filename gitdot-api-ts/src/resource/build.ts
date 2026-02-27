import { z } from "zod";

import { TaskResource } from "./task";

export const BuildResource = z.object({
  id: z.uuid(),
  number: z.number().int(),
  repository_id: z.uuid(),
  trigger: z.enum(["pull_request", "push_to_main"]),
  commit_sha: z.string(),
  status: z.enum(["running", "success", "failure"]),
  total_tasks: z.number().int(),
  completed_tasks: z.number().int(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});
export type BuildResource = z.infer<typeof BuildResource>;

export const GetBuildResource = z.object({
  build: BuildResource,
  tasks: z.array(TaskResource),
});
export type GetBuildResource = z.infer<typeof GetBuildResource>;
