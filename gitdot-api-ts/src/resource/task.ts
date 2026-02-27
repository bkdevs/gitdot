import { z } from "zod";

export const TaskStatus = z.enum([
  "blocked",
  "pending",
  "assigned",
  "running",
  "success",
  "failure",
]);
export type TaskStatus = z.infer<typeof TaskStatus>;

export const TaskResource = z.object({
  id: z.uuid(),
  repository_id: z.uuid(),
  build_id: z.uuid(),
  s2_uri: z.string(),
  name: z.string(),
  command: z.string(),
  status: TaskStatus,
  waits_for: z.array(z.uuid()),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});
export type TaskResource = z.infer<typeof TaskResource>;

export const PollTaskResource = z.object({
  id: z.uuid(),
  repository_id: z.uuid(),
  s2_uri: z.string(),
  name: z.string(),
  command: z.string(),
  status: TaskStatus,
});
export type PollTaskResource = z.infer<typeof PollTaskResource>;
