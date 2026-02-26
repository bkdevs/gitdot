import { z } from "zod";
import { BuildResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateBuildRequest = z.object({
  trigger: z.string(),
  commit_sha: z.string(),
});
export type CreateBuildRequest = z.infer<typeof CreateBuildRequest>;

export const CreateBuildResponse = BuildResource;
export type CreateBuildResponse = z.infer<typeof CreateBuildResponse>;

export const CreateBuild = {
  path: "/repository/{owner}/{repo}/build",
  method: "POST",
  request: CreateBuildRequest,
  response: CreateBuildResponse,
} satisfies Endpoint;
export type CreateBuild = typeof CreateBuild;
