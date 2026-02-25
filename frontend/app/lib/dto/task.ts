import { z } from "zod";

export const TaskResponseSchema = z.object({
  id: z.uuid(),
  repo_owner: z.string(),
  repo_name: z.string(),
  build_id: z.uuid(),
  s2_uri: z.string(),
  name: z.string(),
  command: z.string(),
  status: z.string(),
  waits_for: z.array(z.uuid()),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});

export type TaskResponse = z.infer<typeof TaskResponseSchema>;

export const TasksResponseSchema = z.array(TaskResponseSchema);

export type TasksResponse = z.infer<typeof TasksResponseSchema>;
