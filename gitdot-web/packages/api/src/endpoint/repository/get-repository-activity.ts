import { z } from "zod";
import { RepositoryActivityEventResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryActivityRequest = z.object({});
export type GetRepositoryActivityRequest = z.infer<
  typeof GetRepositoryActivityRequest
>;

export const GetRepositoryActivityResponse = z.array(
  RepositoryActivityEventResource,
);
export type GetRepositoryActivityResponse = z.infer<
  typeof GetRepositoryActivityResponse
>;

export const GetRepositoryActivity = {
  path: "/repository/{owner}/{repo}/activity",
  method: "GET",
  request: GetRepositoryActivityRequest,
  response: GetRepositoryActivityResponse,
} as const satisfies Endpoint;
export type GetRepositoryActivity = typeof GetRepositoryActivity;
