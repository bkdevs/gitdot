import { z } from "zod";
import { RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateRepositoryRequest = z.object({
  description: z.string().optional(),
  readonly: z.boolean().optional(),
});
export type UpdateRepositoryRequest = z.infer<typeof UpdateRepositoryRequest>;

export const UpdateRepositoryResponse = RepositoryResource;
export type UpdateRepositoryResponse = z.infer<typeof UpdateRepositoryResponse>;

export const UpdateRepository = {
  path: "/repository/{owner}/{repo}",
  method: "PATCH",
  request: UpdateRepositoryRequest,
  response: UpdateRepositoryResponse,
} as const satisfies Endpoint;
export type UpdateRepository = typeof UpdateRepository;
