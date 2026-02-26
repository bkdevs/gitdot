import { z } from "zod";

export const RunnerResource = z.object({
  id: z.string().uuid(),
  name: z.string(),
  owner_id: z.string().uuid(),
  owner_name: z.string(),
  owner_type: z.string(),
  last_verified: z.string().datetime().nullable(),
  created_at: z.string().datetime(),
});
export type RunnerResource = z.infer<typeof RunnerResource>;

export const RunnerTokenResource = z.object({
  token: z.string(),
});
export type RunnerTokenResource = z.infer<typeof RunnerTokenResource>;
