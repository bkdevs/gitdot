import { z } from "zod";

export const RunnerResponseSchema = z.object({
  id: z.uuid(),
  name: z.string(),
  owner_id: z.uuid(),
  owner_type: z.string(),
  created_at: z.string(),
});

export type RunnerResponse = z.infer<typeof RunnerResponseSchema>;
