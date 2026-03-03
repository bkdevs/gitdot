import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const DeleteRepositoryRequest = z.object({});
export type DeleteRepositoryRequest = z.infer<typeof DeleteRepositoryRequest>;

export const DeleteRepositoryResponse = z.void();
export type DeleteRepositoryResponse = z.infer<typeof DeleteRepositoryResponse>;

export const DeleteRepository = {
  path: "/repository/{owner}/{repo}",
  method: "DELETE",
  request: DeleteRepositoryRequest,
  response: DeleteRepositoryResponse,
} as const satisfies Endpoint;
export type DeleteRepository = typeof DeleteRepository;
