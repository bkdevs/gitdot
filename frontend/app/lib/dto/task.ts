import { z } from "zod";

export const CreateTaskRequestSchema = z.object({
  repo_owner: z.string(),
  repo_name: z.string(),
  command: z.string(),
});

export type CreateTaskRequest = z.infer<typeof CreateTaskRequestSchema>;

export const TaskResponseSchema = z.object({
  id: z.uuid(),
  repo_owner: z.string(),
  repo_name: z.string(),
  build_id: z.uuid().optional(),
  name: z.string().optional(),
  command: z.string(),
  status: z.string(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});

export type TaskResponse = z.infer<typeof TaskResponseSchema>;

export const TasksResponseSchema = z.array(TaskResponseSchema);

export type TasksResponse = z.infer<typeof TasksResponseSchema>;
