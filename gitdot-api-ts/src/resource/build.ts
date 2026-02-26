import { z } from "zod";

import { TaskResource } from "./task";

export const BuildResource = z.object({
  id: z.string().uuid(),
  number: z.number().int(),
  repository_id: z.string().uuid(),
  trigger: z.string(),
  commit_sha: z.string(),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
});
export type BuildResource = z.infer<typeof BuildResource>;

export const GetBuildResource = z.object({
  build: BuildResource,
  tasks: z.array(TaskResource),
});
export type GetBuildResource = z.infer<typeof GetBuildResource>;
