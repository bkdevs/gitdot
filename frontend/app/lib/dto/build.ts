import { z } from "zod";

export const CreateBuildRequestSchema = z.object({
  repo_owner: z.string(),
  repo_name: z.string(),
  trigger: z.enum(["pull_request", "push_to_main"]),
  commit_sha: z.string(),
});

export type CreateBuildRequest = z.infer<typeof CreateBuildRequestSchema>;

export const BuildResponseSchema = z.object({
  id: z.uuid(),
  repo_owner: z.string(),
  repo_name: z.string(),
  trigger: z.string(),
  commit_sha: z.string(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});

export type BuildResponse = z.infer<typeof BuildResponseSchema>;

export const BuildsResponseSchema = z.array(BuildResponseSchema);

export type BuildsResponse = z.infer<typeof BuildsResponseSchema>;
