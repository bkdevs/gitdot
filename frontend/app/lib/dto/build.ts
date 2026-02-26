import { z } from "zod";
import { TasksResponseSchema } from "./task";

export const CreateBuildRequestSchema = z.object({
  trigger: z.enum(["pull_request", "push_to_main"]),
  commit_sha: z.string(),
});

export type CreateBuildRequest = z.infer<typeof CreateBuildRequestSchema>;

export const BuildResponseSchema = z.object({
  id: z.uuid(),
  number: z.number().int(),
  repository_id: z.uuid(),
  trigger: z.string(),
  commit_sha: z.string(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});

export type BuildResponse = z.infer<typeof BuildResponseSchema>;

export const BuildsResponseSchema = z.array(BuildResponseSchema);

export type BuildsResponse = z.infer<typeof BuildsResponseSchema>;

export const GetBuildResponseSchema = z.object({
  build: BuildResponseSchema,
  tasks: TasksResponseSchema,
});

export type GetBuildResponse = z.infer<typeof GetBuildResponseSchema>;
