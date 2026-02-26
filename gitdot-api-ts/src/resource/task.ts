import { z } from "zod";

export const TaskResource = z.object({
  id: z.string().uuid(),
  repository_id: z.string().uuid(),
  build_id: z.string().uuid(),
  s2_uri: z.string(),
  name: z.string(),
  command: z.string(),
  status: z.string(),
  waits_for: z.array(z.string().uuid()),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
});
export type TaskResource = z.infer<typeof TaskResource>;

export const PollTaskResource = z.object({
  id: z.string().uuid(),
  repository_id: z.string().uuid(),
  s2_uri: z.string(),
  name: z.string(),
  command: z.string(),
  status: z.string(),
});
export type PollTaskResource = z.infer<typeof PollTaskResource>;
