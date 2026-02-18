import { z } from "zod";

export const RunnerResponseSchema = z.object({
  id: z.uuid(),
  name: z.string(),
  owner_id: z.uuid(),
  owner_type: z.string(),
  created_at: z.string(),
});

export type RunnerResponse = z.infer<typeof RunnerResponseSchema>;

export const GetRunnerQuerySchema = z.object({
  owner_name: z.string(),
  owner_type: z.enum(["user", "organization"]),
});

export type GetRunnerQuery = z.infer<typeof GetRunnerQuerySchema>;
