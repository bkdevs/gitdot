import { z } from "zod";

export const RunnerResponseSchema = z.object({
  id: z.uuid(),
  name: z.string(),
  owner_id: z.uuid(),
  owner_type: z.string(),
  last_verified: z.string().optional().nullable(),
  created_at: z.string(),
});

export type RunnerResponse = z.infer<typeof RunnerResponseSchema>;

export const RunnerTokenResponseSchema = z.object({
  token: z.string(),
});

export type RunnerTokenResponse = z.infer<typeof RunnerTokenResponseSchema>;
